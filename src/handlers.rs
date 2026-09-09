use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::{IntoResponse, Response}};

use crate::notification;
use crate::services;
use crate::state::AppState;

pub async fn health() -> &'static str {
    "ok"
}

/// 스케줄러의 18:00 실행이 실패했을 때 데일리 사이클을 수동으로 다시 돌린다.
pub async fn run_cycle(State(state): State<Arc<AppState>>) -> Response {
    tracing::info!("manual daily cycle triggered");

    match services::run_daily_cycle(&state).await {
        Ok(()) => {
            notification::send_discord(&state.discord_client, "**수동 사이클 실행 완료**").await;
            (
                StatusCode::OK,
                Json(serde_json::json!({ "status": "ok" })),
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("manual daily cycle failed: {e:?}");
            notification::send_discord(
                &state.discord_client,
                &format!("**수동 사이클 실행 실패**\n{e}"),
            )
            .await;
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({ "status": "failed", "error": e.to_string() })),
            )
                .into_response()
        }
    }
}
