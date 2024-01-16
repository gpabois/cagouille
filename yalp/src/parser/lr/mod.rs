
use crate::symbol::{traits::{SymbolDefinition, Symbol}, Sym};

use self::{table::LrParserTable, stack::LrParserStack, action::LrParserOp};
use super::{rule::ParserRuleSet, traits::{Parser, ParserSymbolClass}, ParserError};

mod goto;
mod action;
mod stack;
mod state;
mod table;

pub struct LrParser<'a, SymDef> where SymDef: SymbolDefinition, SymDef::Class: ParserSymbolClass {
    pub(self) rules: &'a ParserRuleSet<SymDef>,
    pub table: table::LrParserTable<SymDef>
}

impl<'a, SymDef> Parser<'a> for LrParser<'a, SymDef> where SymDef: SymbolDefinition, SymDef::Class: ParserSymbolClass {
    type Symbol = Sym<SymDef>;
    type SymbolDefinition = SymDef;

    fn generate(rules: &'a ParserRuleSet<Self::SymbolDefinition>) -> Self {
        Self {
            table: LrParserTable::generate(rules),
            rules: rules
        }
    }

    fn parse<V, Token, TokenStream, TokenError>(&self, stream: TokenStream) -> Result<V, ParserError>
    where TokenStream: Iterator<Item = Result<Token, TokenError>>, 
            ParserError: From<TokenError>,
            Self::Symbol: From<Token>,
            <Self::Symbol as SymbolDefinition>::Value: TryInto<V>
    {
        let mut exec = LrParserExecution{
            table: &self.table,
            rules: &self.rules,
            stack: LrParserStack::new(),
            stream
        };

        let sym = exec.parse()?;
        sym.into_value()
    }
}

struct LrParserExecution<'a, SymDef, Token, TokenStream, TokenError> 
where   SymDef: SymbolDefinition, 
        SymDef::Class: ParserSymbolClass,
        Sym<SymDef>: From<Token>,
        ParserError: From<TokenError>,
        TokenStream: Iterator<Item = Result<Token, TokenError>>
{
    table: &'a LrParserTable<SymDef>,
    rules: &'a ParserRuleSet<SymDef>,
    stack: LrParserStack<SymDef>,
    stream: TokenStream
}

impl<'a, SymDef, Token, TokenStream, TokenError> LrParserExecution<'a, SymDef, Token, TokenStream, TokenError> 
where   SymDef: SymbolDefinition, 
        SymDef::Class: ParserSymbolClass,
        Sym<SymDef>: From<Token>,
        ParserError: From<TokenError>,
        TokenStream: Iterator<Item = Result<Token, TokenError>>
{
    pub fn parse(&mut self) -> Result<Sym<SymDef>, ParserError> {
        for tok in &mut self.stream {
            let tok: Sym<SymDef> = tok.map_err(ParserError::from)?.into();
            let cursor = tok.span();
            self.stack.syms.push(tok.clone().into());
            
            let state_id = *self.stack.states.last().expect("parser stack is empty");
            let state = self.table.get(state_id).unwrap();
            let action = state.get_action(&tok.get_type())
            .ok_or_else(||
                ParserError::unexpected_token(
                    tok, 
                    state.iter_terminals().cloned().collect()
                )
            )?;

            match action.op {
                LrParserOp::Shift(next_state) => {
                    // Shift to the next state
                    self.stack.states.push(next_state);
                },
                LrParserOp::Reduce(rule_id) => {
                    // Reduce the stack by the given rule
                    let rule = self.rules.get(rule_id).expect(&format!("missing rule {rule_id}"));
                    let nb_syms = rule.rhs.len();
                    let syms = self.stack.pop(nb_syms).collect::<Vec<_>>();
                    let sym_value = rule.execute(syms)?;
                    
                    let sym = Sym {
                        span: cursor,
                        class: rule.lhs.clone(),
                        value: sym_value
                    };

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