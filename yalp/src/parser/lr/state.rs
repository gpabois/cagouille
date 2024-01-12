use crate::{parser::traits::{ParserSymbol, TerminalSymbol}, lexer::traits::LexerSymbol};

use super::{action::LrParserAction, goto::LrParserGoto};

#[derive(Clone)]
pub struct LrParserState<G: ParserSymbol> {
    actions: Vec<LrParserAction<G>>,
    goto: Vec<LrParserGoto<G>>
}


impl<G: ParserSymbol> LrParserState<G> {
    pub fn new_from_iterators(actions: impl Iterator<Item=LrParserAction<G>>, goto: impl Iterator<Item=LrParserGoto<G>>) -> Self {
        Self {
            actions: actions.collect(),
            goto: goto.collect()
        }
    } 

    pub(super) fn iter_terminals<'a>(&'a self) -> impl Iterator<Item=&'a <G::Terminal as LexerSymbol>::Type> + 'a {
        self.actions.iter().map(|a| &a.r#type)
    }

    pub(super) fn get_goto(&self, symbol: &G::Type) -> Option<&LrParserGoto<G>> {
        self.goto.iter().find(|a: &&LrParserGoto<G>| a.r#type == *symbol)  
    }

    pub(super) fn get_action(&self, terminal: &<G::Terminal as LexerSymbol>::Type) -> Option<&LrParserAction<G>> {
        self.actions.iter().find(|a| a.r#type == *terminal)
    }
}