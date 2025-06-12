pub mod api;
pub mod application;
pub mod infrastructure;
pub mod domain;


use crate::application::{app, state::AppState};
use crate::infrastructure::{database, redis};