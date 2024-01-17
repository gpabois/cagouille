use std::fmt::{Debug, Formatter};

use yalp::parser::traits::ParserSymbolClass;

use crate::html::{VElementAttributes, VElementNode, VNode, VChildrenNode, VElementAttribute};


#[derive(Clone, Debug, PartialEq)]
/// The symbol class
pub enum Class {
    ///////////////
    // Terminals //
    ///////////////
    /// $
    EOS,
    /// <
    LeftAngle,
    /// </
    ClosingLeftAngle,
    /// />
    SingleRightAngle,
    /// >
    RightAngle,
    /// A path at which a named item is exported
    /// Exemple: std::collections::HashMap
    Path,
    /// A literal, a number or a str.
    Lit,
    /// An identifier
    Ident,
    /// =
    Equal,
    /// A braced block containing Rust statements.
    Block,

    ///////////////////
    // Non terminals //
    ///////////////////

    /// Root node
    Root,
    /// A vdom renderer node
    Node,
    /// An element renderer 
    /// 
    /// Either an html element renderer or a component renderer
    /// 
    /// Exemple: 
    /// - <div class="blabla"></div>
    /// - <MyFoo prop_attr={bar}></MyFoo>
    Element,
    /// An open tag
    /// 
    /// Exemple:
    /// - <tag-name attr_1="blabla">
    OpenTag,
    /// A close tag
    /// 
    /// Exemple:
    /// - </tag-name>
    CloseTag,
    /// A single tag
    /// 
    /// Exemple:
    /// - 
    SingleTag,
    /// The children of an element
    ElementChildren,
    /// The attributes of an element
    ElementAttributes,
    /// An element attribute
    ElementAttribute
}

impl ParserSymbolClass for Class {
    fn is_terminal(&self) -> bool {
        match self {
            Class::EOS => true,
            Class::LeftAngle => true,
            Class::ClosingLeftAngle => true,
            Class::SingleRightAngle => true,
            Class::RightAngle => true,
            Class::Path => true,
            Class::Lit => true,
            Class::Ident => true,
            Class::Equal => true,
            Class::Block => true,
            _ => false,
        }
    }

    fn eos() -> Self {
        Class::EOS
    }
}

#[derive(Clone)]
/// The symbol value
pub(super) enum Value {
    // Terminals
    EOS,
    LeftAngle(syn::Token![<]),
    ClosingLeftAngle(syn::Token![<], syn::Token![/]),
    SingleRightAngle(syn::Token![/], syn::Token![>]),
    RightAngle(syn::Token![>]),
    Path(syn::Path),
    Lit(syn::Lit),
    Ident(syn::Ident),
    Equal(syn::Token![=]),
    Block(syn::Block),

    // Non terminals
    Node(VNode),
    Element(VElementNode),
    OpenTag(OpenTag),
    SingleTag(SingleTag),
    CloseTag(CloseTag),
    ElementChildren(VChildrenNode),
    ElementAttributes(VElementAttributes),
    ElementAttribute(VElementAttribute)
}

impl Debug for Value {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        todo!() 
    }
}

impl From<VNode> for Value {
    fn from(value: VNode) -> Self {
        Self::Node(value)
    }
}

impl TryInto<VNode> for Value {
    type Error = ();

    fn try_into(self) -> Result<VNode, Self::Error> {
        match self {
            Value::Node(n) => Ok(n),
            _ => Err(())
        }
    }
}

impl From<VElementNode> for Value {
    fn from(value: VElementNode) -> Self {
        Self::Element(value)
    }
}

impl TryInto<VElementNode> for Value {
    type Error = ();

    fn try_into(self) -> Result<VElementNode, Self::Error> {
        match self {
            Value::Element(n) => Ok(n),
            _ => Err(())         
        }
    }
}

impl TryInto<syn::Block> for Value {
    type Error = ();

    fn try_into(self) -> Result<syn::Block, Self::Error> {
        match self {
            Value::Block(bck) => Ok(bck),
            _ => Err(())
        }
    }
}

impl TryInto<syn::Lit> for Value {
    type Error = ();

    fn try_into(self) -> Result<syn::Lit, Self::Error> {
        match self {
            Value::Lit(lit) => Ok(lit),
            _ => Err(())
        }
    }
}

impl From<syn::Ident> for Value {
    fn from(value: syn::Ident) -> Self {
        Self::Ident(value)
    }
}

impl TryInto<syn::Ident> for Value {
    type Error = ();

    fn try_into(self) -> Result<syn::Ident, Self::Error> {
        match self {
            Value::Ident(val) => Ok(val),
            _ => Err(())
        }
    }
}

impl From<SingleTag> for Value {
    fn from(value: SingleTag) -> Self {
        Self::SingleTag(value)
    }
}

impl TryInto<SingleTag> for Value {
    type Error = ();

    fn try_into(self) -> Result<SingleTag, Self::Error> {
        match self {
            Value::SingleTag(single_tag) => Ok(single_tag),
            _ => Err(())
        }
    }
}

impl From<OpenTag> for Value {
    fn from(value: OpenTag) -> Self {
        Self::OpenTag(value)
    }
}

impl TryInto<OpenTag> for Value {
    type Error = ();

    fn try_into(self) -> Result<OpenTag, Self::Error> {
        match self {
            Value::OpenTag(val) => Ok(val),
            _ => Err(())
        }
    }
}

impl TryInto<VChildrenNode> for Value {
    type Error = ();

    fn try_into(self) -> Result<VChildrenNode, Self::Error> {
        match self {
            Value::ElementChildren(val) => Ok(val),
            _ => Err(())
        }
    }
}

impl From<CloseTag> for Value {
    fn from(value: CloseTag) -> Self {
        Self::CloseTag(value)
    }
}

impl TryInto<CloseTag> for Value {
    type Error = ();

    fn try_into(self) -> Result<CloseTag, Self::Error> {
        match self {
            Value::CloseTag(val) => Ok(val),
            _ => Err(())
        }
    }
}

impl From<VChildrenNode> for Value {
    fn from(value: VChildrenNode) -> Self {
        Self::ElementChildren(value)
    }
}

impl From<VElementAttributes> for Value {
    fn from(value: VElementAttributes) -> Self {
        Self::ElementAttributes(value)
    }
}

impl TryInto<VElementAttributes> for Value {
    type Error = ();

    fn try_into(self) -> Result<VElementAttributes, Self::Error> {
        match self {
            Value::ElementAttributes(val) => Ok(val),
            _ => Err(())
        }
    }
}

impl From<VElementAttribute> for Value {
    fn from(value: VElementAttribute) -> Self {
        Self::ElementAttribute(value)
    }
}

impl TryInto<VElementAttribute> for Value {
    type Error = ();

    fn try_into(self) -> Result<VElementAttribute, Self::Error> {
        match self {
            Value::ElementAttribute(val) => Ok(val),
            _ => Err(())
        }
    }
}

#[derive(Clone)]
pub(super) struct OpenTag {
    pub tag: Option<syn::Ident>,
    pub attrs: VElementAttributes,
}

#[derive(Clone)]
pub(super) struct SingleTag {
    pub tag: Option<syn::Ident>,
    pub attrs: VElementAttributes,
}

#[derive(Clone)]
pub(super) struct CloseTag {
    pub tag: Option<syn::Ident>
}
