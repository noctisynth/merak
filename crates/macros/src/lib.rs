use darling::{FromDeriveInput, FromField};
use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

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
    let ident_name = ident.to_string();

    let model_args = ModelArgs::from_derive_input(&input)?;
    let fields = match &input.data {
        syn::Data::Struct(data) => (&data.fields).into_iter(),
        _ => {
            return Err(syn::Error::new(
                ident.span(),
                "Only named structs are supported",
            ));
        }
    };

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
        } else if field_args.is_foreign_key() {
            return Err(syn::Error::new(ident.span(), "Foreign key field must be of type `RecordId`"));
        }
        Ok::<_, syn::Error>(acc)
    })?;

    let input_ident = Ident::new(&format!("{}Input", ident), ident.span());
    let input_fields = fields
        .clone()
        .filter(|field| {
            let field_args = FieldArgs::from_field(field).unwrap();
            // !field_args.primary && !field_args.created_at && !field_args.updated_at
            !field_args.primary
        })
        .map(|field| {
            let mut field = field.clone();
            field.attrs.retain(|attr| !attr.path().is_ident("field"));
            field
        });

    let data_ident = Ident::new(&format!("{}Data", ident), ident.span());
    let data_fields = fields.clone().map(|field| {
        let mut field = field.clone();
        if is_record_id(&field.ty) {
            field.ty = syn::parse_quote!(String);
        }
        field.attrs.retain(|attr| !attr.path().is_ident("field"));
        field
    });
    let covert_data_fields = fields.clone().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        if is_record_id(&field.ty) {
            quote! {
                #field_ident: model.#field_ident.to_string()
            }
        } else {
            quote! {
                #field_ident: model.#field_ident
            }
        }
    });
    let convert_impl = quote! {
        impl From<#ident> for #data_ident {
            fn from(model: #ident) -> Self {
                #data_ident {
                    #(#covert_data_fields),*
                }
            }
        }
    };

    let table_name = model_args.table_name.unwrap_or(ident_name.to_snake_case());

    #[cfg(feature = "utoipa")]
    let data_struct = quote! {
        #[derive(::serde::Serialize, ::serde::Deserialize, ::utoipa::ToSchema)]
        #vis struct #data_ident {
            #(#data_fields),*
        }
    };
    #[cfg(not(feature = "utoipa"))]
    let data_struct = quote! {
        #[derive(::serde::Serialize, ::serde::Deserialize)]
        #vis struct #data_ident {
            #(#data_fields),*
        }
    };

    let primary_key = fields.clone().find_map(|field| {
        let field_args = FieldArgs::from_field(field).unwrap();
        if field_args.primary {
            Some(field.ident.as_ref().unwrap())
        } else {
            None
        }
    });

    let get_by_primary_key = if let Some(primary_key) = primary_key {
        let primary_ident = Ident::new(&format!("get_by_{}", primary_key), Span::call_site());
        quote! {
            #vis async fn #primary_ident(db: &::merak_core::SurrealClient, id: String) -> surrealdb::Result<Option<Self>> {
                db.select((Self::TABLE_NAME, id)).await
            }
        }
    } else {
        quote! {}
    };

    let operations = if let Some(primary_key) = primary_key {
        let primary_ident = Ident::new(&primary_key.to_string(), Span::call_site());
        quote! {
            #vis async fn save(self, client: &::merak_core::SurrealClient) -> surrealdb::Result<Option<Self>> {
                client.update(self.#primary_ident.clone()).content(self).await
            }

        }
    } else {
        quote! {}
    };

    Ok(quote! {
        use ::merak_core::prelude::*;

        #[derive(::serde::Serialize, ::serde::Deserialize)]
        #vis struct #input_ident {
            #(#input_fields),*
        }

        #data_struct

        #convert_impl

        impl ::merak_core::Model for #ident {
            const TABLE_NAME: &'static str = #table_name;
            type Data = #data_ident;

            fn table_name(&self) -> &'static str { Self::TABLE_NAME }

            fn into_data(self) -> #data_ident { self.into() }
        }

        impl #ident {
            #get_by_primary_key

            #operations

            #(#foreign_methods)*
        }
    }
    .into())
}
