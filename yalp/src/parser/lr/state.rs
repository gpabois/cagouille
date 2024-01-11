use crate::parser::traits::{ParserSymbol, TerminalSymbol};

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

    pub(super) fn get_goto(&self, symbol: &G::Type) -> Option<&LrParserGoto<G>> {
        self.goto.iter().find(|a: &&LrParserGoto<G>| a.r#type == *symbol)  
    }

    pub(super) fn get_action(&self, terminal: &<G::Terminal as TerminalSymbol>::Type) -> Option<&LrParserAction<G>> {
        self.actions.iter().find(|a| a.r#type == *terminal)
    }
}