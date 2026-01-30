use anyhow::{Result, anyhow};
use chrono::Utc;
use merak_core::{Model, SurrealClient};
use surrealdb::RecordId;

use super::{
    jwt::{JwtService, TokenPair},
    password::PasswordService,
};
use crate::models::user::{User, UserInput};

/// Authentication service for user registration, login, and token management
pub struct AuthService {
    jwt_service: JwtService,
    password_service: PasswordService,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(jwt_service: JwtService, password_service: PasswordService) -> Self {
        Self {
            jwt_service,
            password_service,
        }
    }

    /// Create authentication service with default configuration
    pub fn with_default_config() -> Self {
        Self {
            jwt_service: JwtService::default(),
            password_service: PasswordService::default(),
        }
    }

    /// Create authentication service from environment variables
    pub fn from_env() -> Self {
        Self {
            jwt_service: JwtService::from_env(),
            password_service: PasswordService::default(),
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
    ) -> Result<(User, TokenPair)> {
        // Validate password strength
        if !PasswordService::check_password_strength(&password) {
            return Err(anyhow!(
                "Password must be at least 8 characters and contain uppercase, lowercase, and numbers"
            ));
        }

        // Check if username already exists
        let existing_by_username: Option<User> = db
            .query("SELECT * FROM type::table($table) WHERE username = $username")
            .bind(("table", User::TABLE_NAME))
            .bind(("username", username.clone()))
            .await?
            .take(0)?;
        if existing_by_username.is_some() {
            return Err(anyhow!("Username already exists"));
        }

        // Check if email already exists
        let existing_by_email: Option<User> = db
            .query("SELECT * FROM type::table($table) WHERE email = $email")
            .bind(("table", User::TABLE_NAME))
            .bind(("email", email.clone()))
            .await?
            .take(0)?;
        if existing_by_email.is_some() {
            return Err(anyhow!("Email already exists"));
        }

        // Hash the password
        let password_hash = self.password_service.hash_password(&password)?;

        // Create user
        let user_input = UserInput {
            username,
            email,
            password_hash,
        };

        let created = User::objects(db).create(user_input).await?;

        let user = created.ok_or_else(|| anyhow!("Failed to create user"))?;

        // Generate tokens
        let token_pair = self.jwt_service.generate_token_pair(
            &user.id.to_string(),
            &user.username,
            &user.email,
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
    ) -> Result<(User, TokenPair)> {
        // Find user (by username or email)
        let user: Option<User> = db
            .query("SELECT * FROM type::table($table) WHERE username = $identifier OR email = $identifier")
            .bind(("table", User::TABLE_NAME))
            .bind(("identifier", identifier.clone()))
            .await?
            .take(0)?;

        let user = user.ok_or_else(|| anyhow!("Invalid credentials"))?;

        // Verify password
        let is_valid = self
            .password_service
            .verify_password(&password, &user.password_hash)?;

        if !is_valid {
            return Err(anyhow!("Invalid credentials"));
        }

        // Generate tokens
        let token_pair = self.jwt_service.generate_token_pair(
            &user.id.to_string(),
            &user.username,
            &user.email,
        )?;

        Ok((user, token_pair))
    }

    /// Refresh tokens
    ///
    /// # Arguments
    /// - `refresh_token`: Refresh token
    ///
    /// # Returns
    /// New token pair
    pub fn refresh_token(&self, refresh_token: String) -> Result<TokenPair> {
        // Verify refresh token
        let claims = self.jwt_service.verify_refresh_token(&refresh_token)?;

        // Generate new token pair
        let token_pair =
            self.jwt_service
                .generate_token_pair(&claims.sub, &claims.username, &claims.email)?;

        Ok(token_pair)
    }

    /// Verify access token
    ///
    /// # Arguments
    /// - `access_token`: Access token
    ///
    /// # Returns
    /// Token claims
    pub fn verify_access_token(&self, access_token: String) -> Result<super::jwt::Claims> {
        self.jwt_service.verify_access_token(&access_token)
    }

    /// Extract user ID from token
    ///
    /// # Arguments
    /// - `access_token`: Access token
    ///
    /// # Returns
    /// User ID
    pub fn extract_user_id(&self, access_token: &str) -> Result<String> {
        self.jwt_service.extract_user_id(access_token)
    }

    /// Get user information
    ///
    /// # Arguments
    /// - `db`: Database client
    /// - `user_id`: User ID
    ///
    /// # Returns
    /// User information
    pub async fn get_user(&self, db: &SurrealClient, user_id: &str) -> Result<User> {
        let record_id: RecordId = user_id.parse()?;
        let user = User::get_by_id(db, &record_id.key().to_string()).await?;
        user.ok_or_else(|| anyhow!("User not found"))
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
    ) -> Result<Option<User>> {
        // Validate new password strength
        if !PasswordService::check_password_strength(&new_password) {
            return Err(anyhow!(
                "Password must be at least 8 characters and contain uppercase, lowercase, and numbers"
            ));
        }

        // Get the user
        let mut user = self.get_user(db, user_id).await?;

        // Verify old password
        let is_valid = self
            .password_service
            .verify_password(&old_password, &user.password_hash)?;

        if !is_valid {
            return Err(anyhow!("Invalid old password"));
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
