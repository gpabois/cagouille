use crate::symbol::traits::SymbolDefinition;

#[derive(Clone)]
pub struct LrParserGoto<SymDef: SymbolDefinition> {
    pub(super) r#type:      SymDef::Class,
    pub(super) next_state:  usize
}