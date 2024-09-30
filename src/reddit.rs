use crate::reddit::RedditError::{FetchError, MissingData};
use roux::util::{FeedOption, RouxError, TimePeriod};
use roux::Subreddit;
use std::io::{Error, ErrorKind};
use url::Url;

#[derive(thiserror::Error, Debug)]
pub enum RedditError {
    #[error(transparent)]
    FetchError(RouxError),
    #[error(transparent)]
    MissingData(Error),
}

const THIS_WEEK_FEED_OPTION: FeedOption = FeedOption {
    after: None,
    before: None,
    limit: None,
    count: None,
    period: Some(TimePeriod::ThisWeek),
};

const TOP_IMAGES_SEARCH_LIMIT: u32 = 20;

pub async fn top_image_this_week(subreddit: &String) -> Result<Url, RedditError> {
    let res = Subreddit::new(subreddit)
        .top(TOP_IMAGES_SEARCH_LIMIT, Some(THIS_WEEK_FEED_OPTION))
        .await
        .map_err(FetchError)?;
    let url: Option<Url> = res.data.children.iter().find_map(|post| {
        match (post.data.post_hint.as_deref(), post.data.url.as_deref()) {
            (Some("image"), Some(raw_url)) => Url::parse(raw_url).ok(),
            _ => None,
        }
    });
    url.ok_or(MissingData(Error::new(
        ErrorKind::Other,
        format!(
            "Failed to find any top posts with a valid image url (searched top {})",
            TOP_IMAGES_SEARCH_LIMIT
        ),
    )))
}
