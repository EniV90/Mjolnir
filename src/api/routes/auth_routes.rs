use axum::{Router, routing::post};

use crate::{api::handlers::auth_handler::{loging_handler, logout_handler}, 
application::state::SharedState
};

pub fn routes() -> Router<SharedState> {
  Router::new()
    .route("/login", post(loging_handler))
    .route("logout", post(logout_handler))
}