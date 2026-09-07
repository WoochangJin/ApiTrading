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

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = config::Config::from_env();
    let price_client = BrokerClient::new("http://100.122.108.62:5000".to_string());
    let price_response = price::caller::PriceRequest::get_price(
        &price_client,
        "SOXL",
        1000,
        "2024-01-01",
    ).await;
    println!("Price Response: {:?}", price_response);
}
