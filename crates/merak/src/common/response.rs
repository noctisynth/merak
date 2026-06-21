use std::marker::PhantomData;

use chrono::Utc;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa::openapi::Ref;
use utoipa::openapi::{RefOr, Schema};

use crate::common::code::BusinessCode;
pub use crate::common::code::CODE_OK;

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct EmptyData {}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse<C: ToSchema = BusinessCode> {
    #[schema(schema_with = compose_schame::<C>)]
    pub code: BusinessCode,
    pub message: String,
    pub timestamp: i64,
    #[serde(skip)]
    _codes: PhantomData<C>,
}

impl ErrorResponse {
    pub fn new(code: impl Into<BusinessCode>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            timestamp: Utc::now().timestamp_millis(),
            _codes: PhantomData,
        }
    }
}

fn compose_schame<T: ToSchema>() -> RefOr<Schema> {
    RefOr::Ref(Ref::from_schema_name(T::name().as_ref()))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T: Serialize + ToSchema> {
    pub code: BusinessCode,
    pub message: String,
    pub timestamp: i64,
    #[schema(schema_with = compose_schame::<T>)]
    pub data: T,
}

impl<T: Serialize + ToSchema> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self::new(CODE_OK, "OK", data)
    }

    pub fn new(code: impl Into<BusinessCode>, message: impl Into<String>, data: T) -> Self {
        Self {
            timestamp: Utc::now().timestamp_millis(),
            code: code.into(),
            message: message.into(),
            data,
        }
    }
}

impl ApiResponse<EmptyData> {
    pub fn error(code: impl Into<BusinessCode>, message: impl Into<String>) -> Self {
        Self::new(code, message, EmptyData::default())
    }
}
