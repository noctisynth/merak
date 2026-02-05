pub(crate) fn is_record_id(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| segment.ident == "RecordId")
            .unwrap_or(false),
        _ => false,
    }
}
