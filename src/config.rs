use knuffel::{Decode, DecodeScalar};
#[derive(Decode, Debug)]
pub struct Config {
    #[knuffel(child, unwrap(children))]
    pub workers: Vec<Worker>,
    #[knuffel(child, unwrap(argument))]
    pub channel: String,
    #[knuffel(child, unwrap(argument))]
    pub bot: String,
    #[knuffel(child)]
    pub media: Option<Media>,
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

#[derive(Decode, Debug)]
pub struct RedditImageSource {
    #[knuffel(child, unwrap(argument))]
    pub id: String,
    #[knuffel(child, unwrap(argument))]
    pub secret: String,
}

#[derive(Decode, Debug)]
pub struct Media {
    #[knuffel(child)]
    pub sources: MediaSources,
    #[knuffel(child, unwrap(argument))]
    pub transfersh: String,
}

#[derive(Decode, Debug)]
pub struct MediaSources {
    #[knuffel(child)]
    pub reddit: Option<RedditImageSource>,
}

#[derive(DecodeScalar, Debug)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Decode, Debug)]
pub struct RedditImage {
    #[knuffel(arguments)]
    pub subs: Vec<String>,
}

#[derive(Decode, Debug)]
pub struct ReminderMedia {
    #[knuffel(child)]
    pub reddit: Option<RedditImage>,
}

#[derive(Decode, Debug)]
pub struct ReminderMessage {
    #[knuffel(child, unwrap(argument))]
    pub singular: String,
    #[knuffel(child, unwrap(argument))]
    pub plural: String,
}

pub type Hour = u8;

pub type Minute = u8;

#[derive(Decode, Debug)]
pub struct Reminder {
    #[knuffel(child, unwrap(argument))]
    pub weekday: Weekday,
    #[knuffel(child, unwrap(argument))]
    pub hour: Hour,
    #[knuffel(child, unwrap(argument))]
    pub minute: Minute,
    #[knuffel(child)]
    pub message: ReminderMessage,
    #[knuffel(child)]
    pub media: ReminderMedia,
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