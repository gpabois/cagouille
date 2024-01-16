use std::fmt::Debug;

use crate::{span::Span, parser::ParserError};

use self::traits::{Symbol, SymbolDefinition};

pub mod traits {
    use std::fmt::Debug;
    use crate::{span::Span, parser::ParserError};

    pub trait SymbolDefinition : 'static {
        type Class: Clone + PartialEq + Debug;
        type Value: Clone + Debug;
    }

    pub trait Symbol: Clone + Debug + SymbolDefinition {
        /// Returns the value of the symbol
        fn get_value(&self) -> &Self::Value;

        ///
        fn into_value<V>(self) -> Result<V, ParserError> where Self::Value: TryInto<V>;

        /// Returns the type of the symbol
        fn get_type(&self) -> &Self::Class;
    
        /// Returns the span of the symbol
        fn span(&self) -> Span;
    }    

}

pub struct Sym<S: SymbolDefinition> {
    pub span:   Span,
    pub class: S::Class,
    pub value:  S::Value
}

impl<S> Sym<S> where S: SymbolDefinition {
    pub fn new<IntoSpan: Into<Span>, IntoType: Into<S::Class>, IntoValue: Into<S::Value>>(span: IntoSpan, typ: IntoType, value: IntoValue) -> Self {
        Self {
            span: span.into(),
            class: typ.into(),
            value: value.into()
        }
    }
}

impl<S> Clone for Sym<S> 
where S: SymbolDefinition
{
    fn clone(&self) -> Self {
        Self { span: self.span.clone(), class: self.class.clone(), value: self.value.clone() }
    }
}

impl<S> Debug for Sym<S> 
where S: SymbolDefinition
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sym").field("span", &self.span).field("r#type", &self.class).field("value", &self.value).finish()
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
        &self.class
    }

    fn span(&self) -> Span {
        self.span.clone()
    }

    fn into_value<V>(self) -> Result<V, ParserError> where Self::Value: TryInto<V> {
        self.value.clone().try_into().map_err(|_| ParserError::wrong_value(self))
    }
}
