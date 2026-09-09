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

use std::sync::Arc;

use state::AppState;

const BIND_ADDR: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env();
    let state = Arc::new(
        AppState::new(&config)
            .await
            .expect("failed to initialize app state"),
    );

    // 사이클이 실패했을 때 수동으로 다시 돌리기 위한 로컬 전용 HTTP 서버
    let listener = tokio::net::TcpListener::bind(BIND_ADDR)
        .await
        .expect("failed to bind http server");
    tracing::info!("http server listening on {BIND_ADDR}");
    let app = routes::router(state.clone());
    tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            tracing::error!("http server stopped: {e:?}");
        }
    });

    scheduler::run(state).await;
}