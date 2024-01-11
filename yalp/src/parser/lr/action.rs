use crate::parser::traits::{ParserSymbol, TerminalSymbol};

#[derive(Clone)]
pub enum LrParserOp {
    Shift(usize),
    Reduce(usize),
    Accept
}

#[derive(Clone)]
pub struct LrParserAction<G: ParserSymbol> {
    pub(super) r#type: <G::Terminal as TerminalSymbol>::Type,
    pub(super) op: LrParserOp
}
