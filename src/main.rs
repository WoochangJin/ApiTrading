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

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env();
    let state = AppState::new(&config)
        .await
        .expect("failed to initialize app state");

    scheduler::run(state).await;
}