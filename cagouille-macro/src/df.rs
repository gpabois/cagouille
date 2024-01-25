use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput};

use crate::utils::iter_fields;

pub fn generate_df_struct(DeriveInput{ident, data, ..}: &DeriveInput) -> TokenStream {
    let df_ident_name = syn::Ident::new(&format!("{}Df", ident), ident.span());
    
    let df_fields = iter_fields(&data)
    .cloned()
    .map(|mut field| {
        let ty = field.ty;
        field.ty = parse_quote! { Option<#ty> };
        quote!(field)
    })
    .fold(quote!(), |acc, field| {
        quote!(#acc, #field)
    });

    let df_struct = quote!{
        pub struct #df_ident_name {
            #df_fields
        }
    };

    df_struct.into()
}