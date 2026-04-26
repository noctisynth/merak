use surrealdb::types::{RecordId, SqlFormat, ToSql};

pub fn rid_to_string(rid: &RecordId) -> String {
    let mut s = String::new();
    ToSql::fmt_sql(rid, &mut s, SqlFormat::SingleLine);
    s
}
