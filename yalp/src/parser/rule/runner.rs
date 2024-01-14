use crate::symbol::traits::{SymbolDefinition, Symbol};

type AnyRuleRunnerFunc<'a, Sym, E> = &'a (dyn Fn(Vec<Sym>) -> Result<<Sym as SymbolDefinition>::Value, E> + Sync + Send);

#[derive(Clone)]
pub struct ParserRuleRunner<'a, SymDef, E>(pub AnyRuleRunnerFunc<'a, SymDef, E>) where SymDef: SymbolDefinition;

impl<'a, Sym, E> ParserRuleRunner<'a, Sym, E> where Sym: Symbol 
{
    pub fn execute(&self, symbols: Vec<Sym>) -> Result<Sym::Value, E> {
        self.0(symbols)
    }
}

#[derive(Clone)]
pub struct ParserRulesRunners<'a, SymDef, E>(Vec<ParserRuleRunner<'a, SymDef, E>>) where SymDef: SymbolDefinition;  

impl<'a, Sym, E> ParserRulesRunners<'a, Sym, E> where Sym: Symbol
{    
    pub fn new() -> Self {
        Self(vec![])
    }
    
    pub fn add<F: Fn(Vec<Sym>) -> Result<Sym::Value, E> + Sync + Send>(&mut self, f: &'a F) -> &mut Self {
        self.0.push(ParserRuleRunner(f));
        self
    }
    
    pub fn get(&self, rule: usize) -> Option<&ParserRuleRunner<'_, Sym, E>> {
        self.0.get(rule)
    }
}
