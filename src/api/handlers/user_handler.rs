use axum::{
  Router,
  extract::State,
  response::{IntoResponse, Response},
  routing::{get, post},
};

use crate::application::state::SharedState;
use hyper::StatusCode;

pub async fn list_users_handler(State(_state): State<SharedState>) -> Response {
  todo!()
}

pub async fn add_user_handler(State(_state): State<SharedState>) -> Response {
  todo!()
}

pub async fn get_user_handler(State(_state): State<SharedState>) -> Response {
  todo!()
}

pub async fn update_user_handler(State(_state): State<SharedState>) -> Response {
  todo!()
}

pub async fn delete_user_handler(State(_state): State<SharedState>) -> Response {
  todo!()
}