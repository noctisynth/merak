use chrono::{DateTime, Utc};
use merak_macros::Model;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Model, Serialize, Deserialize)]
#[model(table_name = "auth_sessions")]
pub struct AuthSession {
    #[field(primary)]
    pub id: RecordId,
    pub user_id: RecordId,
    pub refresh_jti: String,
    pub refresh_expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: DateTime<Utc>,
}
