use std::collections::{VecDeque, HashMap};

use lazy_static::lazy_static;
use quote::ToTokens;
use syn::{parse::Parse, parse::ParseStream, Token};


pub struct VNodeParser<'a> {
    pub stack: VecDeque<VNodeParserValue>,
    pub lexer: VNodeLexer<'a>
}

pub enum VNodeParserStateAction {
    Accept,
    Shift(u32),
    Reduce(&'static (dyn Fn(&mut VecDeque<VNodeParserValue>) -> (VNodeParserValue, u32) + Sync))
}

impl VNodeParserStateAction {
    pub const fn shift(to: u32) -> Self {
        Self::Shift(to)
    }

    pub const fn reduce(f: &'static (dyn Fn(&mut VecDeque<VNodeParserValue>) -> u32 + Sync)) -> Self {
        Self::Reduce(f)
    }
}

pub struct VNodeParserRule {
    tok_type: VNodeTokenType,
    action: VNodeParserStateAction
}

impl VNodeParserRule {
    pub const fn new(tok_type: VNodeTokenType, action: VNodeParserStateAction) -> Self {
        Self{tok_type, action}
    }
}

lazy_static!{
    static ref VNODE_PARSER_TABLE: Vec<Vec<VNodeParserRule>> = {
        vec![
            // 0: root
            vec![
                // <...
                VNodeParserRule::new(VNodeTokenType::RightAngle, VNodeParserStateAction::shift(1)),
                // EOS, we accept the parsing 
                VNodeParserRule::new(VNodeTokenType::EOS, VNodeParserStateAction::Accept)
            ],
            // 1: root / fetch tag name, or closing tag
            vec![
                // Tag name
                VNodeParserRule::new(VNodeTokenType::Ident, VNodeParserStateAction::shift(2)),
                // Right angle, we go to found children
                VNodeParserRule::new(VNodeTokenType::RightAngle, VNodeParserStateAction::shift(3)),
                // Reduce to root vnode
                VNodeParserRule::new(
                    VNodeTokenType::SingleRightAngle, 
                    VNodeParserStateAction::reduce(&|stack| {
                        // />
                        let t1 = stack.pop_back().unwrap();
                        let t2 = stack.pop_back().unwrap();
                        let mut attrs = VNodeParserValue::VNodeAttributes(Default::default());
                        
                        if let VNodeParserValue::VNodeAttributes(_) = t2 {
                            attrs = t2;
                        }


                        return 0;
                    })
                )
            ]
            // 2: fetch attributes, or closing tag
            // 3: fetch children
        ]
    };
}

pub enum VNodeAttributeValue {
    Block(syn::Block),
    Lit(syn::Lit)
}
pub enum VNodeParserValue {
    VNodeAttribute(syn::Ident, VNodeAttributeValue),
    VNodeAttributes(Vec<(syn::Ident, VNodeAttributeValue)>),
    VNode(VNode)
}

impl<'a> VNodeParser<'a> {
    pub fn new(input: ParseStream) -> Self {
        Self {
            lexer: VNodeLexer::new(input),
            stack: Default::default()
        }
    }

    pub fn parse(&mut self) -> syn::Result<VNode> {

    }
}


pub enum VNode {
    Token(VNodeToken),
    Block(VBlockNode),
    Conditional(VConditionalNode),
    Component(VComponentNode),
    Children(VChildrenNode),
    Empty
}

impl ToTokens for VNode {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        todo!()
    }
}

impl Parse for VNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        VNodeParser::new(input).parse()
    }
}

pub enum VNodeTokenType {
    LeftAngle,
    ClosingLeftAngle,
    SingleRightAngle,
    RightAngle,
    Lit,
    Ident,
    Equal,
    Block,
    EOS
}

#[derive(Clone)]
pub enum VNodeToken {
    LeftAngle(syn::Token![<]),
    ClosingLeftAngle(syn::Token![<], syn::Token![/]),
    SingleRightAngle(syn::Token![/], syn::Token![>]),
    RightAngle(syn::Token![>]),
    Lit(syn::Lit),
    Ident(syn::Ident),
    Equal(syn::Token![=]),
    Block(syn::Block),
    EOS
}


impl VNodeToken {
    pub fn next(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(VNodeToken::EOS);
        }

        if input.peek(Token![<]) && input.peek2(Token![/]) {
            return Ok(VNodeToken::ClosingLeftAngle ( input.parse()?, input.parse()?) );
        }

        if input.peek(Token![<]) {
            return Ok(VNodeToken::LeftAngle(input.parse()?));
        }

        if input.peek(Token![/]) && input.peek2(Token![>]) {
            return Ok(Self::SingleRightAngle(input.parse()?, input.parse()?));
        }

        if input.peek(Token![>]) {
            return Ok(Self::RightAngle(input.parse()?));
        }

        if input.peek(syn::Ident) {
            return Ok(Self::Ident(input.parse()?));
        }
        
        if input.peek(Token![=]) {
            return Ok(Self::Equal(input.parse()?));
        }

        if input.peek(syn::Lit) {
            return Ok(Self::Lit(input.parse()?));
        }

        if input.fork().parse::<syn::Block>().is_ok() {
            return Ok(Self::Block(input.parse()?));
        }

        unreachable!()
    }
}

pub struct VNodeLexer<'a> {
    input: ParseStream<'a>,
    exhausted: bool,
    buffer: VecDeque<syn::Result<VNodeToken>>
}


impl<'a> VNodeLexer<'a> {
    pub fn new(input: ParseStream) -> Self {
        Self{input, exhausted: false, buffer: Default::default()}
    }

    fn consume(&mut self) -> syn::Result<VNodeToken> {
        VNodeToken::next(self.input)
    }

    pub fn peek(&mut self) -> syn::Result<VNodeToken> {
        let tok = self.consume();
        self.buffer.push_back(tok.clone());
        tok
    }
}

impl<'a> Iterator for VNodeLexer<'a> {
    type Item = syn::Result<VNodeToken>;

    fn next(&mut self) -> Option<Self::Item> {

        if self.exhausted {
            return None;
        }

        if self.buffer.len() > 0 {
            return self.buffer.pop_front();
        }

        match self.consume() {
            Err(err) => {
                self.exhausted = true;
                return Some(Err(err));
            },
            Ok(tok) => {
                if let VNodeToken::EOS = tok {
                    self.exhausted = true;
                }
                Some(Ok(tok))
            }
        }
    }
}

/// Virtual children
pub struct VChildrenNode;

/// Virtual Block of statements
pub struct VBlockNode;

impl Parse for VBlockNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

/// Cagouille component
pub struct VComponentNode;

/// HTML Element
pub struct VElementNode;

/// Conditional statements
pub struct VConditionalNode;

/// Virtual root
pub struct RootVNode(VNode);