use std::fmt::Debug;

pub trait Symbol<Type: Clone + PartialEq + Debug>: Clone {

    /// Returns the type of the symbol
    fn get_type(&self) -> Type;
}
