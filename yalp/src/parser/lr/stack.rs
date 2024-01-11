use crate::parser::traits::ParserSymbol;

pub(super) struct LrParserStack<G: ParserSymbol> {
    pub(super) states: Vec<usize>,
    pub(super) syms: Vec<G>,  
}

impl<G: ParserSymbol> LrParserStack<G> {
    pub fn new() -> Self {
        Self {
            states: vec![0],
            syms: vec![]
        }
    }

    pub fn pop<'a>(&'a mut self, count: usize) -> impl Iterator<Item=G> + 'a {
        let split = self.syms.len() - count;
        self.states.drain(split..);
        self.syms.drain(split..).into_iter()
    }
}
