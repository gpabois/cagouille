use std::fmt::Debug;

use crate::symbol::traits::SymbolDefinition;

#[derive(Clone)]
pub struct LrParserGoto<SymDef: SymbolDefinition> {
    pub(super) r#type:      SymDef::Class,
    pub(super) next_state:  usize
}

impl<SymDef> Debug for LrParserGoto<SymDef> where SymDef: SymbolDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LrParserGoto").field("r#type", &self.r#type).field("next_state", &self.next_state).finish()
    }
}