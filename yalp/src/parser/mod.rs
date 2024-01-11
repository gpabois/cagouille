use crate::lexer::LexerError;

pub mod lr;
pub mod rule;

#[derive(Clone, Debug)]
pub enum ParserError<G> {
    LexerError(LexerError),
    Marker(G)
}

impl<G> From<LexerError> for ParserError<G> {
    fn from(value: LexerError) -> Self {
        Self::LexerError(value)
    }
}

pub type ParserResult<G> = Result<G, ParserError<G>>;

pub mod traits {
    use crate::symbol::Symbol;
    use super::{rule::ParserRuleSet, ParserResult};

    pub trait Parser<'a> {
        type Symbol: ParserSymbol;

        /// Generate the parser
        fn generate(rules: &'a ParserRuleSet<Self::Symbol>) -> Self;

        /// Parse the stream of tokens
        fn parse<L: crate::lexer::traits::Lexer<Symbol = <Self::Symbol as ParserSymbol>::Terminal>>(&self, stream: L) -> ParserResult<Self::Symbol>;
    }

    pub trait ParserSymbol: Symbol<Self::Type> + Clone {
        type Terminal: TerminalSymbol + Into<Self>;
        type Type: ParserSymbolType<<Self::Terminal as TerminalSymbol>::Type>;
    }

    pub trait TerminalSymbol: Symbol<Self::Type> + Clone {
        type Type: PartialEq + Clone;
    }

    pub trait ParserSymbolType<TerminalType> : Clone + PartialEq {
        fn is_terminal(&self) -> bool;
        fn expect_terminal_type(&self) -> TerminalType;
    }
}
