use slack_morphism::{SlackApiToken, SlackChannelId, SlackClient, SlackMessageContent};
use slack_morphism::hyper_tokio::SlackClientHyperConnector;
use slack_morphism::api::SlackApiChatPostMessageRequest;
use slack_morphism::blocks::SlackBlock::{Image, Section};
use slack_morphism::blocks::{SlackBlockMarkDownText, SlackImageBlock, SlackSectionBlock};
use slack_morphism::blocks::SlackBlockText::MarkDown;

pub async fn send_reminder(
    bot_token: String,
    channel_id: SlackChannelId,
    message: String,
    thread_messages: Vec<String>,
    image_url: String,
) {
    let client = SlackClient::new(SlackClientHyperConnector::new().unwrap());
    let token: SlackApiToken = SlackApiToken::new(bot_token.into());
    let session = client.open_session(&token);
    let post_chat_req = SlackApiChatPostMessageRequest::new(
        channel_id.clone(),
        SlackMessageContent::new().with_blocks(vec![Section(SlackSectionBlock {
            block_id: None,
            text: Some(MarkDown(SlackBlockMarkDownText {
                text: message.clone(),
                verbatim: None,
            })),
            fields: None,
            accessory: None,
        }), Image(SlackImageBlock {
            block_id: None,
            image_url: image_url.parse().unwrap(),
            alt_text: "".to_string(),
            title: None,
        })]),
    );
    let res = dbg!(session.chat_post_message(&post_chat_req).await);
    let reminder_ts = res.unwrap().ts;
    for thread_message in thread_messages {
        let post_thread_req =
            SlackApiChatPostMessageRequest {
                channel: channel_id.clone(),
                content: SlackMessageContent::new().with_text(thread_message.clone()),
                as_user: None,
                icon_emoji: None,
                icon_url: None,
                link_names: None,
                parse: None,
                thread_ts: Some(reminder_ts.clone()),
                username: None,
                reply_broadcast: None,
                unfurl_links: None,
                unfurl_media: None,
            };
        let thread_res = dbg!(session.chat_post_message(&post_thread_req).await);
    }
}