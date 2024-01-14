use std::fmt::Debug;

use crate::symbol::traits::SymbolDefinition;

use super::{action::LrParserAction, goto::LrParserGoto};

#[derive(Clone)]
pub struct LrParserState<SymDef: SymbolDefinition> {
    actions: Vec<LrParserAction<SymDef>>,
    goto: Vec<LrParserGoto<SymDef>>
}

impl<SymDef> Debug for LrParserState<SymDef> where SymDef: SymbolDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LrParserState").field("actions", &self.actions).field("goto", &self.goto).finish()
    }
}


impl<SymDef: SymbolDefinition> LrParserState<SymDef> {
    pub fn new_from_iterators(actions: impl Iterator<Item=LrParserAction<SymDef>>, goto: impl Iterator<Item=LrParserGoto<SymDef>>) -> Self {
        Self {
            actions: actions.collect(),
            goto: goto.collect()
        }
    } 

    pub(super) fn iter_terminals<'a>(&'a self) -> impl Iterator<Item=&'a SymDef::Class> + 'a {
        self.actions.iter().map(|a| &a.r#type)
    }

    pub(super) fn get_goto(&self, symbol: &SymDef::Class) -> Option<&LrParserGoto<SymDef>> {
        self.goto.iter().find(|a| a.r#type == *symbol)  
    }

    pub(super) fn get_action(&self, terminal: &SymDef::Class) -> Option<&LrParserAction<SymDef>> {
        self.actions.iter().find(|a| a.r#type == *terminal)
    }
}