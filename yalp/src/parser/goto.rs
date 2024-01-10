#[derive(Clone)]
pub(super) struct ParserGoto<G: super::traits::ParserGrammar> {
    pub(super) r#type: G::SymbolType,
    pub(super) next_state: usize
}