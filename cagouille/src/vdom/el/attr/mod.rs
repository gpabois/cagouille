mod render;
mod value;

pub use value::Value;

pub struct Attribute {
    name: String,
    value: Value
}

impl Attribute {
    pub fn new(name: String, value: Value) -> Self {
        Self{name, value}
    }
}

#[derive(Default, Clone)]
pub struct Attributes(std::collections::HashMap<String, Value>);

impl Attributes {
    pub fn set<IntoStr: Into<String>, IntoValue: Into<Value>>(
        &mut self,
        name: IntoStr,
        value: IntoValue,
    ) {
        self.0.insert(name.into(), value.into());
    }
}

impl<'a> FromIterator<(&'a str, Value)> for Attributes {
    fn from_iter<T: IntoIterator<Item = (&'a str, Value)>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|(name, val)| (name.to_owned(), val))
                .collect(),
        )
    }
}

impl FromIterator<(String, Value)> for Attributes {
    fn from_iter<T: IntoIterator<Item = (String, Value)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
