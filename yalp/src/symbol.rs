use std::fmt::Debug;

use crate::span::Span;

use self::traits::{Symbol, SymbolDefinition};

pub mod traits {
    use std::fmt::Debug;
    use crate::span::Span;

    pub trait SymbolDefinition : 'static {
        type Class: Clone + PartialEq + Debug;
        type Value: Clone + Debug;
    }

    pub trait Symbol: Clone + Debug + SymbolDefinition {
        /// Returns the value of the symbol
        fn get_value(&self) -> &Self::Value;

        /// Returns the type of the symbol
        fn get_type(&self) -> &Self::Class;
    
        /// Returns the span of the symbol
        fn span(&self) -> Span;
    }    

}

pub struct Sym<S: SymbolDefinition> {
    pub span:   Span,
    pub r#type: S::Class,
    pub value:  S::Value
}

impl<S> Clone for Sym<S> 
where S: SymbolDefinition
{
    fn clone(&self) -> Self {
        Self { span: self.span.clone(), r#type: self.r#type.clone(), value: self.value.clone() }
    }
}

impl<S> Debug for Sym<S> 
where S: SymbolDefinition
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sym").field("span", &self.span).field("r#type", &self.r#type).field("value", &self.value).finish()
    }
}

impl<S: SymbolDefinition> SymbolDefinition for Sym<S> {
    type Class = S::Class;
    type Value = S::Value;
}

impl<S: SymbolDefinition> traits::Symbol for Sym<S> {
    fn get_value(&self) -> &Self::Value {
        &self.value
    }

    fn get_type(&self) -> &Self::Class {
        &self.r#type
    }

    fn span(&self) -> Span {
        self.span.clone()
    }
}
