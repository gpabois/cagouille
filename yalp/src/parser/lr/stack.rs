use crate::symbol::{traits::SymbolDefinition, Sym};

pub(super) struct LrParserStack<SymDef: SymbolDefinition> {
    pub(super) states: Vec<usize>,
    pub(super) syms: Vec<Sym<SymDef>>,  
}

impl<SymDef: SymbolDefinition> LrParserStack<SymDef> {
    pub fn new() -> Self {
        Self {
            states: vec![0],
            syms: vec![]
        }
    }

    pub fn pop<'a>(&'a mut self, count: usize) -> impl Iterator<Item=Sym<SymDef>> + 'a {
        let split = self.syms.len() - count;
        self.states.drain(split..);
        self.syms.drain(split..).into_iter()
    }
}
