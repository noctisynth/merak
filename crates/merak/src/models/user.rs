use chrono::{DateTime, Utc};
use merak_macros::Model;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Model, Serialize, Deserialize)]
#[model(table_name = "users")]
pub struct User {
    #[field(primary)]
    pub id: RecordId,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
