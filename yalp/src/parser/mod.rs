use proc_macro2::Span;

use crate::lexer::{LexerError, TokenSpan, traits::LexerSymbol};

use self::traits::{TerminalSymbol, ParserSymbol};

pub mod lr;
pub mod rule;

#[derive(Clone, Debug)]
pub enum ParserError<G> where G: ParserSymbol {
    LexerError(LexerError),
    UnexpectedToken{
        got: G::Terminal,
        expecting: Vec<<G::Terminal as LexerSymbol>::Type>,
        state_id: usize
    }
}

impl<G> ToString for ParserError<G> where G: ParserSymbol {
    fn to_string(&self) -> String {
        match self {
            Self::UnexpectedToken { got, expecting, state_id } => {
                format!("unexpecting token {:?}, expecting {:?} [State: {state_id}]", got, expecting)
            },
            _ => todo!()
        }
    }
}

impl<G> ParserError<G> where G: ParserSymbol {
    pub fn unexpected_token(got: G::Terminal, expecting: Vec<<G::Terminal as LexerSymbol>::Type>, state_id: usize) -> Self {
            Self::UnexpectedToken { got, expecting, state_id }
    }

    pub fn span(&self) -> TokenSpan {
        match self {
            Self::UnexpectedToken { got, expecting, state_id } => {
                got.span()
            },
            _ => Span::call_site().into()
        }
    }
} 


impl<G> ParserError<G> where G: ParserSymbol{
    pub fn into_syn_error(self) -> syn::Error {
        let span: proc_macro2::Span = self.span().into();
        syn::Error::new(span.into(), &self.to_string())
    }
}

impl<G> From<LexerError> for ParserError<G> where G: ParserSymbol {
    fn from(value: LexerError) -> Self {
        Self::LexerError(value)
    }
}

impl<G> Into<syn::Error> for ParserError<G> where G: ParserSymbol {
    fn into(self) -> syn::Error {
        todo!()
    }
}

pub type ParserResult<G> = Result<G, ParserError<G>>;

pub mod traits {
    use std::fmt::Debug;

    use crate::{symbol::Symbol, lexer::traits::LexerSymbol};
    use super::{rule::ParserRuleSet, ParserError};

    pub trait Parser<'a> {
        type Symbol: ParserSymbol;

        /// Generate the parser
        fn generate(rules: &'a ParserRuleSet<Self::Symbol>) -> Self;

        /// Parse the stream of tokens
        fn parse<S, L>(&self, stream: L) -> Result<S, ParserError<Self::Symbol>>
        where L: crate::lexer::traits::Lexer<Symbol = <Self::Symbol as ParserSymbol>::Terminal>, 
                Self::Symbol: TryInto<S, Error=ParserError<Self::Symbol>>;
    }

    pub trait ParserSymbol: Symbol<Self::Type> + Clone {
        type Terminal: TerminalSymbol + Into<Self>;
        type Type: ParserSymbolType<<Self::Terminal as LexerSymbol>::Type>;
    }

    pub trait TerminalSymbol: LexerSymbol + Clone + Debug {
    }

    pub trait ParserSymbolType<TerminalType> : Clone + PartialEq + Debug {
        fn is_terminal(&self) -> bool;
        fn expect_terminal_type(&self) -> TerminalType;
    }
}
