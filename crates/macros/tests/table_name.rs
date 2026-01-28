use merak_macros::Model;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[test]
pub fn default_table_name() {
    #[derive(Model, Serialize, Deserialize)]
    struct AnyModel {}

    assert_eq!(AnyModel::table_name(), "any_model");
}

#[test]
pub fn specified_table_name() {
    #[derive(Model, Serialize, Deserialize)]
    #[model(table_name = "any_table")]
    struct AnyModel {}

    assert_eq!(AnyModel::table_name(), "any_table");
}

#[test]
pub fn foreign_key_method() {
    #[derive(Model, Serialize, Deserialize)]
    #[model(table_name = "any_table")]
    struct OtherModel {
        id: RecordId,
    }

    #[derive(Model, Serialize, Deserialize)]
    #[model(table_name = "any_table")]
    struct AnyModel {
        id: RecordId,
        #[field(foreign_key = OtherModel)]
        user: RecordId,
    }

    assert_eq!(AnyModel::table_name(), "any_table");
    assert_eq!(OtherModel::table_name(), "other_model");
}
