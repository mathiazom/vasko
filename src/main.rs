use crate::consts::CATS;
use crate::schedule::schedule_task;
use crate::utils::weekday_to_chrono;
use chrono::{Datelike, Local};
use clokwerk::{Interval, Job};
use config::Config;
use slack_morphism::prelude::*;
use std::fs;
use std::sync::Arc;
use tokio::task::JoinSet;

mod config;
mod consts;
mod utils;
mod slack;
mod schedule;

#[tokio::main]
async fn main() {
    let config_str = fs::read_to_string("config.kdl").expect("Failed to read config file");
    let config = knuffel::parse::<Config>("config.kdl", &config_str).unwrap();
    let current_week_number = Local::now().iso_week().week();
    dbg!(current_week_number);
    let mut join_set = JoinSet::new();
    for week in config.schedule {
        let week_number: u32 = week.number as u32;
        dbg!(Interval::Weeks(week_number - current_week_number));
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
            join_set.spawn(schedule_task(week_number, weekday_to_chrono(&reminder.weekday).clone(), reminder.hour.into(), reminder.minute.into(), move || {
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
    while let Some(res) = join_set.join_next().await {
        res.unwrap();
    }
}
