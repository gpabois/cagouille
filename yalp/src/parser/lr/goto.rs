use crate::parser::traits::ParserSymbol;

#[derive(Clone)]
pub struct LrParserGoto<G: ParserSymbol> {
    pub(super) r#type: G::Type,
    pub(super) next_state: usize
}