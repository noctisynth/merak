use darling::{FromDeriveInput, FromField};
use heck::ToSnakeCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, parse_macro_input};

use crate::attr::{FieldArgs, ModelArgs};

mod attr;

fn is_record_id(ty: &syn::Type) -> bool {
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

#[proc_macro_derive(Model, attributes(model, field))]
pub fn merak_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    expand_model(input).unwrap_or_else(|err| err.to_compile_error().into())
}

fn expand_model(input: DeriveInput) -> syn::Result<TokenStream> {
    let vis = &input.vis;
    let ident = &input.ident;
    let fields = match &input.data {
        syn::Data::Struct(data) => (&data.fields).into_iter(),
        _ => {
            return Err(syn::Error::new(
                ident.span(),
                "Only named structs are supported",
            ));
        }
    };

    let ident_name = ident.to_string();
    let foreign_methods = fields.clone().try_fold(vec![], |mut acc, field| {
        let field_ident = field.ident.as_ref().unwrap();
        let method_ident = Ident::new(&field_ident.to_string().replace("_id", ""), ident.span());
        let field_type = &field.ty;
        let field_args = FieldArgs::from_field(field)?;
        if is_record_id(field_type) && field_args.is_foreign_key() {
            if field_ident == "id" {
                return Err(syn::Error::new(ident.span(), "Foreign key field must NOT be named `id`"));
            }
            let foreign_key = field_args.foreign_key.unwrap();
            acc.push(quote! {
                #vis async fn #method_ident(&self, client: &::merak_core::SurrealClient) -> surrealdb::Result<Option<#foreign_key>> {
                    client.select(&self.#field_ident).await
                }
            });
        };
        Ok::<_, syn::Error>(acc)
    })?;

    let model_args = ModelArgs::from_derive_input(&input)?;

    let table_name = model_args.table_name.unwrap_or(ident_name.to_snake_case());

    Ok(quote! {
        impl #ident {
            #vis fn table_name() -> &'static str { #table_name }

            #vis async fn create(db: &::merak_core::SurrealClient, data: Self) -> surrealdb::Result<Option<Self>> {
                db.create(Self::table_name()).content(data).await
            }

            #vis async fn create_with_id(db: &::merak_core::SurrealClient, id: String, data: Self) -> surrealdb::Result<Option<Self>> {
                db.create((Self::table_name(), id)).content(data).await
            }

            #(#foreign_methods)*
        }
    }
    .into())
}
