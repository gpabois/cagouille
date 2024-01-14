use crate::symbol::traits::SymbolDefinition;

#[derive(Clone)]
pub enum LrParserOp {
    Shift(usize),
    Reduce(usize),
    Accept
}

#[derive(Clone)]
pub struct LrParserAction<SymDef: SymbolDefinition> {
    pub(super) r#type:  SymDef::Class,
    pub(super) op:      LrParserOp
}
