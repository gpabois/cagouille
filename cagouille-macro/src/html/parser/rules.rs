use lazy_static::lazy_static;

use yalp::parser::rule::ParserRuleSet;
use yalp::symbol::traits::Symbol;

use crate::html::{VElementNode, VNode, VChildrenNode, VElementAttributes, VElementAttribute};

use super::symbols::{Class, OpenTag, SingleTag, CloseTag};
use super::ParserDef;

lazy_static!{
    /*  Current grammar rules (v0.1):

        Root -> Element
        Node -> Element
        Node -> block
        Node -> lit
        Element -> SingleTag
        Element -> OpenTag Children CloseTag
        Element -> OpenTag CloseTag
        OpenTag -> < ident >
        OpenTag -> < ident Attrs >
        CloseTag -> </ ident >
        SingleTag -> < ident Attrs />
        SingleTag -> < ident />
        Children -> Children Node
        Children -> Node
        Attrs -> Attrs Attr
        Attrs -> Attr
        Attr -> ident = block
        Attr -> ident = lit
    */

    /// Virtual DOM's renderer grammar rules
    pub(super) static ref RULES: ParserRuleSet<ParserDef> = ParserRuleSet::new()
    .add( // Root -> Element
        Class::Root, [Class::Element],
        &|mut syms| {
            Ok(syms.remove(0).value)
        }
    )
    .add( // Node -> Element
        Class::Node, [Class::Element],
        &|mut syms| {
            let el: VElementNode = syms.remove(0).into_value()?;
            Ok(VNode::Element(el).into())
        } 
    )
    .add( // Node -> block
        Class::Node, [Class::Block],
        &|mut syms| {
            let bck = syms.remove(0).into_value()?;
            Ok(VNode::Block(bck).into())
        }
    )
    .add( // Node -> lit 
        Class::Node, [Class::Lit],
        &|mut syms| {
            let lit = syms.remove(0).into_value()?;
            Ok(VNode::Lit(lit).into())
        }       
    )
    .add( // Element -> SingleTag 
        Class::Element, [Class::SingleTag],
        &|mut syms| {
            let SingleTag{tag, attrs} = syms.remove(0).into_value()?;
            
            Ok(VElementNode {
                tag,
                attrs,
                children: VChildrenNode::default()
            }.into())
        }
    )
    .add( // Element -> OpenTag Children CloseTag
        Class::Element, [Class::OpenTag, Class::ElementChildren, Class::CloseTag],
        &|mut syms| {
            let OpenTag { tag, attrs } = syms.remove(0).into_value()?;
            let children: VChildrenNode = syms.remove(0).into_value()?;
            let CloseTag {tag: _} = syms.remove(0).into_value()?;
            
            Ok(VElementNode {
                tag,
                attrs,
                children
            }.into())
        }
    )
    .add( // Element -> OpenTag CloseTag
        Class::Element, [Class::OpenTag, Class::CloseTag],
        &|mut syms| {
            let OpenTag { tag, attrs } = syms.remove(0).into_value()?;
            let CloseTag {tag: _} = syms.remove(0).into_value()?;
            Ok(VElementNode {
                tag,
                attrs,
                children: VChildrenNode::default()
            }.into())
        }
    )
    .add( // OpenTag -> < ident >
        Class::OpenTag, [Class::LeftAngle, Class::Ident, Class::RightAngle],
        &|mut syms| {
            let ident: syn::Ident = syms.remove(1).into_value()?;
            Ok(OpenTag{ tag: Some(ident), attrs: VElementAttributes::default()}.into())
        }
    )
    .add( // OpenTag -> < ident Attrs >
        Class::OpenTag, [Class::LeftAngle, Class::Ident, Class::ElementAttributes, Class::RightAngle],
        &|mut syms| {
            let ident: syn::Ident = syms.remove(1).into_value()?;
            let attrs: VElementAttributes = syms.remove(1).into_value()?;
            Ok(OpenTag{ tag: Some(ident), attrs}.into())
        }
    )
    .add( // CloseTag -> </ ident >
        Class::CloseTag, [Class::ClosingLeftAngle, Class::Ident, Class::RightAngle],
        &|mut syms| {
            let ident: syn::Ident = syms.remove(1).into_value()?;
            Ok(CloseTag{tag: Some(ident)}.into())
        }

    )
    .add( // SingleTag -> < ident Attrs />
        Class::SingleTag, [Class::LeftAngle, Class::Ident, Class::ElementAttributes, Class::SingleRightAngle],
        &|mut syms| {
            let ident: syn::Ident = syms.remove(1).into_value()?;
            let attrs: VElementAttributes = syms.remove(1).into_value()?;
            Ok(SingleTag{tag: Some(ident), attrs}.into())
        }   
    )
    .add( // SingleTag -> < ident />
        Class::SingleTag, [Class::LeftAngle, Class::Ident, Class::SingleRightAngle],
        &|mut syms| {
            let ident: syn::Ident = syms.remove(1).into_value()?;
            Ok(SingleTag{tag: Some(ident), attrs: VElementAttributes::default()}.into())
        }   
    )
    .add( // Children -> Children Node
        Class::ElementChildren, [Class::ElementChildren, Class::Node],
        &|mut syms| {
            let mut children: VChildrenNode = syms.remove(0).into_value()?;
            let node: VNode = syms.remove(0).into_value()?;
            children.push(node);
            Ok(children.into())
        }
    )
    .add( // Children -> Node
        Class::ElementChildren, [Class::Node],
        &|mut syms| {
            let node: VNode = syms.remove(0).into_value()?;
            Ok(VChildrenNode::from_iter([node]).into())
        }
    )
    .add( // Attrs -> Attrs Attr
        Class::ElementAttributes, [Class::ElementAttributes, Class::ElementAttribute],
        &|mut syms| {
            let mut attrs: VElementAttributes = syms.remove(0).into_value()?;
            let attr: VElementAttribute = syms.remove(0).into_value()?;
            attrs.push(attr);
            Ok(attrs.into())
        }
    )
    .add( // Attrs -> Attr
        Class::ElementAttributes, [Class::ElementAttribute],
        &|mut syms| {
            let attr: VElementAttribute = syms.remove(0).into_value()?;
            Ok(VElementAttributes::from_iter([attr]).into())
        }
    )
    .add( // Attr -> ident = block
        Class::ElementAttribute, [Class::Ident, Class::Equal, Class::Block],
        &|mut syms| {
            let ident: syn::Ident = syms.remove(0).into_value()?;
            let block: syn::Block = syms.remove(1).into_value()?;
            Ok(VElementAttribute::new(ident, block).into())
        }
    )
    .add( // Attr -> ident = lit
        Class::ElementAttribute, [Class::Ident, Class::Equal, Class::Block],
        &|mut syms| {
            let ident: syn::Ident = syms.remove(0).into_value()?;
            let lit: syn::Lit = syms.remove(1).into_value()?;
            Ok(VElementAttribute::new(ident, lit).into())
        }
    )
    .to_owned();
}