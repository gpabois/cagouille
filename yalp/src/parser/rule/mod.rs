pub mod runner;

use std::fmt::Debug;

use crate::symbol::{traits::SymbolDefinition, Sym};

use super::ParserError;


pub struct ParserRule<SymDef: SymbolDefinition> {
    pub id: usize,
    pub lhs: SymDef::Class,
    pub rhs: Vec<SymDef::Class>, 
    pub reducer: runner::ParserRuleRunner<'static, Sym<SymDef>, ParserError>
}

impl<SymDef> Debug for ParserRule<SymDef> 
where SymDef: SymbolDefinition
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParserRule")
            .field("id", &self.id)
            .field("lhs", &self.lhs)
            .field("rhs", &self.rhs)
        .finish()
    }
}

impl<SymDef> Clone for ParserRule<SymDef> where SymDef: SymbolDefinition {
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), lhs: self.lhs.clone(), rhs: self.rhs.clone(), reducer: self.reducer.clone() }
    }
}

impl<SymDef> PartialEq for ParserRule<SymDef> where SymDef: SymbolDefinition + 'static  {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<SymDef> ParserRule<SymDef> where SymDef: SymbolDefinition + 'static {
    pub fn execute(&self, syms: Vec<Sym<SymDef>>) -> Result<SymDef::Value, ParserError> {
        self.reducer.execute(syms)
    }
}

pub struct ParserRuleSet<SymDef>(Vec<ParserRule<SymDef>>) where SymDef: SymbolDefinition + 'static;

impl<SymDef> Clone for ParserRuleSet<SymDef> where SymDef: SymbolDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<SymDef> ParserRuleSet<SymDef> where SymDef: SymbolDefinition {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn root(&self) -> Option<&ParserRule<SymDef>> {
        self.0.first()
    }

    pub fn get(&self, rule_id: usize) -> Option<&ParserRule<SymDef>> {
        self.0.get(rule_id)
    }

    /// Add a new rule
    pub fn add<E: Into<SymDef::Class>, It: IntoIterator<Item=E>, F: Fn(Vec<Sym<SymDef>>) -> Result<SymDef::Value, ParserError> + Sync + Send>(&mut self, lhs: SymDef::Class, rhs: It, reducer: &'static F) -> &mut Self {
        let id = self.0.len();
        self.0.push(
            ParserRule {
                id,
                lhs,
                rhs: rhs.into_iter().map(|t| t.into()).collect(),
                reducer: runner::ParserRuleRunner(reducer)
            }
        );
        self
    }

    pub fn iter_by_lhs<'a>(&'a self, lhs: SymDef::Class) -> impl std::iter::Iterator<Item=&'a ParserRule<SymDef>> {
        self.0.iter().filter(move |r| r.lhs == lhs)
    }
}   
