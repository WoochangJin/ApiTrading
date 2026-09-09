use std::sync::Arc;

use axum::{Router, routing::{get, post}};

use crate::handlers;
use crate::state::AppState;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/cycle/run", post(handlers::run_cycle))
        .with_state(state)
}
