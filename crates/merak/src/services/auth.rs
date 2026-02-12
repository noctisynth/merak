use anyhow::anyhow;
use chrono::Utc;
use merak_core::{Model, SurrealClient};
use surrealdb::RecordId;

use super::{
    error::{AuthError, AuthResult},
    jwt::{JwtService, TokenPair},
    password::PasswordService,
    session::SessionService,
};
use crate::models::auth::{User, UserInput};

/// Authentication service for user registration, login, and token management
pub struct AuthService {
    jwt_service: JwtService,
    password_service: PasswordService,
    session_service: SessionService,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(jwt_service: JwtService, password_service: PasswordService) -> Self {
        Self {
            jwt_service,
            password_service,
            session_service: SessionService::new(),
        }
    }

    /// Create authentication service with default configuration
    pub fn with_default_config() -> Self {
        Self {
            jwt_service: JwtService::default(),
            password_service: PasswordService::default(),
            session_service: SessionService::new(),
        }
    }

    /// Create authentication service from environment variables
    pub fn from_env() -> Self {
        Self {
            jwt_service: JwtService::from_env(),
            password_service: PasswordService::default(),
            session_service: SessionService::new(),
        }
    }

    /// Register a new user
    ///
    /// # Arguments
    /// - `db`: Database client
    /// - `username`: Username
    /// - `email`: Email address
    /// - `password`: Password
    ///
    /// # Returns
    /// The newly created user and token pair
    pub async fn register(
        &self,
        db: &SurrealClient,
        username: String,
        email: String,
        password: String,
    ) -> AuthResult<(User, TokenPair)> {
        // Validate password strength
        if !PasswordService::check_password_strength(&password) {
            return Err(AuthError::WeakPassword);
        }

        // Check if username already exists
        let existing_by_username: Option<User> = db
            .query("SELECT * FROM type::table($table) WHERE username = $username")
            .bind(("table", User::TABLE_NAME))
            .bind(("username", username.clone()))
            .await?
            .take(0)?;
        if existing_by_username.is_some() {
            return Err(AuthError::UsernameExists);
        }

        // Check if email already exists
        let existing_by_email: Option<User> = db
            .query("SELECT * FROM type::table($table) WHERE email = $email")
            .bind(("table", User::TABLE_NAME))
            .bind(("email", email.clone()))
            .await?
            .take(0)?;
        if existing_by_email.is_some() {
            return Err(AuthError::EmailExists);
        }

        // Hash the password
        let password_hash = self.password_service.hash_password(&password)?;

        // Create user
        let now = Utc::now();
        let user_input = UserInput {
            username,
            email,
            password_hash,
            created_at: now,
            updated_at: now,
        };

        let created = User::objects(db).create(user_input).await?;

        let user = created.ok_or_else(|| AuthError::Internal(anyhow!("Failed to create user")))?;

        let session = self
            .session_service
            .create_session(db, &user.id, self.jwt_service.refresh_exp_seconds())
            .await?;
        let token_pair = self.jwt_service.generate_token_pair(
            &user.id.to_string(),
            &user.username,
            &user.email,
            &session.session_id,
            &session.refresh_jti,
        )?;

        Ok((user, token_pair))
    }

    /// User login
    ///
    /// # Arguments
    /// - `db`: Database client
    /// - `identifier`: Username or email
    /// - `password`: Password
    ///
    /// # Returns
    /// The user and token pair
    pub async fn login(
        &self,
        db: &SurrealClient,
        identifier: String,
        password: String,
    ) -> AuthResult<(User, TokenPair)> {
        // Find user (by username or email)
        let user: Option<User> = db
            .query("SELECT * FROM type::table($table) WHERE username = $identifier OR email = $identifier")
            .bind(("table", User::TABLE_NAME))
            .bind(("identifier", identifier.clone()))
            .await?
            .take(0)?;

        let user = user.ok_or(AuthError::InvalidCredentials)?;

        // Verify password
        let is_valid = self
            .password_service
            .verify_password(&password, &user.password_hash)?;

        if !is_valid {
            return Err(AuthError::InvalidCredentials);
        }

        self.session_service
            .cleanup_expired_for_user(db, &user.id)
            .await?;
        let session = self
            .session_service
            .create_session(db, &user.id, self.jwt_service.refresh_exp_seconds())
            .await?;
        let token_pair = self.jwt_service.generate_token_pair(
            &user.id.to_string(),
            &user.username,
            &user.email,
            &session.session_id,
            &session.refresh_jti,
        )?;

        Ok((user, token_pair))
    }

