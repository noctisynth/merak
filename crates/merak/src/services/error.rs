use std::{error::Error as StdError, fmt};

use anyhow::Error as AnyError;

use crate::common::code;

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
    pub fn code(&self) -> i32 {
        match self {
            AuthError::WeakPassword => code::auth::WEAK_PASSWORD,
            AuthError::UsernameExists | AuthError::EmailExists => code::auth::USER_EXISTS,
            AuthError::InvalidCredentials | AuthError::InvalidOldPassword => {
                code::auth::INVALID_CREDENTIALS
            }
            AuthError::TokenExpired | AuthError::SessionExpired => code::auth::TOKEN_EXPIRED,
            AuthError::TokenInvalid(_) | AuthError::TokenRevoked => code::auth::TOKEN_INVALID,
            AuthError::SessionInvalid(_) => code::auth::SESSION_INVALID,
            AuthError::UserNotFound => code::auth::USER_NOT_FOUND,
            AuthError::Internal(_) => {
                code::make_code(code::category::UNKNOWN_ERROR, code::module::AUTH, 99)
            }
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
