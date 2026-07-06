use crate::config::RedditCredentials;
use crate::reddit::RedditError::{FetchError, LoginError, MissingData};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use roux::util::{FeedOption, RouxError, TimePeriod};
use roux::Subreddit;
use serde::Deserialize;
use std::io::{Error, ErrorKind};
use url::Url;

#[derive(thiserror::Error, Debug)]
pub enum RedditError {
    #[error("failed to log in to reddit: {0}")]
    LoginError(String),
    #[error(transparent)]
    FetchError(RouxError),
    #[error(transparent)]
    MissingData(Error),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum TokenResponse {
    Ok { access_token: String },
    Err { error: String },
}

const THIS_WEEK_FEED_OPTION: FeedOption = FeedOption {
    after: None,
    before: None,
    limit: None,
    count: None,
    period: Some(TimePeriod::ThisWeek),
};

const TOP_IMAGES_SEARCH_LIMIT: u32 = 20;

async fn login(user_agent: &str, credentials: &RedditCredentials) -> Result<String, RedditError> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://www.reddit.com/api/v1/access_token")
        .header(USER_AGENT, HeaderValue::from_str(user_agent).unwrap())
        .basic_auth(&credentials.client_id, Some(&credentials.client_secret))
        .form(&[
            ("grant_type", "password"),
            ("username", credentials.username.as_str()),
            ("password", credentials.password.as_str()),
        ])
        .send()
        .await
        .map_err(|e| LoginError(e.to_string()))?;

    let status = response.status();
    let body = response
        .text()
        .await
        .map_err(|e| LoginError(e.to_string()))?;

    if !status.is_success() {
        return Err(LoginError(format!("HTTP {}", status)));
    }

    match serde_json::from_str::<TokenResponse>(&body) {
        Ok(TokenResponse::Ok { access_token }) => Ok(access_token),
        Ok(TokenResponse::Err { error }) => Err(LoginError(format!("auth error: {}", error))),
        Err(e) => Err(LoginError(format!("failed to decode token response: {}", e))),
    }
}

pub async fn top_image_this_week(
    subreddit: &String,
    credentials: &RedditCredentials,
) -> Result<Url, RedditError> {
    let user_agent = format!(
        "linux:vasko-chores-bot:v1.0 (by /u/{})",
        credentials.username
    );
    let access_token = login(&user_agent, credentials).await?;

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
    );
    headers.insert(USER_AGENT, HeaderValue::from_str(&user_agent).unwrap());
    let authed_client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| LoginError(e.to_string()))?;

    let authed_subreddit = Subreddit::new_oauth(subreddit, &authed_client);

    let res = authed_subreddit
        .top(TOP_IMAGES_SEARCH_LIMIT, Some(THIS_WEEK_FEED_OPTION))
        .await
        .map_err(FetchError)?;
    println!(
        "🔎 r/{} top-this-week response: {} posts, hints={:?}",
        subreddit,
        res.data.children.len(),
        res.data
            .children
            .iter()
            .map(|post| post.data.post_hint.as_deref().unwrap_or("<none>"))
            .collect::<Vec<_>>()
    );
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
