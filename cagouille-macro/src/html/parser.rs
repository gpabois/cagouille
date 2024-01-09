use std::vec;

use lazy_static::lazy_static;

use proc_macro2::Span;
use yalp::{parser, symbol::{self, Symbol}};

use super::{lexer::VNodeToken, VElementAttributes, VNode, VElementNode, VElementAttribute, VChildrenNode};

#[derive(Clone, PartialEq, Debug)]
enum VNodeParserSymbolType {
    Token,
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

impl From<VNodeToken> for VNodeParserSymbol {
    fn from(value: VNodeToken) -> Self {
        Self::Token(value)
    }
}

fn wrong_symbol_type(expecting: VNodeParserSymbolType, got: VNodeParserSymbolType) -> String {
    format!("expexting {:?}, got {:?}", expecting, got)
}

impl TryInto<VNode> for VNodeParserSymbol {
    type Error = syn::Error;

    fn try_into(self) -> Result<VNode, Self::Error> {
        match self {
            Self::Node(node) => Ok(node),
            sym => Err(syn::Error::new(
                Span::call_site(),  
                wrong_symbol_type(
                VNodeParserSymbolType::Node, 
                sym.get_type()
            )))
        }
    }
}

impl TryInto<VChildrenNode> for VNodeParserSymbol {
    type Error = syn::Error;

    fn try_into(self) -> Result<VChildrenNode, Self::Error> {
        match self {
            Self::Children(children) => Ok(children),
            sym => Err(syn::Error::new(
                Span::call_site(),  
                wrong_symbol_type(
                VNodeParserSymbolType::Children, 
                sym.get_type()
            )))
        }
    }
}

impl TryInto<VElementNode> for VNodeParserSymbol {
    type Error = syn::Error;

    fn try_into(self) -> Result<VElementNode, Self::Error> {
        match self {
            Self::Element(element) => Ok(element),
            Self::OpenTag(OpenTag { tag, attrs }) => Ok(VElementNode {
                tag, attrs, children: Default::default()
            }),
            Self::SingleTag(SingleTag{tag, attrs}) => Ok(VElementNode {
                tag, attrs, children: Default::default()
            }),
            sym => Err(syn::Error::new(
                Span::call_site(),  
                wrong_symbol_type(
                VNodeParserSymbolType::OpenTag, 
                sym.get_type()
            )))
        }
    }
}

impl TryInto<VElementAttribute> for VNodeParserSymbol {
    type Error = syn::Error;

    fn try_into(self) -> Result<VElementAttribute, Self::Error> {
        match self {
            Self::Attribute(attr) => Ok(attr),
            sym => Err(syn::Error::new(
                Span::call_site(),  
                wrong_symbol_type(
                VNodeParserSymbolType::Attribute, 
                sym.get_type()
            )))
        }
    }
}

impl TryInto<VElementAttributes> for VNodeParserSymbol {
    type Error = syn::Error;

    fn try_into(self) -> Result<VElementAttributes, Self::Error> {
        match self {
            Self::Attributes(attrs) => Ok(attrs),
            sym => Err(syn::Error::new(
                Span::call_site(),  
                wrong_symbol_type(
                VNodeParserSymbolType::Attributes, 
                sym.get_type()
            )))
        }
    }
}

impl TryInto<VNodeToken> for VNodeParserSymbol {
    type Error = syn::Error;

    fn try_into(self) -> Result<VNodeToken, Self::Error> {
        match self {
            Self::Token(token) => Ok(token),
            sym => Err(syn::Error::new(
                Span::call_site(),  
                wrong_symbol_type(
                VNodeParserSymbolType::Token, 
                sym.get_type()
            )))
        }
    }
}

impl yalp::symbol::Symbol for VNodeParserSymbol {
    type Type = VNodeParserSymbolType;

