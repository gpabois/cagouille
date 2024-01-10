mod action;
mod goto;
mod state;

pub mod table;
pub mod stack;
pub mod rule;

use table::ParserTable;
use rule::runner::ParserRulesRunners;
use stack::ParserStack;

use crate::symbol::Symbol;

pub mod traits {
    use crate::symbol::Symbol;
    pub trait ParserGrammar {
        type Symbol: Symbol<Type=Self::SymbolType>;
        type SymbolType: ParserSymbolType<TerminalType=Self::TerminalType> + Clone + PartialEq;

        type Terminal: Symbol<Type=Self::TerminalType> + Into<Self::Symbol>;
        type TerminalType: Clone + Into<Self::SymbolType> + PartialEq;
    }

    pub trait ParserSymbolType: Symbol {
        type TerminalType;

        fn is_terminal(&self) -> bool;
        fn try_into_terminal(&self) -> Option<Self::TerminalType>;
    }
}

pub struct Parser<'table, 'rr, G, E, L> 
where G: traits::ParserGrammar + 'static, 
        L: crate::lexer::traits::Lexer<Symbol = G::Terminal>,
        L::Error: Into<E>
{
    lexer: L,
    stack: stack::ParserStack<G>,
    table: &'table table::ParserTable<G>,
    rules_runners: &'rr ParserRulesRunners<'rr, G, E>
}

impl<'table, 'rr, G, E, L> Parser<'table, 'rr, G, E, L> 
where G: traits::ParserGrammar + 'static, 
    L: crate::lexer::traits::Lexer<Symbol = G::Terminal>,
    L::Error: Into<E>
{
    pub fn new(lexer: L, table: &'table ParserTable<G>, rules_runners: &'rr ParserRulesRunners<'rr, G, E>) -> Self {
        Self{lexer, stack: ParserStack::new(), table, rules_runners}
    }

    pub fn parse(&mut self) -> Result<G::Symbol, E> {
        for tok in &mut self.lexer {
            let tok: G::Terminal = tok.map_err(L::Error::into)?;
            self.stack.syms.push(tok.clone().into());
            let state = self.table.get(*self.stack.states.last().unwrap()).unwrap();
            let action = state.get_action(&tok.get_type()).unwrap();

            match action.op {
                action::ParserOp::Shift(next_state) => {
                    // Shift to the next state
                    self.stack.states.push(next_state);
                },
                action::ParserOp::Reduce(rule_id) => {
                    // Reduce the stack by the given rule
                    let sym = self.rules_runners.get(rule_id).unwrap().execute(&mut self.stack)?;
                    self.stack.syms.push(sym.clone());

                    // fetch the next state to go to.
                    let state = self.table.get(*self.stack.states.last().unwrap()).unwrap();
                    let next_state = state.get_goto(&sym.get_type()).unwrap().next_state;
                    self.stack.states.push(next_state);
                },
                action::ParserOp::Accept => break,
            };
        }

        Ok(self.stack.syms.pop().unwrap())
    }
}