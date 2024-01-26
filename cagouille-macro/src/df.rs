use proc_macro::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput, Path, TypePath};

use crate::utils::iter_fields;

/// Generate differentiable implementation
pub fn generate_differentiable_impl(DeriveInput{ident, data, generics, ..}: &DeriveInput, crate_name: syn::Ident) -> TokenStream {
    let df_stmts = iter_fields(&data)
    .cloned()
    .map(|mut field| {
        let ty = field.ty;
        field.ty = parse_quote! { Option<#ty> };
        field
    })
    .enumerate()
    .fold(quote!(), |acc, (id, field)| {
        match field.ident {
            Some(field_name) => {
                quote!{
                    #acc

                    #field_name: if src.#field_name != dest.#field_name {
                        Some(dest.#field_name.clone())
                    } else {
                        None
                    },
                }
            },
            None => {
                quote!{
                    #acc, 
                    if src.#id != dest.#id {
                        Some(other.#id)
                    } else {
                        None
                    }
                }
            }
        }
    });

    let df_ident_name = syn::Ident::new(&format!("{}Df", ident), ident.span());

    let (impl_gen, type_gen, where_clause) = generics.split_for_impl();

    let trait_path: TypePath = parse_quote!{#crate_name ::df::traits::Differentiable};

    quote! {
        impl #impl_gen #trait_path for #ident #type_gen #where_clause {
            type Df = #df_ident_name;

            fn df<'a, 'fut>(src: &'a Self, dest: &'a Self) -> Self::Df {
                Self::Df { #df_stmts }
            }
        }
    }.into()
}

/// Generate a Df struct
pub fn generate_df_struct(DeriveInput{ident, data, ..}: &DeriveInput) -> TokenStream {
    let df_ident_name = syn::Ident::new(&format!("{}Df", ident), ident.span());
    
    let df_fields = iter_fields(&data)
    .cloned()
    .map(|mut field| {
        let ty = field.ty;
        field.ty = parse_quote! { Option<#ty> };
        quote!(#field)
    })
    .fold(quote!(), |acc, field| {
        quote!(#acc #field,)
    });

    let df_struct = quote!{
        pub struct #df_ident_name {
            #df_fields
        }
    };

    df_struct.into()
}

pub fn derive_differentiable(derive: &DeriveInput, crate_name: syn::Ident) -> TokenStream {
    let df_struct = generate_df_struct(derive);
    let diff_impl = generate_differentiable_impl(derive, crate_name);

    quote! {
        #df_struct
        #diff_impl
    }.into()
}