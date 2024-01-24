mod component;
mod html;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::ToTokens;
use syn::{parse_macro_input, Data, DeriveInput, FieldsNamed, FieldsUnnamed};
use either::Either;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as component::FuncComponent);

    component::fn_to_component(func).into()
}

#[proc_macro_error]
#[proc_macro]
pub fn render(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as html::VNode);
    TokenStream::from(root.into_token_stream())
}

fn iter_fields(data: &Data) -> Box<dyn Iterator<Item=Either<&FieldsNamed, &FieldsUnnamed>>> {
    if let syn::Data::Struct(s) = data {
        if let syn::Fields::Named(fields) = s.fields {
            
        }
    }

    return Box::new(std::iter::empty())
}

#[proc_macro_derive(Differentiable)]
/// Implement differentiability for the struct
pub fn differentiable(input: TokenStream) -> TokenStream {
    let DeriveInput{ident, data, ..} = parse_macro_input!(input);

    let df_ident_name = syn::Ident::new(&format!("{}Df", ident), ident.span());
    if let syn::Data::Struct(s) = data {
        if let syn::Fields::Named(FieldsNamed { named, .. }) = s.fields {
            
        }
    }



}