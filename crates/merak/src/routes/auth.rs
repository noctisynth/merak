use axum::{
    extract::{FromRequestParts, Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::{ToResponse, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};

use merak_core::SurrealClient;

use crate::auth::code::{
    AuthBearerCode, AuthCode, AuthCredentialCode, AuthTokenCode, AuthUserExistsCode,
    AuthUserNotFoundCode, AuthWeakPasswordCode,
};
use crate::auth::{jwt::TokenPair, service::AuthService};
use crate::common::response::{ApiResponse, EmptyData, ErrorResponse};

/// Authentication route state
#[derive(Clone)]
pub struct AuthState {
    pub db: Arc<SurrealClient>,
    pub auth_service: Arc<AuthService>,
}

/// Bearer token extractor
#[derive(Debug, Clone)]
pub struct BearerToken(Bearer);

impl FromRequestParts<AuthState> for BearerToken {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AuthState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|e| {
                    (
                        StatusCode::UNAUTHORIZED,
                        Json(ErrorResponse::new(AuthCode::Unauthorized, e.to_string())),
                    )
                        .into_response()
                })?;

        Ok(BearerToken(bearer))
    }
}

/// Registration request
#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    /// Username (3-50 characters)
    #[schema(min_length = 3, max_length = 50)]
    pub username: String,
    /// Email address
    #[schema(format = "email")]
    pub email: String,
    /// Password (at least 8 characters, containing uppercase, lowercase, and digits)
    #[schema(min_length = 8)]
    pub password: String,
}

/// Login request
#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    /// Username or email
    pub identifier: String,
    /// Password
    pub password: String,
}

/// Refresh token request
#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshTokenRequest {
    /// Refresh token
    pub refresh_token: String,
}

/// User response
#[derive(Debug, Serialize, ToSchema, ToResponse)]
pub struct UserResponse {
    /// User ID
    pub id: String,
    /// Username
    pub username: String,
    /// Email
    pub email: String,
    /// Creation timestamp
    pub created_at: String,
    /// Update timestamp
    pub updated_at: String,
}

impl From<crate::models::user::User> for UserResponse {
    fn from(user: crate::models::user::User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        }
    }
}

/// Registration response
#[derive(Debug, Serialize, ToSchema, ToResponse)]
pub struct RegisterResponse {
    /// User information
    pub user: UserResponse,
    /// Token pair
    pub tokens: TokenPair,
}

/// Login response
#[derive(Debug, Serialize, ToSchema, ToResponse)]
pub struct LoginResponse {
    /// User information
    pub user: UserResponse,
    /// Token pair
    pub tokens: TokenPair,
}

/// Refresh token response
#[derive(Debug, Serialize, ToSchema, ToResponse)]
pub struct RefreshTokenResponse {
    /// New token pair
    pub tokens: TokenPair,
}

/// User registration
///
/// Create a new user account and return access tokens
#[utoipa::path(
    post,
    path = "/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Registration successful", body = ApiResponse<RegisterResponse>),
        (status = 400, description = "Weak password", body = inline(ErrorResponse<AuthWeakPasswordCode>)),
        (status = 409, description = "Username or email already exists", body = inline(ErrorResponse<AuthUserExistsCode>)),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    tag = "Authentication"
)]
pub async fn register(
    State(state): State<AuthState>,
    Json(req): Json<RegisterRequest>,
) -> Response {
    let auth_service = state.auth_service.as_ref();

    match auth_service
        .register(&state.db, req.username, req.email, req.password)
        .await
    {
        Ok((user, tokens)) => (
            StatusCode::CREATED,
            Json(ApiResponse::ok(RegisterResponse {
                user: user.into(),
                tokens,
            })),
        )
            .into_response(),
        Err(e) => (
            e.status_code(),
            Json(ErrorResponse::new(e.code(), e.to_string())),
        )
            .into_response(),
    }
}

