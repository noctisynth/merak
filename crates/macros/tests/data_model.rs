#[cfg(feature = "utoipa")]
#[test]
pub fn data_model() {
    use merak_macros::Model;
    use serde::{Deserialize, Serialize};
    use surrealdb::types::RecordId;

    #[derive(Model, Serialize, Deserialize)]
    struct AnyModel {
        #[field(primary)]
        id: RecordId,
        user: RecordId,
    }

    let model = AnyModel {
        id: RecordId::new("any_table", "1"),
        user: RecordId::new("other_table", "1"),
    };

    let into_data = model.into_data();
    assert!(!into_data.id.is_empty());
    assert!(!into_data.user.is_empty());
}
