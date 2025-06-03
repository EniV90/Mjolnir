use axum::{
    extract::Request,
    response::IntoResponse,
    routing::get,
    Json, Router
};
use hyper::StatusCode;
use serde_json::json;

pub fn router() -> Router {
    Router::new()
    .route("/", get(root_handler))
    .fallback(error_404_handler)
}

async fn root_handler() -> impl IntoResponse {
    Json(json!({"message": "This is mjolnir"}))
}

async fn error_404_handler(request: Request) -> impl IntoResponse {
    tracing::error!("route not found: {:?}", request.uri());
    (StatusCode::NOT_FOUND, Json(json!({"error": "Not Found"})))
}