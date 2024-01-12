use std::vec;

use lazy_static::lazy_static;

use yalp::parser::{rule::ParserRuleSet, ParserError, traits::{ParserSymbolType, Parser}};

use super::{lexer::{VNodeToken, VNodeTokenType}, VElementAttributes, VNode, VElementNode, VElementAttribute, VChildrenNode};

#[derive(Debug, Clone, PartialEq)]
pub enum VNodeParserSymbolType {
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
            Self::Token(tok) => tok.clone(),
            _ => unreachable!("not a terminal type")
        }
    }
}

/// Parser symbols
#[derive(Clone)]
pub enum VNodeParserSymbol {
    // Terminals symbols
    Token(VNodeToken),

    // Non-terminals symbols
    RootNode(VElementNode),
    Node(VNode),
    Element(VElementNode),
    OpenTag(OpenTag),
    SingleTag(SingleTag),
    CloseTag(CloseTag),
    Children(VChildrenNode),
    Attribute(VElementAttribute),
    Attributes(VElementAttributes),
}

impl From<VNode> for VNodeParserSymbol {
    fn from(value: VNode) -> Self {
        Self::Node(value)
    }
}

impl From<VElementNode> for VNodeParserSymbol {
    fn from(value: VElementNode) -> Self {
        Self::Element(value)
    }
}

impl From<OpenTag> for VNodeParserSymbol {
    fn from(value: OpenTag) -> Self {
        Self::OpenTag(value)
    }
}

impl From<SingleTag> for VNodeParserSymbol {
    fn from(value: SingleTag) -> Self {
        Self::SingleTag(value)
    }
}

impl From<CloseTag> for VNodeParserSymbol {
    fn from(value: CloseTag) -> Self {
        Self::CloseTag(value)
    }
}

impl From<VChildrenNode> for VNodeParserSymbol {
    fn from(value: VChildrenNode) -> Self {
        Self::Children(value)
    }
}

impl From<VElementAttribute> for VNodeParserSymbol {
    fn from(value: VElementAttribute) -> Self {
        Self::Attribute(value)
    }
}

impl From<VElementAttributes> for VNodeParserSymbol {
    fn from(value: VElementAttributes) -> Self {
        Self::Attributes(value)
    }
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

impl TryInto<OpenTag> for VNodeParserSymbol {
    type Error = ParserError<VNodeParserSymbol>;

    fn try_into(self) -> Result<OpenTag, Self::Error> {
        match self {
            Self::OpenTag(open_tag) => Ok(open_tag),
            _ => unreachable!()
        }
    }
}

impl TryInto<SingleTag> for VNodeParserSymbol {
    type Error = ParserError<VNodeParserSymbol>;

    fn try_into(self) -> Result<SingleTag, Self::Error> {
        match self {
            Self::SingleTag(single_tag) => Ok(single_tag),
            _ => unreachable!()
        }
    }
}

impl TryInto<VElementNode> for VNodeParserSymbol {
    type Error = ParserError<VNodeParserSymbol>;

