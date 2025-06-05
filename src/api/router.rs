use std::collections::HashMap;

use axum::{
    Json, Router,
    body::Body,
    extract::{Query, Request},
    middleware::Next,
    response::{IntoResponse, Response},
    routing::{any, get},
};
use hyper::{HeaderMap, Method, StatusCode};
use serde_json::json;
use sqlx::PgPool;
use crate::application::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(root_handler))
        .route("/head", get(head_request_handler))
        .route("/any", any(any_request_handler))
        .fallback(error_404_handler)
}

#[tracing::instrument(level = tracing::Level::TRACE, name = "mjolnir", skip_all, fields(method=request.method().to_string(),uri=request.uri().to_string()))]
pub async fn logging_middleware(request: Request<Body>, next: Next) -> Response {
    tracing::trace!(
        "received a {} request to {}",
        request.method(),
        request.uri()
    );
    next.run(request).await
}

async fn root_handler() -> impl IntoResponse {
    Json(json!({"message": "This is mjolnir"}))
}

async fn head_request_handler(method: Method) -> Response {
    if method == Method::HEAD {
        tracing::debug!("Head not found");
        return [("x-some-header", "header from HEAD")].into_response();
    }
    ([("x-some-header", "header from HEAD")], "body from GET").into_response()
}

async fn any_request_handler(
    method: Method,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    request: Request,
) -> impl IntoResponse {
    if tracing::enabled!(tracing::Level::DEBUG) {
        tracing::debug!("method: {:?}", method);
        tracing::debug!("headers: {:?}", headers);
        tracing::debug!("params: {:?}", params);
        tracing::debug!("request: {:?}", request)
    }
    StatusCode::OK
}

async fn error_404_handler(request: Request) -> impl IntoResponse {
    tracing::error!("route not found: {:?}", request.uri());
    (StatusCode::NOT_FOUND, Json(json!({"error": "Not Found"})))
}
