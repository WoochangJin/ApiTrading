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
mod price;

use broker::client::BrokerClient;
use broker::auth::AccessToken;
use broker::account::Accounts;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = config::Config::from_env();
    let broker_client = BrokerClient::new("https://openapi.tossinvest.com".to_string());
    let token_res= AccessToken::issue(
        &broker_client,
        &config.client_id,
        &config.client_secret,
    );

    let token = token_res.await.unwrap().access_token;
    let account = Accounts::get_accounts(
        &broker_client,
        &token,
    );

    println!("Account: {:?}", account.await.unwrap());
}
