use std::marker::PhantomData;

use chrono::Utc;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa::openapi::{
    KnownFormat, ObjectBuilder, RefOr, Schema, SchemaFormat,
    schema::{Ref, Type},
};

pub use crate::common::code::CODE_OK;
use crate::common::code::{BusinessCode, SuccessCode};

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct EmptyData {}

// --- ErrorResponse ---

#[derive(Debug, Serialize)]
#[serde(bound(serialize = ""))]
pub struct ErrorResponse<C = BusinessCode> {
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

impl<C> utoipa::__dev::ComposeSchema for ErrorResponse<C> {
    fn compose(mut generics: Vec<RefOr<Schema>>) -> RefOr<Schema> {
        let code_schema = if generics.is_empty() {
            <BusinessCode as utoipa::PartialSchema>::schema()
        } else {
            generics.remove(0)
        };
        error_response_schema(code_schema)
    }
}

impl<C> utoipa::ToSchema for ErrorResponse<C> {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("ErrorResponse")
    }
}

// --- ApiResponse ---

#[derive(Debug, Serialize)]
#[serde(bound(serialize = "T: Serialize"))]
pub struct ApiResponse<T, C = SuccessCode> {
    pub code: BusinessCode,
    pub message: String,
    pub timestamp: i64,
    pub data: T,
    #[serde(skip)]
    _codes: PhantomData<C>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self::new(CODE_OK, "OK", data)
    }

    pub fn new(code: impl Into<BusinessCode>, message: impl Into<String>, data: T) -> Self {
        Self {
            timestamp: Utc::now().timestamp_millis(),
            code: code.into(),
            message: message.into(),
            data,
            _codes: PhantomData,
        }
    }
}

impl ApiResponse<EmptyData> {
    pub fn error(code: impl Into<BusinessCode>, message: impl Into<String>) -> Self {
        Self::new(code, message, EmptyData::default())
    }
}

impl<T: utoipa::ToSchema, C> utoipa::__dev::ComposeSchema for ApiResponse<T, C> {
    fn compose(_generics: Vec<RefOr<Schema>>) -> RefOr<Schema> {
        let data_schema = RefOr::Ref(Ref::from_schema_name(T::name().as_ref()));
        api_response_schema(
            <SuccessCode as utoipa::PartialSchema>::schema(),
            data_schema,
        )
    }
}

impl<T: utoipa::ToSchema, C> utoipa::ToSchema for ApiResponse<T, C> {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("ApiResponse")
    }
}

// --- Schema helpers ---

fn timestamp_schema() -> RefOr<Schema> {
    ObjectBuilder::new()
        .schema_type(Type::Integer)
        .format(Some(SchemaFormat::KnownFormat(KnownFormat::Int64)))
        .into()
}

fn error_response_schema(code_schema: RefOr<Schema>) -> RefOr<Schema> {
    ObjectBuilder::new()
        .property("code", code_schema)
        .required("code")
        .property("message", ObjectBuilder::new().schema_type(Type::String))
        .required("message")
        .property("timestamp", timestamp_schema())
        .required("timestamp")
        .into()
}

fn api_response_schema(code_schema: RefOr<Schema>, data_schema: RefOr<Schema>) -> RefOr<Schema> {
    ObjectBuilder::new()
        .property("code", code_schema)
        .required("code")
        .property("message", ObjectBuilder::new().schema_type(Type::String))
        .required("message")
        .property("timestamp", timestamp_schema())
        .required("timestamp")
        .property("data", data_schema)
        .required("data")
        .into()
}
