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
use broker::order::OrderRequest;

#[tokio::main]
async fn main() {
    let config = config::Config::from_env();
    let state = AppState::new(&config).await.expect("failed to initialize app state");

    let res = state.with_token(|token| {
        OrderRequest::loc_order(
            state.account_seq, token, &state.broker_client,
            "100", "BUY", "1000", None,   // 100주 * $1000 = $100,000 상당, 무조건 잔액부족 뜰 금액
        )
    }).await;

    println!("Order result: {:?}", res);
}