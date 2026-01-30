use darling::FromField;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Field, Ident, Visibility};

use crate::{FieldArgs, is_record_id};

pub(crate) fn expand_foreign_methods<'a, F>(
    fields: F,
    vis: &Visibility,
) -> Result<Vec<TokenStream>, syn::Error>
where
    F: IntoIterator<Item = &'a Field>,
{
    fields.into_iter().try_fold(vec![], |mut acc, field| {
        let field_ident = field.ident.as_ref().unwrap();
        let method_ident = Ident::new(&field_ident.to_string().replace("_id", ""), Span::call_site());
        let field_type = &field.ty;
        let field_args = FieldArgs::from_field(field)?;
        if is_record_id(field_type) && field_args.is_foreign_key() {
            if field_ident == "id" {
                return Err(syn::Error::new(Span::call_site(), "Foreign key field must NOT be named `id`"));
            }
            let foreign_key = field_args.foreign_key.unwrap();
            acc.push(quote! {
                #vis async fn #method_ident(&self, client: &::merak_core::SurrealClient) -> surrealdb::Result<Option<#foreign_key>> {
                    client.select(&self.#field_ident).await
                }
            });
        } else if field_args.is_foreign_key() {
            return Err(syn::Error::new(Span::call_site(), "Foreign key field must be of type `RecordId`"));
        }
        Ok::<_, syn::Error>(acc)
    })
}

pub(crate) fn expand_input_struct<'a, F>(
    fields: F,
    vis: &Visibility,
    input_ident: &Ident,
) -> Result<TokenStream, syn::Error>
where
    F: IntoIterator<Item = &'a Field>,
{
    let input_fields = fields
        .into_iter()
        .filter(|field| {
            let field_args = FieldArgs::from_field(field).unwrap();
            !field_args.primary && !field_args.created_at && !field_args.updated_at
        })
        .map(|field| {
            let mut field = field.clone();
            field.attrs.retain(|attr| !attr.path().is_ident("field"));
            field
        });
    Ok(quote! {
        #[derive(::serde::Serialize, ::serde::Deserialize)]
        #vis struct #input_ident {
            #(#input_fields),*
        }
    })
}

#[cfg(feature = "utoipa")]
pub(crate) fn expand_data_impl<'a, F>(
    fields: F,
    vis: &Visibility,
    ident: &Ident,
    data_ident: &Ident,
) -> Result<TokenStream, syn::Error>
where
    F: IntoIterator<Item = &'a Field> + Clone,
{
    let data_fields = fields.clone().into_iter().map(|field| {
        let mut field = field.clone();
        if is_record_id(&field.ty) {
            field.ty = syn::parse_quote!(String);
        }
        field.attrs.retain(|attr| !attr.path().is_ident("field"));
        field
    });
    let covert_data_fields = fields.into_iter().map(|field| {
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
    let data_struct = quote! {
        #[derive(::serde::Serialize, ::serde::Deserialize, ::utoipa::ToSchema)]
        #vis struct #data_ident {
            #(#data_fields),*
        }
    };
    Ok(quote! {
        #data_struct

        #convert_impl
    })
}

pub(crate) fn expand_record_impl<'a, F>(
    fields: F,
    vis: &Visibility,
    record_ident: &Ident,
    input_ident: &Ident,
) -> Result<TokenStream, syn::Error>
where
    F: IntoIterator<Item = &'a Field> + Clone,
{
    let record_fields = fields
        .clone()
        .into_iter()
        .filter(|field| {
            let field_args = FieldArgs::from_field(field).unwrap();
            !field_args.primary
        })
        .map(|field| {
            let field_ident = field.ident.as_ref().unwrap();
            let field_type = &field.ty;
            quote! {
                #field_ident: #field_type
            }
        })
        .collect::<Vec<_>>();
    let covert_data_fields = fields
        .into_iter()
        .filter(|field| {
            let field_args = FieldArgs::from_field(field).unwrap();
            !field_args.primary
        })
        .map(|field| {
            let field_args = FieldArgs::from_field(field).unwrap();
            let field_ident = field.ident.as_ref().unwrap();
            if field_args.created_at {
                quote! {
                    created_at: ::chrono::Utc::now()
                }
            } else if field_args.updated_at {
                quote! {
                    updated_at: ::chrono::Utc::now()
                }
            } else {
                quote! {
                    #field_ident: input.#field_ident
                }
            }
        });
    let quote = quote! {
        #[derive(::serde::Serialize, ::serde::Deserialize)]
        #vis struct #record_ident {
            #(#record_fields),*
        }

        impl From<#input_ident> for #record_ident {
            fn from(input: #input_ident) -> Self {
                #record_ident {
                    #(#covert_data_fields),*
                }
            }
        }

        impl ::merak_core::IntoRecord for #input_ident {
            type Record = #record_ident;
            fn into_record(self) -> Self::Record {
                self.into()
            }
        }
    };
    Ok(quote)
}
