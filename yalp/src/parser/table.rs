use super::state::ParserState;

#[derive(Clone)]
pub struct ParserTable<G: super::traits::ParserGrammar>(Vec<ParserState<G>>);

impl<G: super::traits::ParserGrammar> ParserTable<G> {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn add<F: FnOnce() -> ParserState<G>>(&mut self, f: F) -> &mut Self {
        self.0.push(f());
        self
    }
    
    pub fn get(&self, state: usize) -> Option<&ParserState<G>> {
        self.0.get(state)
    }
}
