use crate::parser::traits::ParserSymbol;

type AnyRuleRunnerFunc<'a, G, E> = &'a (dyn Fn(&[G]) -> Result<G, E> + Sync + Send);

#[derive(Clone)]
pub struct ParserRuleRunner<'a, G, E>(pub AnyRuleRunnerFunc<'a, G, E>) where G: ParserSymbol;

impl<'a, G, E> ParserRuleRunner<'a, G, E> where G: ParserSymbol 
{
    pub fn execute(&self, symbols: &[G]) -> Result<G, E> {
        self.0(symbols)
    }
}

#[derive(Clone)]
pub struct ParserRulesRunners<'a, G, E>(Vec<ParserRuleRunner<'a, G, E>>) where G: ParserSymbol;  

impl<'a, G, E> ParserRulesRunners<'a, G, E> where G: ParserSymbol
{    
    pub fn new() -> Self {
        Self(vec![])
    }
    
    pub fn add<F: Fn(&[G]) -> Result<G, E> + Sync + Send>(&mut self, f: &'a F) -> &mut Self {
        self.0.push(ParserRuleRunner(f));
        self
    }
    
    pub fn get(&self, rule: usize) -> Option<&ParserRuleRunner<'_, G, E>> {
        self.0.get(rule)
    }
}
