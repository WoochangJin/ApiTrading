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

use state::AppState;
use broker::account::Accounts;

#[tokio::main]
async fn main() {
    let config = config::Config::from_env();
    let state = AppState::new(&config).await.expect("failed to initialize app state");

    let accounts = state
        .with_token(|token| Accounts::get_accounts(&state.broker_client, token))
        .await
        .expect("failed to fetch accounts");

    println!("Accounts: {:?}", accounts);
}
