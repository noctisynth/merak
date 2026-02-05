use merak_macros::Model;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[test]
pub fn default_table_name() {
    #[derive(Model, Serialize, Deserialize)]
    struct AnyModel {}

    let model = AnyModel {};

    assert_eq!(model.table_name(), "any_model");

    assert_eq!(AnyModel::TABLE_NAME, "any_model");
}

#[test]
pub fn specified_table_name() {
    #[derive(Model, Serialize, Deserialize)]
    #[model(table_name = "any_table")]
    struct AnyModel {}

    let model = AnyModel {};

    assert_eq!(model.table_name(), "any_table");
}

#[test]
pub fn foreign_key_method() {
    #[derive(Model, Serialize, Deserialize)]
    #[model(table_name = "other_table")]
    struct OtherModel {
        #[field(primary)]
        id: RecordId,
    }

    #[derive(Model, Serialize, Deserialize)]
    #[model(table_name = "any_table")]
    struct AnyModel {
        #[field(primary)]
        id: RecordId,
        #[field(foreign_key = OtherModel)]
        user: RecordId,
    }

    assert_eq!(AnyModel::TABLE_NAME, "any_table");
    assert_eq!(OtherModel::TABLE_NAME, "other_table");
}
