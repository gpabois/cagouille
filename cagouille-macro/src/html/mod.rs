use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use yalp::parser::traits::Parser;

use self::parser::VNODE_PARSER;

mod lexer;
mod parser;

#[derive(Clone)]
pub enum VNode {
    Block(syn::Block),
    Branch(VBranchNode),
    Element(VElementNode),
    Lit(syn::Lit),
    Children(VChildrenNode),
    Empty
}

impl From<syn::Lit> for VNode {
    fn from(value: syn::Lit) -> Self {
        Self::Lit(value)
    }
}

#[derive(Clone)]
pub struct VBlockNode;

impl From<syn::Block> for VNode {
    fn from(value: syn::Block) -> VNode {
        VNode::Block(value)
    }
}

#[derive(Clone)]
pub enum VElementAttributeValue {
    Lit(syn::Lit),
    Block(syn::Block)
}

impl From<syn::Lit> for VElementAttributeValue {
    fn from(value: syn::Lit) -> Self {
        Self::Lit(value)
    }
}

impl From<syn::Block> for VElementAttributeValue {
    fn from(value: syn::Block) -> Self {
        Self::Block(value)
    }
}

#[derive(Clone)]
pub struct VElementAttribute {
    name: syn::Ident,
    value: VElementAttributeValue
}


impl VElementAttribute {
    pub fn new<N: Into<syn::Ident>, V: Into<VElementAttributeValue>>(name: N, value: V) -> Self {
        Self {
            name: name.into(),
            value: value.into()
        }
    }
}

#[derive(Default, Clone)]
pub struct VElementAttributes(Vec<VElementAttribute>);

impl VElementAttributes {
    pub fn push(&mut self, value: VElementAttribute) {
        self.0.push(value)
    }
}

#[derive(Clone)]
pub struct VElementNode {
    tag: Option<syn::Ident>,
    attrs: VElementAttributes,
    children: VChildrenNode
}

impl VElementNode {
    pub fn new(tag: Option<syn::Ident>, attrs: VElementAttributes, children: VChildrenNode) -> Self {
        Self{tag, attrs, children}
    }

    pub fn is_empty(&self) -> bool {
        self.tag.is_none()
    }

    pub fn is_html_element(&self) -> bool {
        if self.tag.is_none() {
            return false
        }

        let tag = self.tag.clone().unwrap();
        return tag.to_string().chars().into_iter().next().unwrap().is_lowercase();
    }
    
    pub fn is_component(&self) -> bool {
        if self.tag.is_none() {
            return false
        }

        let tag = self.tag.clone().unwrap();
        return tag.to_string().chars().into_iter().next().unwrap().is_uppercase();
    }
}

impl ToTokens for VElementNode {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if self.is_html_element() {
            
        }
    }
}

impl Into<VNode> for VElementNode {
    fn into(self) -> VNode {
        VNode::Element(self)
    }
}

#[derive(Clone, Default)]
pub struct VChildrenNode(Vec<VNode>);

impl VChildrenNode {
    pub fn push(&mut self, node: VNode) {
        self.0.push(node)
    }
}

#[derive(Clone)]
pub struct VBranchNode;

impl ToTokens for VNode {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            VNode::Element(element) => {
                element.to_tokens(tokens)
            },
            VNode::Block(_) => todo!(),
            VNode::Branch(_) => todo!(),
            VNode::Lit(_) => todo!(),
            VNode::Children(_) => todo!(),
            VNode::Empty => todo!(),
        }
    }
}

impl Parse for VNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lexer = lexer::VNodeLexer::new(input);
        let root_node = VNODE_PARSER.parse::<VElementNode, _>(lexer).map_err(|err| err.into_syn_error())?;
        Ok(root_node.into())
    }
}
