use crate::config::ReminderImage;
use crate::reddit::top_image_this_week;
use crate::reminder::SendReminderError::{SlackError, SlackHyperError};
use chrono::Local;
use slack_morphism::api::SlackApiChatPostMessageRequest;
use slack_morphism::blocks::SlackBlock::{Image, Section};
use slack_morphism::blocks::SlackBlockText::{MarkDown, Plain};
use slack_morphism::blocks::{
    SlackBlockMarkDownText, SlackBlockPlainText, SlackImageBlock, SlackSectionBlock,
};
use slack_morphism::errors::SlackClientError;
use slack_morphism::hyper_tokio::SlackClientHyperConnector;
use slack_morphism::{
    SlackApiToken, SlackApiTokenValue, SlackChannelId, SlackClient, SlackMessageContent,
};
use std::io::Error;
use url::Url;

#[derive(thiserror::Error, Debug)]
pub enum SendReminderError {
    #[error(transparent)]
    SlackHyperError(Error),
    #[error(transparent)]
    SlackError(SlackClientError),
}

pub async fn send_reminder(
    bot_token: SlackApiTokenValue,
    channel_id: SlackChannelId,
    message: String,
    thread_messages: Vec<String>,
    image_url: Option<Url>,
    image_pretext: Option<String>,
) -> Result<(), SendReminderError> {
    let client = SlackClient::new(SlackClientHyperConnector::new().map_err(SlackHyperError)?);
    let token = SlackApiToken::new(bot_token);
    let session = client.open_session(&token);
    let mut blocks = vec![Section(SlackSectionBlock {
        block_id: None,
        text: Some(MarkDown(SlackBlockMarkDownText {
            text: message.clone(),
            verbatim: None,
        })),
        fields: None,
        accessory: None,
    })];
    if let Some(url) = image_url {
        if let Some(pretext) = image_pretext {
            blocks.push(Section(SlackSectionBlock {
                block_id: None,
                text: Some(Plain(SlackBlockPlainText {
                    text: pretext,
                    emoji: Some(true),
                })),
                fields: None,
                accessory: None,
            }))
        }
        blocks.push(Image(SlackImageBlock {
            block_id: None,
            image_url: url,
            alt_text: "".to_string(),
            title: None,
        }))
    }
    let res = session
        .chat_post_message(&SlackApiChatPostMessageRequest::new(
            channel_id.clone(),
            SlackMessageContent::new().with_blocks(blocks),
        ))
        .await
        .map_err(SlackError)?;
    println!("💬 Reminder posted successfully");
    for thread_message in thread_messages {
        let post_thread_req = SlackApiChatPostMessageRequest {
            channel: channel_id.clone(),
            content: SlackMessageContent::new().with_text(thread_message.clone()),
            as_user: None,
            icon_emoji: None,
            icon_url: None,
            link_names: None,
            parse: None,
            thread_ts: Some(res.ts.clone()),
            username: None,
            reply_broadcast: None,
            unfurl_links: None,
            unfurl_media: None,
        };
        session
            .chat_post_message(&post_thread_req)
            .await
            .map_err(SlackError)?;
    }
    println!("🧵 Thread messages posted successfully");
    Ok(())
}

pub async fn send_reminder_task(
    bot_token: String,
    channel: String,
    message: String,
    thread_messages: Vec<String>,
    images: Vec<ReminderImage>,
) {
    println!("⏰ Reminder task woke up at {}", Local::now());
    let mut image_pretext: Option<String> = None;
    let mut image_url: Option<Url> = None;
    for img in images {
        match img {
            ReminderImage::Reddit(rm) => {
                if let Ok(top_image_url) = top_image_this_week(&rm.sub).await {
                    (image_url, image_pretext) = (Some(top_image_url), rm.pretext);
                    break;
                }
            }
        }
    }
    send_reminder(
        bot_token.into(),
        channel.into(),
        message,
        thread_messages,
        image_url,
        image_pretext,
    )
    .await
    .unwrap_or_else(|e| {
        dbg!(e);
    })
}
