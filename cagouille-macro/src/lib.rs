mod component;
mod html;
mod df;
mod utils;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::ToTokens;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

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

#[proc_macro_derive(Differentiable)]
/// Implement differentiability for the struct
pub fn differentiable(input: TokenStream) -> TokenStream {
    let derive: DeriveInput = parse_macro_input!(input);
    df::derive_differentiable(&derive, syn::Ident::new("::cagouille", derive.span())).into()
}

#[proc_macro_derive(Self_Differentiable)]
/// Implement differentiability for the struct
pub fn self_differentiable(input: TokenStream) -> TokenStream {
    let derive: DeriveInput = parse_macro_input!(input);
    df::derive_differentiable(&derive, syn::Ident::new("crate", derive.span())).into()
}