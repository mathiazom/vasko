use clokwerk::{AsyncScheduler, Interval, Job};
use config::Config;
use std::sync::Arc;
use std::time::Duration;
use std::fs;
use chrono::NaiveTime;
use slack_morphism::prelude::*;
use crate::consts::CATS;
use crate::utils::weekday_to_clokwerk_interval;

mod config;
mod consts;
mod utils;
mod slack;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config_str = fs::read_to_string("config.kdl").expect("Failed to read config file");
    let config = knuffel::parse::<Config>("config.kdl", &config_str).unwrap();
    let mut scheduler = AsyncScheduler::new();
    for week in config.schedule {
        let worker_ids = week.workers.clone();
        let slack_ids: Vec<String> = worker_ids.iter().map(|id| {
            format!("<@{}>", config.workers.iter().find(|w| w.name.eq(id)).unwrap().slack_id)
        }).collect();
        let workers_count = slack_ids.len();
        let human_slack_mentions = utils::join_human_readable(slack_ids.clone());
        let task_texts: Vec<String> = config.tasks.iter().map(|x| x.text.clone()).collect();
        for (i, reminder) in config.reminders.iter().enumerate() {
            let message_template = match workers_count {
                1 => reminder.message.singular.clone(),
                _ => reminder.message.plural.clone()
            };
            let message = message_template.replace("%s", human_slack_mentions.as_str());
            let bot_token = Arc::new(config.bot.clone());
            let channel = Arc::new(config.channel.clone());
            let message = Arc::new(message.clone());
            let thread_messages: Arc<Vec<String>> = Arc::new(task_texts.iter().enumerate().map(|(i, t)| format!("{}\n{}", slack_ids.get(i).unwrap(), t)).collect());
            dbg!(scheduler.every(weekday_to_clokwerk_interval(&reminder.weekday)).at_time(NaiveTime::from_hms_opt(reminder.hour.into(), reminder.minute.into(), 0).unwrap()).run(move || {
                slack::send_reminder(
                    bot_token.clone().to_string(),
                    channel.clone().to_string().into(),
                    message.clone().to_string(),
                    thread_messages.clone().to_vec(),
                    CATS[i % CATS.len()].to_string()
                )
            }));
        }
    }
    loop {
        scheduler.run_pending().await;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
