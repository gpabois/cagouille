pub trait Symbol<Type: Clone + PartialEq>: Clone {

    /// Returns the type of the symbol
    fn get_type(&self) -> Type;
}
