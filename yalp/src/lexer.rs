pub mod traits {
    use crate::symbol::Symbol;

    pub trait Lexer: Iterator<Item=Result<Self::Symbol, Self::Error>> {
        type Symbol: Symbol;
        type Error;
    }
}