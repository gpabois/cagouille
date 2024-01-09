pub trait Symbol: Clone {
    type Type: Clone + PartialEq;

    fn get_type(&self) -> Self::Type;
}
