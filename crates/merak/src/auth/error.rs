use std::{error::Error as StdError, fmt};

use anyhow::Error as AnyError;
use axum::http::StatusCode;

use crate::auth::code::AuthCode;
use crate::common::code::{BusinessCode, Category, Module, make_code};

#[derive(Debug)]
pub enum AuthError {
    WeakPassword,
    UsernameExists,
    EmailExists,
    InvalidCredentials,
    TokenExpired,
    TokenInvalid(String),
    TokenRevoked,
    SessionExpired,
    SessionInvalid(String),
    UserNotFound,
    InvalidOldPassword,
    Internal(AnyError),
}

pub type AuthResult<T> = std::result::Result<T, AuthError>;

impl AuthError {
    pub fn code(&self) -> BusinessCode {
        match self {
            AuthError::WeakPassword => AuthCode::WeakPassword.into(),
            AuthError::UsernameExists | AuthError::EmailExists => AuthCode::UserExists.into(),
            AuthError::InvalidCredentials | AuthError::InvalidOldPassword => {
                AuthCode::InvalidCredentials.into()
            }
            AuthError::TokenExpired | AuthError::SessionExpired => AuthCode::TokenExpired.into(),
            AuthError::TokenInvalid(_) | AuthError::TokenRevoked => AuthCode::TokenInvalid.into(),
            AuthError::SessionInvalid(_) => AuthCode::SessionInvalid.into(),
            AuthError::UserNotFound => AuthCode::UserNotFound.into(),
            AuthError::Internal(_) => make_code(Category::UnknownError, Module::Auth, 99),
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            AuthError::WeakPassword => StatusCode::BAD_REQUEST,
            AuthError::UsernameExists | AuthError::EmailExists => StatusCode::CONFLICT,
            AuthError::InvalidCredentials | AuthError::InvalidOldPassword => {
                StatusCode::UNAUTHORIZED
            }
            AuthError::TokenExpired | AuthError::SessionExpired => StatusCode::UNAUTHORIZED,
            AuthError::TokenInvalid(_) | AuthError::TokenRevoked => StatusCode::UNAUTHORIZED,
            AuthError::SessionInvalid(_) => StatusCode::UNAUTHORIZED,
            AuthError::UserNotFound => StatusCode::NOT_FOUND,
            AuthError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::WeakPassword => write!(
                f,
                "Password must be at least 8 characters and contain uppercase, lowercase, and numbers"
            ),
            AuthError::UsernameExists => write!(f, "Username already exists"),
            AuthError::EmailExists => write!(f, "Email already exists"),
            AuthError::InvalidCredentials => write!(f, "Invalid credentials"),
            AuthError::TokenExpired => write!(f, "Token expired"),
            AuthError::TokenInvalid(reason) => {
                if reason.is_empty() {
                    write!(f, "Invalid token")
                } else {
                    write!(f, "{}", reason)
                }
            }
            AuthError::TokenRevoked => write!(f, "Refresh token revoked"),
            AuthError::SessionExpired => write!(f, "Session expired"),
            AuthError::SessionInvalid(reason) => {
                if reason.is_empty() {
                    write!(f, "Session invalid")
                } else {
                    write!(f, "{}", reason)
                }
            }
            AuthError::UserNotFound => write!(f, "User not found"),
            AuthError::InvalidOldPassword => write!(f, "Invalid old password"),
            AuthError::Internal(err) => write!(f, "{}", err),
        }
    }
}

impl StdError for AuthError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            AuthError::Internal(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

impl From<AnyError> for AuthError {
    fn from(err: AnyError) -> Self {
        AuthError::Internal(err)
    }
}

impl From<surrealdb::Error> for AuthError {
    fn from(err: surrealdb::Error) -> Self {
        AuthError::Internal(err.into())
    }
}
