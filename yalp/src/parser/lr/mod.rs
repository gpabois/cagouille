use std::fmt::format;

use crate::{symbol::Symbol, lexer::traits::Lexer};
use self::{table::LrParserTable, stack::LrParserStack, action::LrParserOp};
use super::{rule::ParserRuleSet, traits::{Parser, ParserSymbol}, ParserResult, ParserError};

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

    fn parse<S, L>(&self, stream: L) -> Result<S, ParserError<G>> 
    where L: Lexer<Symbol = G::Terminal>, 
            G: TryInto<S, Error=ParserError<G>>
    {
        let mut exec = LrParserExecution{
            table: &self.table,
            rules: &self.rules,
            stack: LrParserStack::new(),
            stream
        };

        let sym = exec.parse()?;

        G::try_into(sym)
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
            
            let state_id = *self.stack.states.last().expect("parser stack is empty");
            let state = self.table.get(state_id).unwrap();
            let action = state.get_action(&tok.get_type()).ok_or_else(||
                ParserError::unexpected_token(
                    tok, 
                    state.iter_terminals().cloned().collect(), 
                    state_id
                )
            )?;

            match action.op {
                LrParserOp::Shift(next_state) => {
                    // Shift to the next state
                    self.stack.states.push(next_state);
                },
                LrParserOp::Reduce(mut rule_id) => {
                    // Reduce the stack by the given rule
                    let rule = self.rules.get(rule_id).expect(&format!("missing rule {rule_id}"));
                    let nb_syms = rule.rhs.len();
                    let syms = self.stack.pop(nb_syms).collect::<Vec<_>>();
                    let sym = rule.execute( &syms)?;
                    self.stack.syms.push(sym.clone());

                    // Fetch the next state to go to.
                    let state = self.table.get(*self.stack.states.last().expect("expecting state to retrieve goto")).unwrap();
                    let next_state = state.get_goto(&sym.get_type())
                        .expect(&format!("unexpected symbol {:?}", sym.get_type()))
                        .next_state;
                    self.stack.states.push(next_state);
                },
                LrParserOp::Accept => break,
            };
        }

        Ok(self.stack.syms.pop().unwrap())
    }
}