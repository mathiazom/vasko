use crate::config::ReadConfigError::{IoError, ParseError};
use knuffel::{Decode, DecodeScalar, Error};
use std::fs;

#[derive(Decode, Debug)]
pub struct Config {
    #[knuffel(child, unwrap(children))]
    pub workers: Vec<Worker>,
    #[knuffel(child, unwrap(argument))]
    pub channel: String,
    #[knuffel(child, unwrap(argument))]
    pub bot: String,
    #[knuffel(child, unwrap(children))]
    pub reminders: Vec<Reminder>,
    #[knuffel(child, unwrap(children))]
    pub tasks: Vec<Task>,
    #[knuffel(child, unwrap(children))]
    pub schedule: Vec<Week>,
}

#[derive(Decode, Debug)]
pub struct Worker {
    #[knuffel(argument)]
    pub name: String,
    #[knuffel(argument)]
    pub slack_id: String,
}

#[derive(DecodeScalar, Clone, Debug)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Decode, Clone, Debug)]
pub struct RedditImage {
    #[knuffel(child, unwrap(argument))]
    pub sub: String,
    #[knuffel(child, unwrap(argument))]
    pub pretext: Option<String>,
}

#[derive(Decode, Clone, Debug)]
pub enum ReminderImage {
    Reddit(RedditImage),
}

#[derive(Decode, Clone, Debug)]
pub struct ReminderMessage {
    #[knuffel(child, unwrap(argument))]
    pub singular: String,
    #[knuffel(child, unwrap(argument))]
    pub plural: String,
}

pub type Hour = u8;

pub type Minute = u8;

#[derive(Decode, Clone, Debug)]
pub struct Reminder {
    #[knuffel(child, unwrap(argument))]
    pub weekday: Weekday,
    #[knuffel(child, unwrap(argument))]
    pub hour: Hour,
    #[knuffel(child, unwrap(argument))]
    pub minute: Minute,
    #[knuffel(child)]
    pub message: ReminderMessage,
    #[knuffel(child, unwrap(children))]
    pub image: Vec<ReminderImage>,
}

#[derive(Decode, Debug)]
pub struct Task {
    #[knuffel(argument)]
    pub text: String,
}

pub type WeekNumber = u8;

#[derive(Decode, Debug)]
pub struct Week {
    #[knuffel(argument)]
    pub number: WeekNumber,
    #[knuffel(arguments)]
    pub workers: Vec<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum ReadConfigError {
    #[error(transparent)]
    IoError(std::io::Error),
    #[error(transparent)]
    ParseError(Error),
}

pub fn read_config(path: &str) -> Result<Config, ReadConfigError> {
    let config_str = fs::read_to_string(path).map_err(IoError)?;
    knuffel::parse::<Config>(path, &config_str).map_err(ParseError)
}
