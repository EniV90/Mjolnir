use core::error;
use std::fmt::{Display, Formatter, Result};

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct APIError {
    pub status: u16,
    pub error: Vec<APIErrorEntry>,
}

impl Display for APIError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let api_error = serde_json::to_string_pretty(&self).unwrap_or_default();
        write!(f, "{}", api_error)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum APIErrorCode {
    AuthenticationWrongCredentials,
    AuthenticationMissingCredentials,
    AuthenticationTokenCreationError,
    AuthenticationInvalidToken,
    AuthenticationRevokedTokensInactive,
    AuthenticationForbidden,
    ResourceNotFound,
    UserNotFound,
    ApiVersionError,
    DatabaseError,
    RedisError,
}

impl Display for APIErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            serde_json::json!(self).as_str().unwrap_or_default()
        )
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum APIErrorKind {
    AuthenticationError,
    ResourceNotFound,
    ValidationError,
    DatabaseError,
    RedisError,
}

impl Display for APIErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            serde_json::json!(self).as_str().unwrap_or_default()
        )
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct APIErrorEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl APIErrorEntry {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            timestamp: Utc::now(),
            ..Default::default()
        }
    }

    pub fn code<S: ToString>(mut self, code: S) -> Self {
        self.code = Some(code.to_string());
        self
    }

    pub fn kind<S: ToString>(mut self, kind: S) -> Self {
        self.kind = Some(kind.to_string());
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn detail(mut self, detail: serde_json::Value) -> Self {
        self.detail = Some(detail);
        self
    }

    pub fn reason(mut self, reason: &str) -> Self {
        self.reason = Some(reason.to_owned());
        self
    }

    pub fn instance(mut self, instance: &str) -> Self {
        self.instance = Some(instance.to_owned());
        self
    }

    pub fn trace_id(mut self) -> Self {
        let mut trace_id = uuid::Uuid::new_v4().to_string();
        trace_id.retain(|c| c != '_');
        self.trace_id = Some(trace_id);
        self
    }
}

impl From<StatusCode> for APIErrorEntry {
    fn from(status_code: StatusCode) -> Self {
        let error_message = status_code.to_string();
        let error_code = error_message.replace(" ", "_").to_lowercase();
        Self::new(&error_message).code(error_code)
    }
}

impl From<sqlx::Error> for APIErrorEntry {
    fn from(e: sqlx::Error) -> Self {
        if cfg!(debug_assertions) {
            let (code, kind) = match e {
                sqlx::Error::RowNotFound => (
                    APIErrorCode::ResourceNotFound,
                    APIErrorKind::ResourceNotFound,
                ),
                _ => (APIErrorCode::DatabaseError, APIErrorKind::DatabaseError),
            };
            Self::new(&e.to_string()).code(code).kind(kind).trace_id()
        } else {
            let error_entry = Self::from(StatusCode::INTERNAL_SERVER_ERROR).trace_id();
            let trace_id = error_entry.trace_id.as_deref().unwrap_or("");

            tracing::error!("SQLX error: {}, trace id: {}", e.to_string(), trace_id);
            error_entry
        }
    }
}

impl From<redis::RedisError> for APIErrorEntry {
    fn from(e: redis::RedisError) -> Self {
        if cfg!(debug_assertions) {
            Self::new(&e.to_string())
                .code(APIErrorCode::RedisError)
                .kind(APIErrorKind::RedisError)
                .description(&format!("Redis error: {}", e))
                .trace_id()
        } else {
            let error_entry = Self::from(StatusCode::INTERNAL_SERVER_ERROR).trace_id();
            let trace_id = error_entry.trace_id.as_deref().unwrap_or("");

            tracing::error!("Redis error: {}, trace id: {}", e.to_string(), trace_id);
            error_entry
        }
    }
}

impl From<(StatusCode, Vec<APIErrorEntry>)> for APIError {
    fn from(error_from: (StatusCode, Vec<APIErrorEntry>)) -> Self {
        let (status_code, error) = error_from;
        Self {
            status: status_code.as_u16(),
            error,
        }
    }
}

impl From<(StatusCode, APIErrorEntry)> for APIError {
    fn from(error_from: (StatusCode, APIErrorEntry)) -> Self {
        let (status_code, error_entry) = error_from;
        Self {
            status: status_code.as_u16(),
            error: vec![error_entry],
        }
    }
}

impl From<StatusCode> for APIError {
    fn from(status_code: StatusCode) -> Self {
        Self {
            status: status_code.as_u16(),
            error: vec![status_code.into()],
        }
    }
}

impl From<sqlx::Error> for APIError {
    fn from(error: sqlx::Error) -> Self {
        let status_code = match error {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self {
            status: status_code.as_u16(),
            error: vec![APIErrorEntry::from(error)],
        }
    }
}

impl From<redis::RedisError> for APIError {
    fn from(error: redis::RedisError) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            error: vec![APIErrorEntry::from(error)],
        }
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        tracing::error!("Error response: {:?}", self);
        let status_code =
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status_code, Json(self)).into_response()
    }
}
