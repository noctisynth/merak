#[cfg(feature = "utoipa")]
#[test]
pub fn data_model() {
    use merak_macros::Model;
    use serde::{Deserialize, Serialize};
    use surrealdb::RecordId;

    #[derive(Model, Serialize, Deserialize)]
    struct AnyModel {
        #[field(primary)]
        id: RecordId,
        user: RecordId,
    }

    let model = AnyModel {
        id: RecordId::from(("any_table", "1")),
        user: RecordId::from(("other_table", "1")),
    };

    let data = AnyModelData {
        id: "any_table:⟨1⟩".to_string(),
        user: "other_table:⟨1⟩".to_string(),
    };

    let into_data = model.into_data();

    assert_eq!(into_data.id, data.id);
    assert_eq!(into_data.user, data.user);
}
