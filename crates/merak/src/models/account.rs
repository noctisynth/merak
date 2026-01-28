use merak_macros::Model;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Model, Serialize, Deserialize, ToSchema)]
pub struct Account {
    // id: RecordId,
}
