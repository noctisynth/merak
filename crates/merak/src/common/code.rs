pub const CODE_OK: i32 = 0;

pub mod category {
    pub const SUCCESS: i32 = 0;
    pub const BUSINESS_ERROR: i32 = 1;
    pub const PROCESSING: i32 = 2;
    pub const PARTIAL_SUCCESS: i32 = 3;
    pub const UNKNOWN_ERROR: i32 = 9;
}

pub mod module {
    pub const AUTH: i32 = 1;
    pub const USER: i32 = 2;
    pub const ORG: i32 = 3;
    pub const PROJECT: i32 = 4;
    pub const SPACE: i32 = 5;
    pub const WORKFLOW: i32 = 6;
    pub const NODE: i32 = 7;
    pub const SUBTASK: i32 = 8;
    pub const LINK: i32 = 9;
    pub const DOC: i32 = 10;
    pub const COMMENT: i32 = 11;
    pub const NOTIFICATION: i32 = 12;
    pub const COMMON: i32 = 99;
}

/// Build a business code using the CMMRR scheme.
/// C: category, MM: module, RR: reason.
pub const fn make_code(category: i32, module: i32, reason: i32) -> i32 {
    (category * 10000) + (module * 100) + reason
}

/// Common module error codes
pub mod common {
    use super::*;

    /// Resource not found
    pub const NOT_FOUND: i32 = make_code(category::BUSINESS_ERROR, module::COMMON, 1);
}

/// Authentication module error codes
pub mod auth {
    use super::*;

    /// Invalid credentials (wrong username/email or password)
    pub const INVALID_CREDENTIALS: i32 = make_code(category::BUSINESS_ERROR, module::AUTH, 1);

    /// User already exists (username or email conflict)
    pub const USER_EXISTS: i32 = make_code(category::BUSINESS_ERROR, module::AUTH, 2);

    /// Password does not meet strength requirements
    pub const WEAK_PASSWORD: i32 = make_code(category::BUSINESS_ERROR, module::AUTH, 3);

    /// Token has expired
    pub const TOKEN_EXPIRED: i32 = make_code(category::BUSINESS_ERROR, module::AUTH, 4);

    /// Token is invalid or malformed
    pub const TOKEN_INVALID: i32 = make_code(category::BUSINESS_ERROR, module::AUTH, 5);

    /// Session is invalid or has been revoked
    pub const SESSION_INVALID: i32 = make_code(category::BUSINESS_ERROR, module::AUTH, 6);

    /// User not found
    pub const USER_NOT_FOUND: i32 = make_code(category::BUSINESS_ERROR, module::AUTH, 7);

    /// Unauthorized (missing or invalid authorization header)
    pub const UNAUTHORIZED: i32 = make_code(category::BUSINESS_ERROR, module::AUTH, 8);
}