    fn try_into(self) -> Result<VElementNode, Self::Error> {
        match self {
            Self::Element(element) => Ok(element),
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

lazy_static!{
    /*
        RootNode -> Element
        Node -> Element
        Node -> block
        Node -> lit
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
        Attrs -> Attrs Attr
        Attrs -> Attr
        Attr -> ident = block
        Attr -> ident = lit
    */

    pub static ref VNODE_RULES: ParserRuleSet<VNodeParserSymbol> = yalp::parser::rule::ParserRuleSet::<VNodeParserSymbol>::new()
        .add( // RootNode -> Element
            VNodeParserSymbolType::RootNode, [VNodeParserSymbolType::Element],
            &|syms| {
                let el: VElementNode = syms[0].clone().try_into()?;
                Ok(VNodeParserSymbol::RootNode(el))
            }
        )
        .add( // Element -> SingleTag
            VNodeParserSymbolType::Element, [VNodeParserSymbolType::SingleTag],
            &|syms| {
                let SingleTag { tag, attrs } = syms[0].clone().try_into()?;
                Ok(VElementNode::new(tag, attrs, Default::default()).into())
            }
        )
        .add( // Element -> OpenTag Children CloseTag
            VNodeParserSymbolType::Element, [VNodeParserSymbolType::OpenTag, VNodeParserSymbolType::Children, VNodeParserSymbolType::CloseTag],
            &|syms| {
                let OpenTag { tag, attrs } = syms[0].clone().try_into()?;
                let children: VChildrenNode = syms[1].clone().try_into()?;
                Ok(VElementNode::new(tag, attrs, children).into())
            }
        )
        .add( // Element -> OpenTag CloseTag
            VNodeParserSymbolType::Element, [VNodeParserSymbolType::OpenTag, VNodeParserSymbolType::CloseTag],  
            &|syms| {
                let OpenTag{tag, attrs} = syms[0].clone().try_into()?;
                Ok(VElementNode::new(tag, attrs, Default::default()).into())
            }
        )
        .add( // OpenTag -> < ident >
            VNodeParserSymbolType::OpenTag, [VNodeTokenType::LeftAngle, VNodeTokenType::Ident, VNodeTokenType::RightAngle],
            &|syms| {
                let tag: syn::Ident = syms[1].clone().try_into()?;
                Ok(OpenTag { tag: Some(tag), attrs: Default::default() }.into())
            }
        )
        .add( // OpenTag -> < ident Attrs >
            VNodeParserSymbolType::OpenTag, [VNodeTokenType::LeftAngle.into(), VNodeTokenType::Ident.into(), VNodeParserSymbolType::Attributes, VNodeTokenType::RightAngle.into()],
            &|syms| {
                let tag: syn::Ident = syms[1].clone().try_into()?;
                let attrs: VElementAttributes = syms[2].clone().try_into()?;
                Ok(OpenTag { tag: Some(tag), attrs }.into())
            }
        )
        .add( // CloseTag -> closing_angle ident >
            VNodeParserSymbolType::CloseTag, [VNodeTokenType::ClosingLeftAngle, VNodeTokenType::Ident, VNodeTokenType::RightAngle],
            &|syms| {
                let tag: syn::Ident = syms[1].clone().try_into()?;
                Ok(CloseTag { tag: Some(tag) }.into())
            }
        )
        .add( // SingleTag -> < ident Attrs />
            VNodeParserSymbolType::SingleTag, [VNodeTokenType::LeftAngle.into(), VNodeTokenType::Ident.into(), VNodeParserSymbolType::Attributes, VNodeTokenType::SingleRightAngle.into()],
            &|syms| {
                let tag: syn::Ident = syms[1].clone().try_into()?;
                let attrs: VElementAttributes = syms[2].clone().try_into()?;
                Ok(SingleTag { tag: Some(tag), attrs }.into())
            }
        )
        .add( // SingleTag -> < ident />
            VNodeParserSymbolType::SingleTag, [VNodeTokenType::LeftAngle.into(), VNodeTokenType::Ident.into(), VNodeParserSymbolType::Attributes, VNodeTokenType::SingleRightAngle.into()],
            &|syms| {
                let tag: syn::Ident = syms[1].clone().try_into()?;
                Ok(SingleTag { tag: Some(tag), attrs: Default::default() }.into())
            }
        )
        .add( // Children -> Children Node
            VNodeParserSymbolType::Children, [VNodeParserSymbolType::Children, VNodeParserSymbolType::Node],
            &|syms| {
                let mut children: VChildrenNode = syms[0].clone().try_into()?;
                let node: VNode = syms[1].clone().try_into()?;
                children.0.push(node);
                Ok(children.into())
            } 
        )
        .add( // Children -> Node
            VNodeParserSymbolType::Children, [VNodeParserSymbolType::Node],
            &|syms| {
                let node: VNode = syms[0].clone().try_into()?;
                Ok(VChildrenNode(vec![node]).into())
            } 
        )
        .add( // Node -> Element
            VNodeParserSymbolType::Node, [VNodeParserSymbolType::Element],
            &|syms| {
                let el: VElementNode = syms[0].clone().try_into()?;
                Ok(VNode::Element(el).into())
            } 
        )
        .add( // Node -> block
            VNodeParserSymbolType::Node, [VNodeTokenType::Block],
            &|syms| {
                let block: syn::Block = syms[0].clone().try_into()?;
                Ok(VNode::Block(block).into())
            } 
        )
        .add( // Node -> lit
            VNodeParserSymbolType::Node, [VNodeTokenType::Lit],
            &|syms| {
                let lit: syn::Lit = syms[0].clone().try_into()?;
                Ok(VNode::Lit(lit).into())
            } 
        )
        .add( //  Attrs -> Attrs Attr
            VNodeParserSymbolType::Attributes, [VNodeParserSymbolType::Attributes, VNodeParserSymbolType::Attribute],
            &|syms| {
                let mut attrs: VElementAttributes = syms[0].clone().try_into()?;
                let attr: VElementAttribute = syms[1].clone().try_into()?;
                attrs.0.push(attr);
                Ok(attrs.into())
            } 
        )
        .add( // Attrs -> Attr
            VNodeParserSymbolType::Attributes, [VNodeParserSymbolType::Attribute],
            &|syms| {
                let attr: VElementAttribute = syms[0].clone().try_into()?;
                Ok(VElementAttributes(vec![attr]).into())
            } 
        )
        .add( // Attr -> ident = block
            VNodeParserSymbolType::Attribute, [VNodeTokenType::Ident, VNodeTokenType::Equal, VNodeTokenType::Block],
            &|syms| {
                let name: syn::Ident = syms[0].clone().try_into()?;
                let block: syn::Block = syms[1].clone().try_into()?;
                Ok(VElementAttribute::new(name, block).into())
            }
        )
        .add( // Attr -> ident = lit
            VNodeParserSymbolType::Attribute, [VNodeTokenType::Ident, VNodeTokenType::Equal, VNodeTokenType::Lit],
            &|syms| {
                let name: syn::Ident = syms[0].clone().try_into()?;
                let lit: syn::Lit = syms[1].clone().try_into()?;
                Ok(VElementAttribute::new(name, lit).into())
            }
        )
        .to_owned();

    pub static ref VNODE_PARSER: yalp::parser::lr::LrParser<'static, VNodeParserSymbol> = yalp::parser::lr::LrParser::generate(&VNODE_RULES);
}
