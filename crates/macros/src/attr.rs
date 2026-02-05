use darling::{FromDeriveInput, FromField};

#[derive(Default, FromDeriveInput)]
#[darling(default, attributes(model))]
pub struct ModelArgs {
    pub table_name: Option<String>,
}

#[derive(Default, FromField)]
#[darling(default, attributes(field))]
pub struct FieldArgs {
    pub primary: bool,
    pub foreign_key: Option<syn::Ident>,
    pub created_at: bool,
    pub updated_at: bool,
}

impl FieldArgs {
    pub fn is_foreign_key(&self) -> bool {
        self.foreign_key.is_some()
    }
}
