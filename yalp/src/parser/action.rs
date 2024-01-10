#[derive(Clone)]
pub(super) enum ParserOp {
    Shift(usize),
    Reduce(usize),
    Accept
}

#[derive(Clone)]
pub(super) struct ParserAction<G: super::traits::ParserGrammar> {
    pub(super) r#type: G::TerminalType,
    pub(super) op: ParserOp
}
