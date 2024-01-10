pub trait Symbol: Clone {
    type Type: Clone + PartialEq;

    /// Returns the type of the symbol
    fn get_type(&self) -> Self::Type;
}
