use crate::config::read_config;
use crate::schedule::schedule_task;
use crate::utils::{from_next_local_isowdhm_opt, join_human_readable, weekday_to_chrono};
use humantime::format_duration;
use tokio::task::JoinSet;
use utils::duration_until_datetime;

const CONFIG_PATH: &str = "config.kdl";

mod config;
mod reddit;
mod reminder;
mod schedule;
mod utils;

#[tokio::main]
async fn main() {
    let config = read_config(CONFIG_PATH)
        .expect(format!("Error reading config at {}", CONFIG_PATH).as_str());
    let mut tasks = JoinSet::new();
    let all_workers = config.workers;
    for week in config.schedule {
        let week_number = week.number as u32;
        let slack_ids = utils::slack_ids_from_week_workers(&week.workers, &all_workers);
        let workers_count = slack_ids.len();
        let human_slack_mentions = join_human_readable(&slack_ids);
        let task_texts: Vec<String> = config.tasks.iter().map(|x| x.text.clone()).collect();
        for reminder in config.reminders.clone() {
            let message = (match workers_count {
                1 => reminder.message.singular,
                _ => reminder.message.plural,
            })
            .replace("%s", human_slack_mentions.as_str());
            let thread_messages: Vec<String> = task_texts
                .iter()
                .enumerate()
                .filter_map(|(i, t)| slack_ids.get(i).map(|s| format!("{}\n{}", s, t)))
                .collect();
            let Some(target_datetime) = from_next_local_isowdhm_opt(
                week_number,
                weekday_to_chrono(&reminder.weekday),
                reminder.hour.into(),
                reminder.minute.into(),
            ) else {
                println!("Failed to determine target datetime for reminder schedule");
                continue;
            };
            if let Some(target_duration) = duration_until_datetime(target_datetime) {
                tasks.spawn(schedule_task(
                    target_duration,
                    reminder::send_reminder_task(
                        config.bot.clone(),
                        config.channel.clone(),
                        message,
                        thread_messages,
                        reminder.image,
                    ),
                ));
                println!(
                    "🗓️ Scheduled reminder for {} ({} from now)",
                    target_datetime,
                    format_duration(target_duration)
                );
            } else {
                println!("🔕 Skipped overdue reminder ({})", target_datetime);
            }
        }
    }
    println!(
        "{} task{} spawned",
        tasks.len(),
        if tasks.len() == 1 { "" } else { "s" }
    );
    while let Some(res) = tasks.join_next().await {
        res.unwrap();
    }
}