/// User login
///
/// Login with username or email and return access tokens
#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = ApiResponse<LoginResponse>),
        (status = 401, description = "Invalid credentials", body = inline(ErrorResponse<AuthCredentialCode>)),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    tag = "Authentication"
)]
pub async fn login(State(state): State<AuthState>, Json(req): Json<LoginRequest>) -> Response {
    let auth_service = state.auth_service.as_ref();

    match auth_service
        .login(&state.db, req.identifier, req.password)
        .await
    {
        Ok((user, tokens)) => (
            StatusCode::OK,
            Json(ApiResponse::ok(LoginResponse {
                user: user.into(),
                tokens,
            })),
        )
            .into_response(),
        Err(e) => (
            e.status_code(),
            Json(ErrorResponse::new(e.code(), e.to_string())),
        )
            .into_response(),
    }
}

/// Refresh access token
///
/// Use a refresh token to get new access token and refresh token
#[utoipa::path(
    post,
    path = "/refresh",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refresh successful", body = ApiResponse<RefreshTokenResponse>),
        (status = 401, description = "Token invalid or expired", body = inline(ErrorResponse<AuthTokenCode>)),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    tag = "Authentication"
)]
pub async fn refresh_token(
    State(state): State<AuthState>,
    Json(req): Json<RefreshTokenRequest>,
) -> Response {
    let auth_service = state.auth_service.as_ref();

    match auth_service
        .refresh_token(&state.db, req.refresh_token)
        .await
    {
        Ok(tokens) => (
            StatusCode::OK,
            Json(ApiResponse::ok(RefreshTokenResponse { tokens })),
        )
            .into_response(),
        Err(e) => (
            e.status_code(),
            Json(ErrorResponse::new(e.code(), e.to_string())),
        )
            .into_response(),
    }
}

/// User logout
///
/// Invalidate the current session token on the server
#[utoipa::path(
    post,
    path = "/logout",
    responses(
        (status = 200, description = "Logout successful", body = ApiResponse<EmptyData>),
        (status = 401, description = "Unauthorized", body = inline(ErrorResponse<AuthBearerCode>)),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Authentication"
)]
pub async fn logout(State(state): State<AuthState>, BearerToken(bearer): BearerToken) -> Response {
    let auth_service = state.auth_service.as_ref();

    let token = bearer.token();
    match auth_service.logout(&state.db, token).await {
        Ok(()) => (StatusCode::OK, Json(ApiResponse::ok(EmptyData::default()))).into_response(),
        Err(e) => (
            e.status_code(),
            Json(ErrorResponse::new(e.code(), e.to_string())),
        )
            .into_response(),
    }
}

/// Get current user information
///
/// Requires a valid access token in the request header: `Authorization: Bearer <token>`
#[utoipa::path(
    get,
    path = "/me",
    responses(
        (status = 200, description = "Successfully retrieved user information", body = ApiResponse<UserResponse>),
        (status = 401, description = "Unauthorized", body = inline(ErrorResponse<AuthBearerCode>)),
        (status = 404, description = "User not found", body = inline(ErrorResponse<AuthUserNotFoundCode>)),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Authentication"
)]
pub async fn get_me(State(state): State<AuthState>, BearerToken(bearer): BearerToken) -> Response {
    let auth_service = state.auth_service.as_ref();

    // Extract token from Authorization header
    let token = bearer.token();

    // Verify token and get user ID
    let claims = match auth_service.verify_access_token(&state.db, token).await {
        Ok(claims) => claims,
        Err(e) => {
            return (
                e.status_code(),
                Json(ErrorResponse::new(e.code(), e.to_string())),
            )
                .into_response();
        }
    };

    // Get user information
    match auth_service.get_user(&state.db, &claims.sub).await {
        Ok(user) => (
            StatusCode::OK,
            Json(ApiResponse::ok(UserResponse::from(user))),
        )
            .into_response(),
        Err(e) => (
            e.status_code(),
            Json(ErrorResponse::new(e.code(), e.to_string())),
        )
            .into_response(),
    }
}

/// Create authentication routes
pub fn routes() -> OpenApiRouter<AuthState> {
    OpenApiRouter::new()
        .routes(routes!(register))
        .routes(routes!(login))
        .routes(routes!(refresh_token))
        .routes(routes!(logout))
        .routes(routes!(get_me))
}

// pub struct AuthApiDoc;
