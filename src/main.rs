mod config;
mod state;
mod error;
mod broker;
mod dto;
mod handlers;
mod models;
mod notification;
mod repositories;
mod risk;
mod routes;
mod scheduler;
mod services;
mod strategy;
mod ws;

use broker::client::BrokerClient;
use broker::auth::AccessToken;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = config::Config::from_env();
    let client = BrokerClient::new("https://api.example.com".to_string());
    let token = AccessToken::issue(&client, &config.appkey, &config.secretkey).await.unwrap();
    println!("Access Token: {}", token.token);
}
