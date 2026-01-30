use anyhow::anyhow;
use chrono::{Duration, Utc};
use merak_core::{Model, SurrealClient};
use surrealdb::RecordId;
use uuid::Uuid;

use super::error::{AuthError, AuthResult};
use crate::models::auth::{AuthSession, AuthSessionInput};

#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub session_id: String,
    pub refresh_jti: String,
}

pub struct SessionService;

impl SessionService {
    pub fn new() -> Self {
        Self
    }

    pub async fn create_session(
        &self,
        db: &SurrealClient,
        user_id: &RecordId,
        refresh_exp_seconds: i64,
    ) -> AuthResult<SessionInfo> {
        let now = Utc::now();
        let session_id = Uuid::new_v4().to_string();
        let refresh_jti = Uuid::new_v4().to_string();
        let refresh_expires_at = now + Duration::seconds(refresh_exp_seconds);
        let session_input = AuthSessionInput {
            user_id: user_id.clone(),
            refresh_jti: refresh_jti.clone(),
            refresh_expires_at,
            created_at: now,
            last_used_at: now,
        };
        let created = AuthSession::objects(db)
            .create_with_id(session_id.clone(), session_input)
            .await?;
        created.ok_or_else(|| AuthError::Internal(anyhow!("Failed to create session")))?;
        Ok(SessionInfo {
            session_id,
            refresh_jti,
        })
    }

    pub async fn cleanup_expired_for_user(
        &self,
        db: &SurrealClient,
        user_id: &RecordId,
    ) -> AuthResult<()> {
        let now = Utc::now();
        db.query("DELETE FROM type::table($table) WHERE user_id = $user_id AND refresh_expires_at < $now")
            .bind(("table", AuthSession::TABLE_NAME))
            .bind(("user_id", user_id.clone()))
            .bind(("now", now))
            .await?;
        Ok(())
    }

    pub async fn load_active_session(
        &self,
        db: &SurrealClient,
        session_id: &str,
    ) -> AuthResult<AuthSession> {
        let session = AuthSession::get_by_id(db, session_id).await?;
        let session =
            session.ok_or_else(|| AuthError::SessionInvalid("Session not found".to_string()))?;
        if session.refresh_expires_at < Utc::now() {
            let _ = AuthSession::objects(db).delete(session_id).await?;
            return Err(AuthError::SessionExpired);
        }
        Ok(session)
    }

    pub async fn rotate_refresh_jti(
        &self,
        db: &SurrealClient,
        mut session: AuthSession,
        refresh_exp_seconds: i64,
    ) -> AuthResult<String> {
        let now = Utc::now();
        let new_refresh_jti = Uuid::new_v4().to_string();
        session.refresh_jti = new_refresh_jti.clone();
        session.refresh_expires_at = now + Duration::seconds(refresh_exp_seconds);
        session.last_used_at = now;
        let _ = session.save(db).await?;
        Ok(new_refresh_jti)
    }

    pub async fn delete_session(&self, db: &SurrealClient, session_id: &str) -> AuthResult<()> {
        let _ = AuthSession::objects(db).delete(session_id).await?;
        Ok(())
    }
}

impl Default for SessionService {
    fn default() -> Self {
        Self::new()
    }
}
