use super::super::{traits::ParserGrammar, stack::ParserStack};

pub type RuleRunnerResult<G, E> = Result<<G as ParserGrammar>::Symbol, E>;

type AnyRuleRunnerFunc<'a, G, E> = &'a (dyn Fn(&mut ParserStack<G>) -> RuleRunnerResult<G, E> + Sync + Send);

#[derive(Clone)]
pub struct ParserRuleRunner<'a, G, E>(AnyRuleRunnerFunc<'a, G, E>) where G: ParserGrammar;

impl<'a, G, E> ParserRuleRunner<'a, G, E> where G: ParserGrammar 
{
    pub fn execute(&self, stack: &mut ParserStack<G>) -> RuleRunnerResult<G, E> {
        self.0(stack)
    }
}

#[derive(Clone)]
pub struct ParserRulesRunners<'a, G, E>(Vec<ParserRuleRunner<'a, G, E>>) where G: ParserGrammar;  

impl<'a, G, E> ParserRulesRunners<'a, G, E> where G: ParserGrammar
{    
    pub fn new() -> Self {
        Self(vec![])
    }
    
    pub fn add<F: Fn(&mut ParserStack<G>) -> RuleRunnerResult<G, E> + Sync + Send>(&mut self, f: &'a F) -> &mut Self {
        self.0.push(ParserRuleRunner(f));
        self
    }
    
    pub fn get(&self, rule: usize) -> Option<&ParserRuleRunner<'_, G, E>> {
        self.0.get(rule)
    }
}