    /// Refresh tokens
    ///
    /// # Arguments
    /// - `db`: Database client
    /// - `refresh_token`: Refresh token
    ///
    /// # Returns
    /// New token pair
    pub async fn refresh_token(
        &self,
        db: &SurrealClient,
        refresh_token: String,
    ) -> AuthResult<TokenPair> {
        // Verify refresh token
        let claims = self.jwt_service.verify_refresh_token(&refresh_token)?;
        let refresh_jti = claims
            .jti
            .clone()
            .ok_or_else(|| AuthError::TokenInvalid("Refresh token missing jti".to_string()))?;
        let session = self
            .session_service
            .load_active_session(db, &claims.sid)
            .await?;
        if session.user_id.to_string() != claims.sub {
            return Err(AuthError::SessionInvalid(
                "Session user mismatch".to_string(),
            ));
        }
        if session.refresh_jti != refresh_jti {
            return Err(AuthError::TokenRevoked);
        }

        let new_refresh_jti = self
            .session_service
            .rotate_refresh_jti(db, session, self.jwt_service.refresh_exp_seconds())
            .await?;

        // Generate new token pair
        let token_pair = self.jwt_service.generate_token_pair(
            &claims.sub,
            &claims.username,
            &claims.email,
            &claims.sid,
            &new_refresh_jti,
        )?;

        Ok(token_pair)
    }

    /// Verify access token
    ///
    /// # Arguments
    /// - `access_token`: Access token
    ///
    /// # Returns
    /// Token claims
    pub async fn verify_access_token(
        &self,
        db: &SurrealClient,
        access_token: &str,
    ) -> AuthResult<super::jwt::Claims> {
        let claims = self.jwt_service.verify_access_token(access_token)?;
        let session = self
            .session_service
            .load_active_session(db, &claims.sid)
            .await?;
        if session.user_id.to_string() != claims.sub {
            return Err(AuthError::SessionInvalid(
                "Session user mismatch".to_string(),
            ));
        }
        Ok(claims)
    }

    /// Extract user ID from token
    ///
    /// # Arguments
    /// - `access_token`: Access token
    ///
    /// # Returns
    /// User ID
    pub async fn extract_user_id(
        &self,
        db: &SurrealClient,
        access_token: &str,
    ) -> AuthResult<String> {
        let claims = self.verify_access_token(db, access_token).await?;
        Ok(claims.sub)
    }

    /// Logout current session
    ///
    /// # Arguments
    /// - `db`: Database client
    /// - `access_token`: Access token
    pub async fn logout(&self, db: &SurrealClient, access_token: &str) -> AuthResult<()> {
        let claims = self.verify_access_token(db, access_token).await?;
        self.session_service.delete_session(db, &claims.sid).await
    }

    /// Get user information
    ///
    /// # Arguments
    /// - `db`: Database client
    /// - `user_id`: User ID
    ///
    /// # Returns
    /// User information
    pub async fn get_user(&self, db: &SurrealClient, user_id: &str) -> AuthResult<User> {
        let record_id: RecordId = user_id
            .parse()
            .map_err(|e| AuthError::Internal(anyhow!("Failed to parse user id: {}", e)))?;
        let user = User::get_by_id(db, &record_id.key().to_string()).await?;
        user.ok_or(AuthError::UserNotFound)
    }

    /// Update user password
    ///
    /// # Arguments
    /// - `db`: Database client
    /// - `user_id`: User ID
    /// - `old_password`: Old password
    /// - `new_password`: New password
    ///
    /// # Returns
    /// Updated user
    pub async fn update_password(
        &self,
        db: &SurrealClient,
        user_id: &str,
        old_password: String,
        new_password: String,
    ) -> AuthResult<Option<User>> {
        // Validate new password strength
        if !PasswordService::check_password_strength(&new_password) {
            return Err(AuthError::WeakPassword);
        }

        // Get the user
        let mut user = self.get_user(db, user_id).await?;

        // Verify old password
        let is_valid = self
            .password_service
            .verify_password(&old_password, &user.password_hash)?;

        if !is_valid {
            return Err(AuthError::InvalidOldPassword);
        }

        // Hash the new password
        let new_password_hash = self.password_service.hash_password(&new_password)?;

        // Update password
        user.password_hash = new_password_hash;
        user.updated_at = Utc::now();

        let updated = user.save(db).await?;

        Ok(updated)
    }
}

impl Default for AuthService {
    fn default() -> Self {
        Self::with_default_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_service_creation() {
        let _service = AuthService::default();
    }

    #[test]
    fn test_password_strength_validation() {
        // Valid passwords
        assert!(PasswordService::check_password_strength("Test1234"));
        assert!(PasswordService::check_password_strength("MySecurePass123"));

        // Invalid passwords
        assert!(!PasswordService::check_password_strength("test1234")); // No uppercase
        assert!(!PasswordService::check_password_strength("TEST1234")); // No lowercase
        assert!(!PasswordService::check_password_strength("TestPass")); // No digits
        assert!(!PasswordService::check_password_strength("Test1")); // Too short
    }
}
