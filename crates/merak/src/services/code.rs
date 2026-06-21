use crate::common::code::{Category, Module, combine_codes, define_codes};

// Full enum for runtime use
define_codes!(AuthCode, Category::BusinessError, Module::Auth, {
    /// Invalid credentials
    InvalidCredentials = 1,
    /// User already exists
    UserExists = 2,
    /// Weak password
    WeakPassword = 3,
    /// Token expired
    TokenExpired = 4,
    /// Token invalid
    TokenInvalid = 5,
    /// Session invalid
    SessionInvalid = 6,
    /// User not found
    UserNotFound = 7,
    /// Unauthorized
    Unauthorized = 8,
});

// --- Schema-only sub-enums for precise per-endpoint OpenAPI annotations ---

define_codes!(AuthWeakPasswordCode, Category::BusinessError, Module::Auth, {
    /// Weak password
    WeakPassword = 3,
});

define_codes!(AuthUserExistsCode, Category::BusinessError, Module::Auth, {
    /// User already exists
    UserExists = 2,
});

define_codes!(AuthCredentialCode, Category::BusinessError, Module::Auth, {
    /// Invalid credentials
    InvalidCredentials = 1,
});

define_codes!(AuthTokenCode, Category::BusinessError, Module::Auth, {
    /// Token expired
    TokenExpired = 4,
    /// Token invalid
    TokenInvalid = 5,
    /// Session invalid
    SessionInvalid = 6,
});

define_codes!(AuthUnauthorizedCode, Category::BusinessError, Module::Auth, {
    /// Unauthorized
    Unauthorized = 8,
});

define_codes!(AuthUserNotFoundCode, Category::BusinessError, Module::Auth, {
    /// User not found
    UserNotFound = 7,
});

combine_codes!(AuthBearerCode, [AuthTokenCode, AuthUnauthorizedCode]);
