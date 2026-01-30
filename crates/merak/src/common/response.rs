use chrono::Utc;
use serde::Serialize;
use utoipa::{ToResponse, ToSchema};

pub use crate::common::code::CODE_OK;

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct EmptyData {}

#[derive(Debug, Serialize, ToSchema, ToResponse)]
pub struct ErrorResponse {
    /// Error message.
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[schema(bound = "T: ToSchema")]
pub struct ApiResponse<T> {
    /// Business result code for 2xx responses (CMMRR or 0).
    pub code: i32,
    /// Message describing the result.
    pub message: String,
    /// Server timestamp in milliseconds.
    pub timestamp: i64,
    /// Response payload.
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self::new(CODE_OK, "OK", data)
    }

    pub fn new(code: i32, message: impl Into<String>, data: T) -> Self {
        Self {
            timestamp: Utc::now().timestamp_millis(),
            code,
            message: message.into(),
            data,
        }
    }
}

impl ApiResponse<EmptyData> {
    pub fn error(code: i32, message: impl Into<String>) -> Self {
        Self::new(code, message, EmptyData::default())
    }
}
