use axum::{
    Router,
    extract::State,
    response::{IntoResponse, Response},
    routing::{get, post},
};

use crate::application::state::SharedState;
use hyper::StatusCode;

pub async fn loging_handler(State(_state): State<SharedState>) -> Response {
    tracing::debug!("entered: get_login_handler()");
    (StatusCode::FORBIDDEN, "forbidden").into_response()
}

pub async fn logout_handler(State(_state): State<SharedState>) -> Response {
    tracing::debug!("entered: get_logout_handler()");
    (StatusCode::FORBIDDEN, "forbidden").into_response()
}
