use chrono::{Datelike, Local, NaiveTime, Weekday};
use std::future::Future;
use tokio::time::{sleep, Duration};

pub async fn schedule_task<F, Fut>(
    week_number: u32,
    weekday: Weekday,
    hour: u32,
    minute: u32,
    task: F
) where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let now = Local::now();
    let target_time = NaiveTime::from_hms_opt(hour, minute, 0).unwrap();

    // Calculate the next occurrence of the specified week number
    let days_until_target_week = (week_number as i64 - now.iso_week().week() as i64) * 7;
    let mut target_date = now + chrono::Duration::days(days_until_target_week);

    // Adjust to the specified weekday within the target week
    while !target_date.weekday().eq(&weekday) {
        target_date += chrono::Duration::days(1);
    }

    // Combine date and time
    let mut target_datetime = target_date.with_time(target_time).unwrap();

    // If the target datetime is in the past, move to the next occurrence
    if target_datetime <= now {
        target_datetime += chrono::Duration::weeks(1);
    }

    // Calculate duration until the target time
    let duration_until_target = target_datetime.signed_duration_since(now);

    if duration_until_target.num_seconds() > 0 {

        dbg!(duration_until_target);

        // Sleep until the target time
        sleep(Duration::from_secs(duration_until_target.num_seconds() as u64)).await;

        // Execute the task
        task().await;
    }
}
