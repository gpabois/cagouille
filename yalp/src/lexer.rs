use syn::token::Token;


#[derive(Clone, Debug)]
pub enum TokenSpan {
    ProcMacroSpan(proc_macro2::Span)
}

impl Into<proc_macro2::Span> for TokenSpan {
    fn into(self) -> proc_macro2::Span {
        match self {
            Self::ProcMacroSpan(span) => span,
            _ => unreachable!("not a macro span")
        }
    }
}

impl From<proc_macro2::Span> for TokenSpan {
    fn from(value: proc_macro2::Span) -> Self {
        Self::ProcMacroSpan(value)
    }
}


#[derive(Clone, Debug)]
pub struct LexerError {
    span: TokenSpan,
    message: String
}

impl From<syn::Error> for LexerError {
    fn from(value: syn::Error) -> Self {
        Self {
            span: value.span().into(),
            message:  value.to_string()
        }
    }
}

impl Into<syn::Error> for LexerError {
    fn into(self) -> syn::Error {
        syn::Error::new(self.span.into(), self.message)
    }
}

pub mod traits {
    use crate::symbol::Symbol;

    use super::{TokenSpan, LexerError};

    pub trait LexerSymbol: Symbol<Self::Type> {
        type Type: Clone + PartialEq;

        fn span(&self) -> TokenSpan;
    }

    pub trait Lexer: Iterator<Item=Result<Self::Symbol, LexerError>> {
        type Symbol: LexerSymbol;
    }
}