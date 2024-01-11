use crate::{symbol::Symbol, lexer::traits::Lexer};

use self::{table::LrParserTable, stack::LrParserStack, action::LrParserOp};

use super::{rule::ParserRuleSet, traits::{Parser, ParserSymbol}, ParserResult};

mod goto;
mod action;
mod stack;
mod state;
mod table;

pub struct LrParser<'a, G> where G: ParserSymbol + 'static {
    pub(self) rules: &'a ParserRuleSet<G>,
    pub(self) table: table::LrParserTable<G>
}

impl<'a, G> Parser<'a> for LrParser<'a, G> where G: ParserSymbol + 'static {
    type Symbol = G;

    fn generate(rules: &'a ParserRuleSet<Self::Symbol>) -> Self {
        Self {
            table: LrParserTable::generate(rules),
            rules: rules
        }
    }

    fn parse<L>(&self, stream: L) -> ParserResult<G> 
    where L: Lexer<Symbol = G::Terminal>
    {
        let mut exec = LrParserExecution{
            table: &self.table,
            rules: &self.rules,
            stack: LrParserStack::new(),
            stream
        };

        exec.parse()
    }
}

struct LrParserExecution<'a, G, L> 
where G: ParserSymbol + 'static, L: Lexer<Symbol = G::Terminal>
{
    table: &'a LrParserTable<G>,
    rules: &'a ParserRuleSet<G>,
    stack: LrParserStack<G>,
    stream: L
}

impl<'a, G, L> LrParserExecution<'a, G, L> 
where G: ParserSymbol, L: Lexer<Symbol = G::Terminal>
{
    pub fn parse(&mut self) -> ParserResult<G> {
        for tok in &mut self.stream {
            let tok: G::Terminal = tok?;
            self.stack.syms.push(tok.clone().into());
            let state = self.table.get(*self.stack.states.last().unwrap()).unwrap();
            let action = state.get_action(&tok.get_type()).unwrap();

            match action.op {
                LrParserOp::Shift(next_state) => {
                    // Shift to the next state
                    self.stack.states.push(next_state);
                },
                LrParserOp::Reduce(rule_id) => {
                    // Reduce the stack by the given rule
                    let nb_syms = self.rules.get(rule_id).unwrap().rhs.len();
                    let syms = self.stack.pop(nb_syms).collect::<Vec<_>>();
                    let sym = self.rules.get(rule_id).unwrap().execute( &syms)?;
                    self.stack.syms.push(sym.clone());

                    // fetch the next state to go to.
                    let state = self.table.get(*self.stack.states.last().unwrap()).unwrap();
                    let next_state = state.get_goto(&sym.get_type()).unwrap().next_state;
                    self.stack.states.push(next_state);
                },
                LrParserOp::Accept => break,
            };
        }

        Ok(self.stack.syms.pop().unwrap())
    }
}