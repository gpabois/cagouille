use lazy_static::lazy_static;
use syn::Block;
use yalp::parser::rule::ParserRuleSet;

use crate::html::{VElementNode, VNode};

use super::symbols::Class;
use super::VDomRendererParserDef;

lazy_static!{
    /*
        Root -> Element
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

    static ref RULES: ParserRuleSet<VDomRendererParserDef> = ParserRuleSet::new()
    .add( // Root -> Element
        Class::Root, [Class::Element],
        &|mut syms| {
            Ok(syms.remove(0).value)
        }
    )
    .add( // Node -> Element
        Class::Node, [Class::Element],
        &|mut syms| {
            let el: VElementNode = syms.remove(0).try_into()?;
            Ok(VNode::Element(el).into())
        } 
    )
    .add( // Node -> block
        Class::Node, [Class:Block],
        &|mut syms| {
            let bck = syms.remove(0).try_into()?;
        }
    )
    .to_owned();
}