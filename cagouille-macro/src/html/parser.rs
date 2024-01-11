use std::vec;

use lazy_static::lazy_static;

use proc_macro2::Span;
use yalp::{symbol::{self, Symbol}, parser::{rule::ParserRuleSet, ParserError, traits::{ParserSymbolType, Parser}}};

use crate::html::VBlockNode;

use super::{lexer::{VNodeToken, VNodeTokenType}, VElementAttributes, VNode, VElementNode, VElementAttribute, VChildrenNode};

#[derive(Clone, PartialEq)]
enum VNodeParserSymbolType {
    Token(VNodeTokenType),
    RootNode,
    Element,
    OpenTag,
    CloseTag,
    SingleTag,
    Children,
    Node,
    Attributes,
    Attribute
}

impl ParserSymbolType<VNodeTokenType> for VNodeParserSymbolType {
    fn is_terminal(&self) -> bool {
        match self {
            Self::Token(_) => true,
            _ => false
        }
    }

    fn expect_terminal_type(&self) -> VNodeTokenType {
        match self {
            Self::Token(tok) => *tok,
            _ => unreachable!("not a terminal type")
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

/// Parser Value
#[derive(Clone)]
enum VNodeParserSymbol {
    // Terminals symbols
    Token(VNodeToken),

    // Non-terminals symbols
    Attribute(VElementAttribute),
    Attributes(VElementAttributes),
    OpenTag(OpenTag),
    SingleTag(SingleTag),
    CloseTag(CloseTag),
    Element(VElementNode),
    Children(VChildrenNode),
    Node(VNode),
    RootNode(VElementNode)
}

impl From<VNodeTokenType> for VNodeParserSymbolType {
    fn from(value: VNodeTokenType) -> Self {
        Self::Token(value)
    }
}

impl yalp::parser::traits::ParserSymbol for VNodeParserSymbol {
    type Terminal = VNodeToken;
    type Type = VNodeParserSymbolType;
}

impl From<VNodeToken> for VNodeParserSymbol {
    fn from(value: VNodeToken) -> Self {
        Self::Token(value)
    }
}

impl TryInto<syn::Ident> for VNodeParserSymbol {
    type Error = ParserError<VNodeParserSymbol>;

    fn try_into(self) -> Result<syn::Ident, Self::Error> {
        match self {
            Self::Token(VNodeToken::Ident(ident)) => Ok(ident),
            _ => unreachable!()
        }
    }
}

impl TryInto<syn::Block> for VNodeParserSymbol {
    type Error = ParserError<VNodeParserSymbol>;

    fn try_into(self) -> Result<syn::Block, Self::Error> {
        match self {
            Self::Token(VNodeToken::Block(block)) => Ok(block),
            _ => unreachable!()
        }
    }
}

impl TryInto<syn::Lit> for VNodeParserSymbol {
    type Error = ParserError<VNodeParserSymbol>;

    fn try_into(self) -> Result<syn::Lit, Self::Error> {
        match self {
            Self::Token(VNodeToken::Lit(lit)) => Ok(lit),
            _ => unreachable!()
        }
    }
}

impl TryInto<VNode> for VNodeParserSymbol {
    type Error = ParserError<VNodeParserSymbol>;

    fn try_into(self) -> Result<VNode, Self::Error> {
        match self {
            Self::Node(node) => Ok(node),
            _ => unreachable!()
        }
    }
}

impl TryInto<VChildrenNode> for VNodeParserSymbol {
    type Error = ParserError<VNodeParserSymbol>;

    fn try_into(self) -> Result<VChildrenNode, Self::Error> {
        match self {
            Self::Children(children) => Ok(children),
            _ => unreachable!()
        }
    }
}

impl TryInto<VElementNode> for VNodeParserSymbol {
    type Error = ParserError<VNodeParserSymbol>;

    fn try_into(self) -> Result<VElementNode, Self::Error> {
        match self {
            Self::Element(element) => Ok(element),
            Self::OpenTag(OpenTag { tag, attrs }) => Ok(VElementNode {
                tag, attrs, children: Default::default()
            }),
            Self::SingleTag(SingleTag{tag, attrs}) => Ok(VElementNode {
                tag, attrs, children: Default::default()
            }),
            _ => unreachable!()
        }
    }
}

impl TryInto<VElementAttribute> for VNodeParserSymbol {
    type Error = ParserError<VNodeParserSymbol>;

    fn try_into(self) -> Result<VElementAttribute, Self::Error> {
        match self {
            Self::Attribute(attr) => Ok(attr),
            _ => unreachable!()
        }
    }
}

impl TryInto<VElementAttributes> for VNodeParserSymbol {
    type Error = ParserError<VNodeParserSymbol>;

    fn try_into(self) -> Result<VElementAttributes, Self::Error> {
        match self {
            Self::Attributes(attrs) => Ok(attrs),
            _ => unreachable!()
        }
    }
}

impl TryInto<VNodeToken> for VNodeParserSymbol {
    type Error = syn::Error;

    fn try_into(self) -> Result<VNodeToken, Self::Error> {
        match self {
            Self::Token(token) => Ok(token),
            _ => unreachable!()
        }
    }
}

impl yalp::symbol::Symbol<VNodeParserSymbolType> for VNodeParserSymbol {
    
    fn get_type(&self) -> VNodeParserSymbolType {
        match self {
            VNodeParserSymbol::Token(tok) => VNodeParserSymbolType::Token(tok.get_type()),
            VNodeParserSymbol::Attribute(_) => VNodeParserSymbolType::Attribute,
            VNodeParserSymbol::Attributes(_) => VNodeParserSymbolType::Attributes,
            VNodeParserSymbol::OpenTag (_) => VNodeParserSymbolType::OpenTag,
            VNodeParserSymbol::SingleTag (_) => VNodeParserSymbolType::SingleTag,
            VNodeParserSymbol::CloseTag (_) => VNodeParserSymbolType::CloseTag,
            VNodeParserSymbol::Children(_) => VNodeParserSymbolType::Children,
            VNodeParserSymbol::Node(_) => VNodeParserSymbolType::Node,
            VNodeParserSymbol::RootNode(_) => VNodeParserSymbolType::RootNode,
            VNodeParserSymbol::Element(_) => VNodeParserSymbolType::Element,
        }
    }
}


#[derive(Clone)]
pub struct VNodeParser;

lazy_static!{
    /*
        RootNode -> Element
        Element -> SingleTag
        Element -> OpenTag Children CloseTag
        Element -> OpenTag CloseTag
        OpenTag -> < ident >
        OpenTag -> < ident Attrs >
        CloseTag -> closing_angle ident >
        SingleTag -> < ident Attrs />
        SingleTag -> < ident />
        Children -> Children Node
        Children -> Node
        Node -> Element
        Node -> block
        Node -> lit
        Attrs -> Attrs Attr
        Attrs -> Attr
        Attr -> ident = block
        Attr -> ident = lit
    */

    static ref VNODE_RULES: ParserRuleSet<VNodeParserSymbol> = yalp::parser::rule::ParserRuleSet::<VNodeParserSymbol>::new()
        .add( // RootNode -> Element
            VNodeParserSymbolType::RootNode, [VNodeParserSymbolType::Element],
            &|syms| {
                let el: VElementNode = syms[0].try_into()?;
                Ok(VNodeParserSymbol::RootNode(el))
            }
        )
        .add( // Element -> SingleTag
            VNodeParserSymbolType::Element, [VNodeParserSymbolType::SingleTag],
            &|syms| {
                let el: VElementNode = syms[0].try_into()?;
                Ok(VNodeParserSymbol::Element(el))
            }
        )
        .add( // Element -> OpenTag Children CloseTag
            VNodeParserSymbolType::Element, [VNodeParserSymbolType::OpenTag, VNodeParserSymbolType::Children, VNodeParserSymbolType::CloseTag],
            &|syms| {
                let mut el: VElementNode = syms[0].try_into()?;
                el.children = syms[1].try_into()?;
                Ok(VNodeParserSymbol::Element(el))
            }
        )
        .add( // Element -> OpenTag CloseTag
            VNodeParserSymbolType::Element, [VNodeParserSymbolType::OpenTag, VNodeParserSymbolType::CloseTag],  
            &|syms| {
                let el: VElementNode = syms[0].try_into()?;
                Ok(VNodeParserSymbol::Element(el))
            }
        )
        .add( // OpenTag -> < ident >
            VNodeParserSymbolType::OpenTag, [VNodeTokenType::LeftAngle, VNodeTokenType::Ident, VNodeTokenType::RightAngle],
            &|syms| {
                let tag: syn::Ident = syms[1].try_into()?;
                Ok(VNodeParserSymbol::OpenTag(OpenTag { tag: Some(tag), attrs: Default::default() }))
            }
        )
        .add( // OpenTag -> < ident Attrs >
            VNodeParserSymbolType::OpenTag, [VNodeTokenType::LeftAngle.into(), VNodeTokenType::Ident.into(), VNodeParserSymbolType::Attributes, VNodeTokenType::RightAngle.into()],
            &|syms| {
                let tag: syn::Ident = syms[1].try_into()?;
                let attrs: VElementAttributes = syms[2].try_into()?;
                Ok(VNodeParserSymbol::OpenTag(OpenTag { tag: Some(tag), attrs }))
            }
        )
        .add( // CloseTag -> closing_angle ident >
            VNodeParserSymbolType::CloseTag, [VNodeTokenType::ClosingLeftAngle, VNodeTokenType::Ident, VNodeTokenType::RightAngle],
            &|syms| {
                let tag: syn::Ident = syms[1].try_into()?;
                Ok(VNodeParserSymbol::CloseTag(CloseTag { tag: Some(tag) }))
            }
        )
        .add( // SingleTag -> < ident Attrs />
            VNodeParserSymbolType::SingleTag, [VNodeTokenType::LeftAngle.into(), VNodeTokenType::Ident.into(), VNodeParserSymbolType::Attributes, VNodeTokenType::SingleRightAngle.into()],
            &|syms| {
                let tag: syn::Ident = syms[1].try_into()?;
                let attrs: VElementAttributes = syms[2].try_into()?;
                Ok(VNodeParserSymbol::SingleTag(SingleTag { tag: Some(tag), attrs }))
            }
        )
        .add( // SingleTag -> < ident />
            VNodeParserSymbolType::SingleTag, [VNodeTokenType::LeftAngle.into(), VNodeTokenType::Ident.into(), VNodeParserSymbolType::Attributes, VNodeTokenType::SingleRightAngle.into()],
            &|syms| {
                let tag: syn::Ident = syms[1].try_into()?;
                Ok(VNodeParserSymbol::SingleTag(SingleTag { tag: Some(tag), attrs: Default::default() }))
            }
        )
        .add( // Children -> Children Node
            VNodeParserSymbolType::Children, [VNodeParserSymbolType::Children, VNodeParserSymbolType::Node],
            &|syms| {
                let mut children: VChildrenNode = syms[0].try_into()?;
                let node: VNode = syms[1].try_into()?;
                children.0.push(node);
                Ok(VNodeParserSymbol::Children(children))
            } 
        )
        .add( // Children -> Node
            VNodeParserSymbolType::Children, [VNodeParserSymbolType::Node],
            &|syms| {
                let node: VNode = syms[0].try_into()?;
                Ok(VNodeParserSymbol::Children(VChildrenNode(vec![node])))
            } 
        )
        .add( // Node -> Element
            VNodeParserSymbolType::Node, [VNodeParserSymbolType::Element],
            &|syms| {
                let el: VElementNode = syms[0].try_into()?;
                Ok(VNodeParserSymbol::Node(VNode::Element(el)))
            } 
        )
        .add( // Node -> block
            VNodeParserSymbolType::Node, [VNodeTokenType::Block],
            &|syms| {
                let block: syn::Block = syms[0].try_into()?;
                Ok(VNodeParserSymbol::Node(VNode::Block(block)))
            } 
        )
        .add( // Node -> lit
            VNodeParserSymbolType::Node, [VNodeTokenType::Lit],
            &|syms| {
                let lit: syn::Lit = syms[0].try_into()?;
                Ok(VNodeParserSymbol::Node(VNode::Lit(lit)))
            } 
        )
        .add( //  Attrs -> Attrs Attr
            VNodeParserSymbolType::Attributes, [VNodeParserSymbolType::Attributes, VNodeParserSymbolType::Attribute],
            &|syms| {
                let mut attrs: VElementAttributes = syms[0].try_into()?;
                let attr: VElementAttribute = syms[1].try_into()?;
                attrs.0.push(attr);
                Ok(VNodeParserSymbol::Attributes(attrs))
            } 
        )
        .add( // Attrs -> Attr
            VNodeParserSymbolType::Attributes, [VNodeParserSymbolType::Attribute],
            &|syms| {
                let attr: VElementAttribute = syms[0].try_into()?;
                Ok(VNodeParserSymbol::Attributes(VElementAttributes(vec![attr])))
            } 
        )
        .add( // Attr -> ident = block
            VNodeParserSymbolType::Attribute, [VNodeTokenType::Ident, VNodeTokenType::Equal, VNodeTokenType::Block],
            &|syms| {
                let name: syn::Ident = syms[0].try_into()?;
                let block: syn::Block = syms[1].try_into()?;
                Ok(VNodeParserSymbol::Attribute(VElementAttribute::new(name, block)))
            }
        )
        .add( // Attr -> ident = lit
            VNodeParserSymbolType::Attribute, [VNodeTokenType::Ident, VNodeTokenType::Equal, VNodeTokenType::Lit],
            &|syms| {
                let name: syn::Ident = syms[0].try_into()?;
                let lit: syn::Lit = syms[1].try_into()?;
                Ok(VNodeParserSymbol::Attribute(VElementAttribute::new(name, lit)))
            }
        )
        .to_owned();

    pub static ref VNODE_PARSER: yalp::parser::lr::LrParser<'static, VNodeParserSymbol> = yalp::parser::lr::LrParser::generate(&VNODE_RULES);
}
