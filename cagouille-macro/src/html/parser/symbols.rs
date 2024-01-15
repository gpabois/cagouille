use std::fmt::{Debug, Formatter};

use yalp::{parser::{traits::ParserSymbolClass, ParserError}, symbol::Sym};

use crate::html::{VElementAttributes, VElementNode, VNode, VChildrenNode, VElementAttribute};

use super::VDomRendererParserDef;

#[derive(Clone, Debug, PartialEq)]
/// The symbol class
pub enum Class {
    ///////////////
    // Terminals //
    ///////////////

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
}

#[derive(Clone)]
/// The symbol value
pub enum Value {
    // Terminals
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
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { todo!() }
}

impl From<VNode> for Value {
    fn from(value: VNode) -> Self {
        Self::Node(value)
    }
}

impl TryFrom<Sym<VDomRendererParserDef>> for VNode {
    type Error = ParserError;

    fn try_from(sym: Sym<VDomRendererParserDef>) -> Result<Self, Self::Error> {
        match sym.value {
            Value::Node(n) => Ok(n),
            _ => Err(ParserError::unexpected_token(sym, vec![Class::Node]))
        }
    }
}

impl From<VElementNode> for Value {
    fn from(value: VElementNode) -> Self {
        Self::Element(value)
    }
}

impl TryFrom<Sym<VDomRendererParserDef>> for VElementNode {
    type Error = ParserError;

    fn try_from(sym: Sym<VDomRendererParserDef>) -> Result<Self, Self::Error> {
        match sym.value {
            Value::Element(n) => Ok(n),
            _ => Err(ParserError::unexpected_token(sym, vec![Class::Node]))
        }
    }
}

#[derive(Clone)]
struct OpenTag {
    tag: Option<syn::Ident>,
    attrs: VElementAttributes,
}

#[derive(Clone)]
struct SingleTag {
    tag: Option<syn::Ident>,
    attrs: VElementAttributes,
}

#[derive(Clone)]
struct CloseTag {
    tag: Option<syn::Ident>
}