    fn get_type(&self) -> Self::Type {
        match self {
            VNodeParserSymbol::Token(_) => Self::Type::Token,
            VNodeParserSymbol::Attribute(_) => Self::Type::Attribute,
            VNodeParserSymbol::Attributes(_) => Self::Type::Attributes,
            VNodeParserSymbol::OpenTag (_) => Self::Type::OpenTag,
            VNodeParserSymbol::SingleTag (_) => Self::Type::SingleTag,
            VNodeParserSymbol::CloseTag (_) => Self::Type::CloseTag,
            VNodeParserSymbol::Children(_) => Self::Type::Children,
            VNodeParserSymbol::Node(_) => Self::Type::Node,
            VNodeParserSymbol::RootNode(_) => Self::Type::RootNode,
            VNodeParserSymbol::Element(_) => Self::Type::Element,
        }
    }
}


#[derive(Clone)]
pub struct VNodeParser;

impl yalp::parser::traits::Parser for VNodeParser {
    type Error = syn::Error;
    type Symbol = VNodeParserSymbol;
    type Terminal = VNodeToken;
}


lazy_static!{
    /*
        S' -> RootNode
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

    static ref RULE_RUNNERS: parser::ParserRulesRunners<'static, VNodeParser> = parser::ParserRulesRunners::new()
    // 0: S' -> RootNode
    .add(&|stack| {
        let element: VElementNode = stack.try_pop_into::<VElementNode>().unwrap()?;
        Ok(VNodeParserSymbol::RootNode(element))
    })
    // 1: RootNode -> Element
    .add(&|stack| {
        let element: VElementNode = stack.try_pop_into::<VElementNode>().unwrap()?;
        Ok(VNodeParserSymbol::RootNode(element))
    })
    // 2: OpenTag -> < ident >
    .add(&|stack| {
        stack.pop();
        let ident = stack.try_pop_into::<VNodeToken>().unwrap()?.expect_ident();
        stack.pop();

        Ok(VNodeParserSymbol::OpenTag(OpenTag{ tag: Some(ident), attrs: Default::default() }))
    })
    // 3: OpenTag -> < ident Attrs >
    .add(&|stack| {
        stack.pop();
        let ident = stack.try_pop_into::<VNodeToken>().unwrap()?.expect_ident();
        let attrs = stack.try_pop_into::<VElementAttributes>().unwrap()?;
        stack.pop();
        Ok(VNodeParserSymbol::OpenTag(OpenTag{ tag: Some(ident), attrs }))
    })
    // 4: CloseTag -> </ ident >
    .add(&|stack| {
        stack.pop();
        let ident = stack.try_pop_into::<VNodeToken>().unwrap()?.expect_ident();
        stack.pop();
        Ok(VNodeParserSymbol::CloseTag(CloseTag{ tag: Some(ident) }))
    })
    // 5: SingleElement -> < ident Attrs />
    .add(&|stack| {
        stack.pop();
        let ident = stack.try_pop_into::<VNodeToken>().unwrap()?.expect_ident();
        let attrs = stack.try_pop_into::<VElementAttributes>().unwrap()?;
        stack.pop();
        Ok(VNodeParserSymbol::SingleTag(SingleTag{ tag: Some(ident), attrs }))
    })
    // 6: SingleElement -> < ident />
    .add(&|stack| {
        stack.pop();
        let ident = stack.try_pop_into::<VNodeToken>().unwrap()?.expect_ident();
        stack.pop();
        Ok(VNodeParserSymbol::SingleTag (SingleTag{ tag: Some(ident), attrs: Default::default() }))
    })   
    // 7: Children -> Children Node
    .add(&|stack| {
        let mut children = stack.try_pop_into::<VChildrenNode>().unwrap()?;
        children.push(stack.try_pop_into::<VNode>().unwrap()?);
        Ok(VNodeParserSymbol::Children(children))
    })
    // 8: Children -> Node 
    .add(&|stack| {
        let mut children = VChildrenNode::default();
        children.push(stack.try_pop_into::<VNode>().unwrap()?);
        Ok(VNodeParserSymbol::Children(children))
    })
    // 9: Node -> Element 
    .add(&|stack| {
        let element = stack.try_pop_into::<VElementNode>().unwrap()?;
        Ok(VNodeParserSymbol::Node(element.into()))
    })
    // 10: Node -> block
    .add(&|stack| {
        let block = stack.try_pop_into::<VNodeToken>().unwrap()?.expect_block();
        Ok(VNodeParserSymbol::Node(block.into()))
    })
    // 11: Node -> lit
    .add(&|stack| {
        let lit = stack.try_pop_into::<VNodeToken>().unwrap()?.expect_lit();
        Ok(VNodeParserSymbol::Node(lit.into()))
    })
    // 12: Element -> SingleTag
    .add(&|stack| {
        Ok(VNodeParserSymbol::Element(
            stack.try_pop_into::<VElementNode>().unwrap()?
        ))
    })   
    // 13: Element -> OpenTag Children CloseTag
    .add(&|stack| {
        let mut el = stack.try_pop_into::<VElementNode>().unwrap()?;
        let children = stack.try_pop_into::<VChildrenNode>().unwrap()?;
        stack.pop();
        
        el.children = children;

        Ok(VNodeParserSymbol::Element(el))
    }) 
    // 14: Element -> OpenTag CloseTag
    .add(&|stack| {
        let mut el = stack.try_pop_into::<VElementNode>().unwrap()?;
        stack.pop();
        
        Ok(VNodeParserSymbol::Element(el))
    }) 
    // 15: Attrs -> Attrs Attr
    .add(&|stack| {
        let mut attrs = stack.try_pop_into::<VElementAttributes>().unwrap()?;
        let attr = stack.try_pop_into::<VElementAttribute>().unwrap()?;
        attrs.push(attr);

        Ok(VNodeParserSymbol::Attributes(attrs))
    }) 
    // 16: Attrs -> Attr
    .add(&|stack| {
        let attr = stack.try_pop_into::<VElementAttribute>().unwrap()?;
        let mut attrs = VElementAttributes::default();
        attrs.push(attr);

        Ok(VNodeParserSymbol::Attributes(attrs))
    }) 
    // 17: Attr -> ident = block
    .add(&|stack| {
        let ident = stack.try_pop_into::<VNodeToken>().unwrap()?.expect_ident();
        stack.pop();
        let block = stack.try_pop_into::<VNodeToken>().unwrap()?.expect_block();

        Ok(VNodeParserSymbol::Attribute(VElementAttribute::new(ident, block)))
    }) 
    // 18: Attr -> ident = lit
    .add(&|stack| {
        let ident = stack.try_pop_into::<VNodeToken>().unwrap()?.expect_ident();
        stack.pop();
        let lit = stack.try_pop_into::<VNodeToken>().unwrap()?.expect_lit();

        Ok(VNodeParserSymbol::Attribute(VElementAttribute::new(ident, lit)))
    }) 
    .to_owned();
    
    static ref PARSER_TABLE: Vec<VNodeParserState> = {
        vec![
            // s0
            VNodeParserState::new()
            .action(VNodeTokenType::LeftAngle, VNodeParserStateActionOp::Shift(5))
            .goto(VNodeParserSymbolType::RootNode, 1)
            .goto(VNodeParserSymbolType::Element, 2)
            .goto(VNodeParserSymbolType::OpenTag, 4)
            .goto(VNodeParserSymbolType::SingleElement, 3)
            .to_owned(),
            // s1
            VNodeParserState::new()
            .action(VNodeTokenType::EOS, VNodeParserStateActionOp::Accept)
            .to_owned(),
            // s2
            VNodeParserState::new()
            .action(VNodeTokenType::EOS, VNodeParserStateActionOp::Reduce(1))
            .to_owned(),
            // s3
            VNodeParserState::new()
            .action(VNodeTokenType::EOS, VNodeParserStateActionOp::Reduce(2))
            .to_owned(),
            // s4
            VNodeParserState::new()
            .action(VNodeTokenType::LeftAngle, VNodeParserStateActionOp::Shift(15))
            .action(VNodeTokenType::ClosingLeftAngle, VNodeParserStateActionOp::Shift(9))
            .action(VNodeTokenType::Block, VNodeParserStateActionOp::Shift(11))
            .action(VNodeTokenType::Lit, VNodeParserStateActionOp::Shift(12))
            .goto(VNodeParserSymbolType::Element, 10)
            .goto(VNodeParserSymbolType::OpenTag, 14)
            .goto(VNodeParserSymbolType::CloseTag, 7)
            .goto(VNodeParserSymbolType::SingleElement, 13)
            .goto(VNodeParserSymbolType::Children, 6)
            .goto(VNodeParserSymbolType::Node, 8)
            .to_owned(),
            // s5
            VNodeParserState::new()
            .action(VNodeTokenType::Ident, VNodeParserStateActionOp::Shift(16))
            .to_owned()
            // s6
            
        ]
    };
}
