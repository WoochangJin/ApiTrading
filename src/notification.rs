use serde::Serialize;
use crate::broker::client::BrokerClient;

#[derive(Serialize)]
struct DiscordMessage<'a> {
    content: &'a str,
}

pub async fn send_discord(client: &BrokerClient, message: &str) {
    let body = DiscordMessage { content: message };
    let header = reqwest::header::HeaderMap::new();
    if let Err(e) = client.post_empty("", header, &body).await {
        tracing::warn!("failed to send discord notification: {e:?}");
    }
}