pub struct ParserStack<G: super::traits::ParserGrammar> {
    pub(super) states: Vec<usize>,
    pub(super) syms: Vec<G::Symbol>,  
}

impl<G: super::traits::ParserGrammar> ParserStack<G> {
    pub fn new() -> Self {
        Self {
            states: vec![0],
            syms: vec![]
        }
    }
}

impl<G> ParserStack<G> 
where G: super::traits::ParserGrammar + 'static
{
    pub fn pop(&mut self) -> Option<G::Symbol> {
        self.states.pop();
        self.syms.pop()
    }

    pub fn try_pop_into<F>(&mut self) -> Option<Result<F, <G::Symbol as TryInto<F>>::Error>> 
    where G::Symbol: TryInto<F>
    {
        self
        .pop()
        .map(|sym |G::Symbol::try_into(sym))
    }
}