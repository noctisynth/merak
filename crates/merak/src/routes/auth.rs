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

use crate::auth::{jwt::TokenPair, service::AuthService};
use crate::common::response::{ApiResponse, CODE_OK, EmptyData, ErrorResponse};

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
                        Json(ErrorResponse {
                            message: e.to_string(),
                        }),
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
        (status = 400, description = "Invalid request parameters", body = ErrorResponse),
        (status = 409, description = "Username or email already exists", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
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
        Err(e) => {
            let status = if e.to_string().contains("already exists") {
                StatusCode::CONFLICT
            } else if e.to_string().contains("Password must be") {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            let error_response = ErrorResponse {
                message: e.to_string(),
            };
            (status, Json(error_response)).into_response()
        }
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
        (status = 401, description = "Invalid username or password", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
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
        Err(e) => {
            let message = e.to_string();
            let status = if message.contains("Invalid credentials") {
                StatusCode::UNAUTHORIZED
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (status, Json(ErrorResponse { message })).into_response()
        }
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
        (status = 401, description = "Refresh token invalid or expired", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
pub async fn refresh_token(
    State(state): State<AuthState>,
    Json(req): Json<RefreshTokenRequest>,
) -> Response {
    let auth_service = state.auth_service.as_ref();

    match auth_service.refresh_token(req.refresh_token) {
        Ok(tokens) => (
            StatusCode::OK,
            Json(ApiResponse::ok(RefreshTokenResponse { tokens })),
        )
            .into_response(),
        Err(e) => {
            let status = if e.to_string().contains("Invalid") || e.to_string().contains("expired") {
                StatusCode::UNAUTHORIZED
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            let error_response = ErrorResponse {
                message: e.to_string(),
            };
            (status, Json(error_response)).into_response()
        }
    }
}

/// User logout
///
/// Client should delete stored tokens (server uses stateless JWT, no additional processing needed)
#[utoipa::path(
    post,
    path = "/logout",
    responses(
        (status = 200, description = "Logout successful", body = ApiResponse<EmptyData>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
pub async fn logout() -> impl IntoResponse {
    // Since we use stateless JWT, the server doesn't need to maintain session state
    // The client should delete locally stored tokens
    (
        StatusCode::OK,
        Json(ApiResponse::new(
            CODE_OK,
            "Logged out successfully",
            EmptyData::default(),
        )),
    )
}

/// Get current user information
///
/// Requires a valid access token in the request header: `Authorization: Bearer <token>`
#[utoipa::path(
    get,
    path = "/me",
    responses(
        (status = 200, description = "Successfully retrieved user information", body = ApiResponse<UserResponse>),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
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
    let user_id = match auth_service.extract_user_id(token) {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    message: e.to_string(),
                }),
            )
                .into_response();
        }
    };

    // Get user information
    match auth_service.get_user(&state.db, &user_id).await {
        Ok(user) => (
            StatusCode::OK,
            Json(ApiResponse::ok(UserResponse::from(user))),
        )
            .into_response(),
        Err(e) => {
            let status = if e.to_string().contains("not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            let error_response = ErrorResponse {
                message: e.to_string(),
            };
            (status, Json(error_response)).into_response()
        }
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
