use clokwerk::Interval;
use crate::config::Weekday;

pub fn join_human_readable(items: Vec<String>) -> String {
    match items.len() {
        0 => String::new(),
        1 => items[0].clone(),
        _ => format!("{} og {}", items[..items.len() - 1].join(", "), items[items.len() - 1]),
    }
}

pub fn weekday_to_clokwerk_interval(weekday: &Weekday) -> Interval {
    match weekday {
        Weekday::Monday => Interval::Monday,
        Weekday::Tuesday => Interval::Tuesday,
        Weekday::Wednesday => Interval::Wednesday,
        Weekday::Thursday => Interval::Thursday,
        Weekday::Friday => Interval::Friday,
        Weekday::Saturday => Interval::Saturday,
        Weekday::Sunday => Interval::Sunday,
    }
}