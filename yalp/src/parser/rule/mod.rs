pub mod runner;

use super::{ParserResult, ParserError, traits::ParserSymbol};



#[derive(Clone)]
pub struct ParserRule<G> where G: ParserSymbol + 'static {
    pub id: usize,
    pub lhs: G::Type,
    pub rhs: Vec<G::Type>, 
    pub reducer: runner::ParserRuleRunner<'static, G, ParserError<G>>
}

impl<G> ParserRule<G> where G: ParserSymbol + 'static {
    pub fn execute(&self, syms: &[G]) -> ParserResult<G> {
        self.reducer.execute(syms)
    }
}

impl<G> PartialEq for ParserRule<G> where G: ParserSymbol + 'static  {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}


#[derive(Clone)]
pub struct ParserRuleSet<G>(Vec<ParserRule<G>>) where G: ParserSymbol + 'static;

impl<G> ParserRuleSet<G> where G: ParserSymbol {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn root(&self) -> Option<&ParserRule<G>> {
        self.0.first()
    }

    pub fn get(&self, rule_id: usize) -> Option<&ParserRule<G>> {
        self.0.get(rule_id)
    }

    /// Add a new rule
    pub fn add<E: Into<G::Type>, It: IntoIterator<Item=E>, F: Fn(&[G]) -> ParserResult<G> + Sync + Send>(&mut self, lhs: G::Type, rhs: It, reducer: &'static F) -> &mut Self {
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

    pub fn iter_by_lhs<'a>(&'a self, lhs: G::Type) -> impl std::iter::Iterator<Item=&'a ParserRule<G>> {
        self.0.iter().filter(move |r| r.lhs == lhs)
    }
}   
