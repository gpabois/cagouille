#[derive(Clone)]
pub enum Value {
    String(String),
    Boolean(bool),
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl Value {
    pub fn is_literal(&self) -> bool {
        match self {
            Self::String(_) => true,
            Self::Boolean(_) => true,
        }
    }
}
