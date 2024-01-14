use std::fmt::Debug;

use crate::symbol::traits::SymbolDefinition;

#[derive(Clone, Debug)]
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

impl<SymDef> Debug for LrParserAction<SymDef> where SymDef: SymbolDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LrParserAction").field("r#type", &self.r#type).field("op", &self.op).finish()
    }
}