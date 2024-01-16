use crate::span::Span;
use crate::lexer::LexerError;
use crate::symbol::Sym;
use crate::symbol::traits::{SymbolDefinition, Symbol};

pub mod lr;
pub mod rule;

#[derive(Clone, Debug)]
pub struct ParserError {
    pub span: Span,
    pub message: String
}

impl ToString for ParserError {
    fn to_string(&self) -> String {
        self.message.to_owned()
    }
}

impl ParserError {
    pub fn unexpected_token<SymDef: SymbolDefinition>(got: Sym<SymDef>, expecting: Vec<SymDef::Class>) -> Self {
        Self {
            span: got.span().clone(),
            message: format!("unexpecting token {:?}, expecting {:?}", got, expecting)
        }
    }

    pub fn wrong_value<SymDef: SymbolDefinition>(got: Sym<SymDef>) -> Self {
        Self {
            span: got.span().clone(),
            message: format!("unexpecting value {:?}", got.value)
        }
    }


    pub fn span(&self) -> Span {
        self.span.clone()
    }
} 

impl ParserError {
    pub fn into_syn_error(self) -> syn::Error {
        let span: proc_macro2::Span = self.span().into();
        syn::Error::new(span.into(), &self.to_string())
    }
}

impl From<LexerError> for ParserError {
    fn from(value: LexerError) -> Self {
        Self {
            span: value.span,
            message: value.message
        }
    }
}


pub mod traits {
    use std::fmt::Debug;

    use crate::symbol::traits::SymbolDefinition;

    use super::{rule::ParserRuleSet, ParserError};

    pub trait Parser<'a> {
        type Symbol: SymbolDefinition;
        type SymbolDefinition: SymbolDefinition;

        /// Generate the parser
        fn generate(rules: &'a ParserRuleSet<Self::SymbolDefinition>) -> Self;

        /// Parse the stream of tokens
        fn parse<V, Token, TokenStream, TokenError>(&self, stream: TokenStream) -> Result<V, ParserError>
        where TokenStream: Iterator<Item = Result<Token, TokenError>>, 
                ParserError: From<TokenError>,
                Self::Symbol: From<Token>,
                <Self::Symbol as SymbolDefinition>::Value: TryInto<V>;
    }

    pub trait ParserSymbolClass: Clone + PartialEq + Debug {
        fn is_terminal(&self) -> bool;
    }
}
