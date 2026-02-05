use anyhow::{Result, anyhow};
use argon2::{
    Algorithm, Argon2, Params, Version,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

/// Argon2 configuration
#[derive(Debug, Clone)]
pub struct PasswordConfig {
    /// Memory cost (KB)
    pub m_cost: u32,
    /// Time cost
    pub t_cost: u32,
    /// Parallelism cost
    pub p_cost: u32,
    /// Output length
    pub output_len: usize,
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self {
            m_cost: 64 * 1024, // 64 MB
            t_cost: 3,
            p_cost: 4,
            output_len: 32,
        }
    }
}

impl PasswordConfig {
    /// Create Argon2 instance
    pub fn to_argon2(&self) -> Argon2<'static> {
        let params = Params::new(self.m_cost, self.t_cost, self.p_cost, Some(self.output_len))
            .expect("Invalid Argon2 parameters");
        Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
    }
}

/// Password hashing and verification service
pub struct PasswordService {
    argon2: Argon2<'static>,
}

impl PasswordService {
    /// Create password service with default configuration
    pub fn new() -> Self {
        Self {
            argon2: PasswordConfig::default().to_argon2(),
        }
    }

    /// Create password service with custom configuration
    pub fn with_config(config: PasswordConfig) -> Self {
        Self {
            argon2: config.to_argon2(),
        }
    }

    /// Hash a password
    ///
    /// # Arguments
    /// - `password`: The plaintext password
    ///
    /// # Returns
    /// The hashed password string
    pub fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = &self.argon2;

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow!("Failed to hash password: {}", e))?;

        Ok(password_hash.to_string())
    }

    /// Verify a password
    ///
    /// # Arguments
    /// - `password`: The plaintext password to verify
    /// - `hash`: The hashed password string
    ///
    /// # Returns
    /// Whether the password matches the hash
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash =
            PasswordHash::new(hash).map_err(|e| anyhow!("Failed to parse password hash: {}", e))?;

        match self
            .argon2
            .verify_password(password.as_bytes(), &parsed_hash)
        {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(anyhow!("Password verification failed: {}", e)),
        }
    }

    /// Check password strength
    ///
    /// # Arguments
    /// - `password`: The password to check
    ///
    /// # Returns
    /// Whether the password meets minimum strength requirements
    pub fn check_password_strength(password: &str) -> bool {
        // Minimum 8 characters
        if password.len() < 8 {
            return false;
        }

        // At least one lowercase letter
        if !password.chars().any(|c| c.is_lowercase()) {
            return false;
        }

        // At least one uppercase letter
        if !password.chars().any(|c| c.is_uppercase()) {
            return false;
        }

        // At least one digit
        if !password.chars().any(|c| c.is_ascii_digit()) {
            return false;
        }

        true
    }
}

impl Default for PasswordService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let service = PasswordService::new();
        let password = "TestPassword123!";

        let hash = service.hash_password(password).unwrap();
        assert!(!hash.is_empty());
        assert_ne!(hash, password);

        let is_valid = service.verify_password(password, &hash).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_verify_wrong_password() {
        let service = PasswordService::new();
        let password = "TestPassword123!";
        let wrong_password = "WrongPassword123!";

        let hash = service.hash_password(password).unwrap();
        let is_valid = service.verify_password(wrong_password, &hash).unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn test_hash_is_different_each_time() {
        let service = PasswordService::new();
        let password = "TestPassword123!";

        let hash1 = service.hash_password(password).unwrap();
        let hash2 = service.hash_password(password).unwrap();

        // Hashes should be different due to random salt
        assert_ne!(hash1, hash2);

        // But both should verify correctly
        assert!(service.verify_password(password, &hash1).unwrap());
        assert!(service.verify_password(password, &hash2).unwrap());
    }

    #[test]
    fn test_check_password_strength() {
        // Valid passwords
        assert!(PasswordService::check_password_strength("Test1234"));
        assert!(PasswordService::check_password_strength("MySecurePass123"));

        // Invalid passwords - too short
        assert!(!PasswordService::check_password_strength("Test1"));

        // Invalid passwords - no uppercase letters
        assert!(!PasswordService::check_password_strength("test1234"));

        // Invalid passwords - no lowercase letters
        assert!(!PasswordService::check_password_strength("TEST1234"));

        // Invalid passwords - no digits
        assert!(!PasswordService::check_password_strength("TestPass"));
    }
}
