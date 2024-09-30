use crate::config::Weekday;
use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveTime, TimeZone};
use std::time::Duration;

pub fn join_human_readable(items: &Vec<String>) -> String {
    match items.len() {
        0 => String::new(),
        1 => items[0].clone(),
        _ => format!(
            "{} og {}",
            items[..items.len() - 1].join(", "),
            items[items.len() - 1]
        ),
    }
}

pub fn weekday_to_chrono(weekday: &Weekday) -> chrono::Weekday {
    match weekday {
        Weekday::Monday => chrono::Weekday::Mon,
        Weekday::Tuesday => chrono::Weekday::Tue,
        Weekday::Wednesday => chrono::Weekday::Wed,
        Weekday::Thursday => chrono::Weekday::Thu,
        Weekday::Friday => chrono::Weekday::Fri,
        Weekday::Saturday => chrono::Weekday::Sat,
        Weekday::Sunday => chrono::Weekday::Sun,
    }
}

fn from_local_isoywdhm_opt(
    year: i32,
    week: u32,
    weekday: chrono::Weekday,
    hour: u32,
    min: u32,
) -> Option<DateTime<Local>> {
    let target_date = NaiveDate::from_isoywd_opt(year, week, weekday)?;
    let target_time = NaiveTime::from_hms_opt(hour, min, 0)?;
    let naive_target_datetime = target_date.and_time(target_time);
    Local.from_local_datetime(&naive_target_datetime).single()
}

pub fn from_next_local_isowdhm_opt(
    week: u32,
    weekday: chrono::Weekday,
    hour: u32,
    min: u32,
) -> Option<DateTime<Local>> {
    let now = Local::now();
    let current_year = now.year();
    let mut datetime = from_local_isoywdhm_opt(current_year, week, weekday, hour, min)?;
    if week < now.iso_week().week() && datetime <= now {
        datetime = from_local_isoywdhm_opt(current_year + 1, week, weekday, hour, min)?;
    }
    Some(datetime)
}

pub fn duration_until_datetime(date_time: DateTime<Local>) -> Option<Duration> {
    let seconds_until = date_time.signed_duration_since(Local::now()).num_seconds();
    if seconds_until <= 0 {
        return None;
    }
    Some(Duration::from_secs(seconds_until as u64))
}
