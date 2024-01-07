mod component;
mod html;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::ToTokens;
use syn::parse_macro_input;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as component::FuncComponent);

    component::fn_to_component(func).into()
}

#[proc_macro_error]
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as html::VNode);
    TokenStream::from(root.into_token_stream())
}
