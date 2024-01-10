use super::{action::{ParserAction, ParserOp}, goto::ParserGoto};

#[derive(Clone)]
pub struct ParserState<G: super::traits::ParserGrammar> {
    actions: Vec<super::action::ParserAction<G>>,
    goto: Vec<super::goto::ParserGoto<G>>
}


impl<G: super::traits::ParserGrammar> ParserState<G> {
    pub fn new() -> Self {
        Self{actions: Default::default(), goto: Default::default()}
    }

    pub fn reduce(&mut self, terminal: G::TerminalType, rule: usize) -> &mut Self {
        self.actions.push(ParserAction {
            r#type: terminal,
            op: ParserOp::Reduce(rule)
        });

        self
    } 

    pub fn accept(&mut self, terminal:G::TerminalType) -> &mut Self {
        self.actions.push(ParserAction {
            r#type: terminal,
            op: ParserOp::Accept
        });

        self
    } 

    pub fn shift(&mut self, terminal: G::TerminalType, next_state: usize) -> &mut Self {
        self.actions.push(ParserAction {
            r#type: terminal,
            op: ParserOp::Shift(next_state)
        });

        self
    } 

    pub fn goto(&mut self, symbol: G::SymbolType, next_state: usize) -> &mut Self {
        self.goto.push(ParserGoto {
            r#type: symbol,
            next_state
        });

        self
    } 

    pub(super) fn get_goto(&self, symbol: &G::SymbolType) -> Option<&ParserGoto<G>> {
        self.goto.iter().find(|a: &&ParserGoto<G>| a.r#type == *symbol)  
    }

    pub(super) fn get_action(&self, terminal: &G::TerminalType) -> Option<&ParserAction<G>> {
        self.actions.iter().find(|a| a.r#type == *terminal)
    }
}