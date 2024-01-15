mod symbols;
mod rules;

pub struct VDomRendererParserDef;

impl yalp::symbol::traits::SymbolDefinition for VDomRendererParserDef {
    type Class = symbols::Class;
    type Value = symbols::Value;
}
