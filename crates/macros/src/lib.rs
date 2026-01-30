use darling::{FromDeriveInput, FromField};
use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{DeriveInput, Ident, parse_macro_input};

#[cfg(feature = "utoipa")]
use crate::expand::expand_data_impl;
use crate::{
    attr::{FieldArgs, ModelArgs},
    expand::{expand_foreign_methods, expand_input_struct, expand_record_impl},
    utils::is_record_id,
};

mod attr;
mod expand;
mod utils;

#[proc_macro_derive(Model, attributes(model, field))]
pub fn merak_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    expand_model(input).unwrap_or_else(|err| err.to_compile_error().into())
}

fn expand_model(input: DeriveInput) -> syn::Result<TokenStream> {
    let vis = &input.vis;
    let ident = &input.ident;
    let ident_name = ident.to_string();

    let model_args = ModelArgs::from_derive_input(&input)?;
    let fields = match &input.data {
        syn::Data::Struct(data) => &data.fields,
        _ => {
            return Err(syn::Error::new(
                ident.span(),
                "Only named structs are supported",
            ));
        }
    };

    let foreign_methods = expand_foreign_methods(fields, vis)?;

    let input_ident = Ident::new(&format!("{}Input", ident), Span::call_site());
    let input_struct = expand_input_struct(fields, vis, &input_ident)?;

    let record_ident = Ident::new(&format!("{}Record", ident), Span::call_site());
    let record_impl = expand_record_impl(fields, vis, &record_ident, &input_ident)?;

    #[cfg(feature = "utoipa")]
    let (data_ident, data_impl) = {
        let data_ident = Ident::new(&format!("{}Data", ident), Span::call_site());
        let data_impl = expand_data_impl(fields, vis, ident, &data_ident)?;
        (data_ident, data_impl)
    };
    #[cfg(not(feature = "utoipa"))]
    let data_impl = quote! {};

    let table_name = model_args.table_name.unwrap_or(ident_name.to_snake_case());

    let primary_key = fields.iter().find_map(|field| {
        let field_args = FieldArgs::from_field(field).unwrap();
        if field_args.primary {
            Some(field.ident.as_ref().unwrap())
        } else {
            None
        }
    });

    let operations = if let Some(primary_key) = primary_key {
        let primary_ident = Ident::new(&primary_key.to_string(), Span::call_site());
        let get_by_primary_ident =
            Ident::new(&format!("get_by_{}", primary_key), Span::call_site());
        quote! {
            #vis async fn #get_by_primary_ident(db: &::merak_core::SurrealClient, id: &str) -> surrealdb::Result<Option<Self>> {
                db.select((Self::TABLE_NAME, id)).await
            }

            #vis async fn save(self, client: &::merak_core::SurrealClient) -> surrealdb::Result<Option<Self>> {
                client.update(self.#primary_ident.clone()).content(self).await
            }

            #vis async fn delete(self, client: &::merak_core::SurrealClient) -> surrealdb::Result<Option<Self>> {
                client.delete(self.#primary_ident.clone()).await
            }
        }
    } else {
        quote! {}
    };

    #[cfg(feature = "utoipa")]
    let trait_impl = quote! {
        impl ::merak_core::Model for #ident {
            const TABLE_NAME: &'static str = #table_name;
            type Data = #data_ident;
            type Input = #input_ident;

            fn table_name(&self) -> &'static str { Self::TABLE_NAME }
            fn into_data(self) -> #data_ident { self.into() }
        }
    };
    #[cfg(not(feature = "utoipa"))]
    let trait_impl = quote! {
        impl ::merak_core::Model for #ident {
            const TABLE_NAME: &'static str = #table_name;
            type Input = #input_ident;

            fn table_name(&self) -> &'static str { Self::TABLE_NAME }
        }
    };

    Ok(quote! {
        use ::merak_core::prelude::*;

        #input_struct

        #record_impl

        #data_impl

        #trait_impl

        impl #ident {
            #operations

            #(#foreign_methods)*
        }
    }
    .into())
}
