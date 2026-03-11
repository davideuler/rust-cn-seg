pub mod handlers;
pub mod models;

use axum::{routing::{get, post}, Router};
use tower_http::cors::CorsLayer;
use crate::api::handlers::*;

pub fn create_router() -> Router {
    Router::new()
        .route("/api/health", get(health_handler))
        .route("/api/segment", post(segment_handler))
        .route("/api/sensitive", post(sensitive_handler))
        .route("/api/analyze", post(analyze_handler))
        .route("/api/dict/add", post(add_word_handler))
        .layer(CorsLayer::permissive())
}
