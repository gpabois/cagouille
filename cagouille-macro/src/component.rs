use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Parse, Item, ReturnType, FnArg, Type, Ident, ItemFn};

pub struct FuncComponent {
    name: Ident,
    props_arg: FnArg,
    props_type: Box<Type>,
    return_type: Box<Type>,
    func: ItemFn,
}

impl Parse for FuncComponent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let parsed: Item = input.parse()?;

        let func = match parsed {
            Item::Fn(func) => func,
            item => {
                return Err(syn::Error::new_spanned(
                    item,
                    "`component` attribute can only be applied to functions",
                ))
            }
        };

        if func.sig.asyncness.is_none() {
            return Err(syn::Error::new_spanned(
                func.sig.asyncness,
                "function components must be async",
            ));
        }

        let return_type = match &func.sig.output {
            ReturnType::Default => {
                return Err(syn::Error::new_spanned(
                    func.sig,
                    "function components must return `cagouille::Node`, or `cagouille::NodeResult`",
                ))
            }
            ReturnType::Type(_, ty) => ty.clone(),
        };

        let props_arg = func.sig.inputs.iter().next().ok_or_else(|| {
            syn::Error::new_spanned(
                &func.sig.inputs,
                "function components must include properties as the first argument",
            )
        })?.clone();

        let props_type = match &props_arg {
            FnArg::Typed(arg) => match &*arg.ty {
                Type::Reference(ref_ty) => ref_ty.elem.clone(),
                _ => todo!(),
            },
            arg => return Err(syn::Error::new_spanned(
                arg,
                "function components must include properties as the first argument",
            ))
        };

        Ok(Self{
            name: func.sig.ident.clone(),
            props_arg,
            props_type,
            return_type,
            func
        })
    }
}

impl FuncComponent {
    /// Generate the struct definition
    pub fn generate_struct_definition(&self) -> TokenStream {
        let vis = &self.func.vis;
        let name = &self.name;
        let generics = &self.func.sig.generics;
        let (_, _, where_clause) = self.func.sig.generics.split_for_impl();


        quote!{
            #vis struct #name #generics #where_clause {
                inner: ::cagouille::component::function::FunctionComponent::<Self>
            }
        }
    }

    /// Generate runner's statements 
    pub fn generate_inner_run_fn(&self) -> TokenStream {
        let name = &self.name;
        let block = &self.func.block;
        let attrs = &self.func.attrs;
        let fn_token = self.func.sig.fn_token;
        let asyncness = self.func.sig.asyncness;
        let return_type = &self.return_type;
        let (impl_generics, _ty_generics, where_clause) = self.func.sig.generics.split_for_impl();
        let props_arg = &self.props_arg;

        quote!{
            #(#attrs)*
            #asyncness #fn_token run #impl_generics(_state: &mut ::cagouille::component::function::FcState<#name>, #props_arg) -> #return_type
            #where_clause
            #block
        }
    }

    /// Generate runner
    pub fn generate_runner_trait_implementation(&self) -> TokenStream {
        let name = &self.name;
        let inner_fn = self.generate_inner_run_fn();

        quote! {
            impl ::cagouille::component::function::traits::FunctionComponentRunner for #name {
                type Component = Self;

                fn run<'state, 'props, 'fut>(state: &'state mut ::cagouille::component::function::FcState<#name>, props: &'props <Self::Component as ::cagouille::component::traits::Component>::Properties) 
                -> ::cagouille::futures::future::BoxFuture<'fut, ::cagouille::vdom::VNodeResult> 
                where 'state: 'fut, 'props: 'fut
                {
                    #inner_fn
                    
                    Box::pin(async {
                        run(state, props).await.into()
                    })
                }
            }
        }
    }

    /// Generate component implementation
    pub fn generate_component_trait_implementation(&self) -> TokenStream {
        let name = &self.name;
        let props_type = &self.props_type;

        quote! {
            impl ::cagouille::component::traits::Component for #name {
                type Message = ();
                type Properties = #props_type;

                fn new(state: ::cagouille::component::State<Self>) -> Self {
                    Self {
                        inner: ::cagouille::component::function::FunctionComponent::<Self>::new(state),
                    }
                }

                /// Process an event.
                fn process_event<'state, 'fut>(
                    &mut self, 
                    state: &'state ::cagouille::component::State<Self>, 
                    event: ::cagouille::component::ComponentEvent<'_, Self>
                ) -> ::cagouille::futures::future::BoxFuture<'fut, Result<(), ::cagouille::error::Error>>
                where 'state: 'fut
                {
                    Box::pin(self.inner.process_event(state, event))
                }

                /// Render the component.
                fn render<'s, 'fut>(
                    &self, 
                    state: &'s ::cagouille::component::State<Self>
                ) -> ::cagouille::futures::future::BoxFuture<'fut, ::cagouille::vdom::VNodeResult> 
                where 's: 'fut
                {
                    Box::pin(self.inner.render(&state.props))
                }
            }
        }
    }
}

/// Transform the function into a Component
pub fn fn_to_component(func: FuncComponent) -> TokenStream {
    let struct_def = func.generate_struct_definition();
    let comp_impl = func.generate_component_trait_implementation();
    let fc_runner_impl = func.generate_runner_trait_implementation();

    quote! {
        #struct_def
        #comp_impl
        #fc_runner_impl
    }
}