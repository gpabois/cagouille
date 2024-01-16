use lazy_static::lazy_static;
use syn::parse::ParseStream;
use yalp::parser::{lr::LrParser, traits::Parser, ParserError};
use rules::RULES;

use self::lexer::Lexer;

use super::VElementNode;

mod symbols;
mod rules;
mod lexer;

pub(self) struct ParserDef;

impl yalp::symbol::traits::SymbolDefinition for ParserDef {
    type Class = symbols::Class;
    type Value = symbols::Value;
}

lazy_static!{
    pub(self) static ref PARSER: LrParser<'static, ParserDef> = LrParser::generate(&RULES);
}

/// Parse the virtual dom renderer script
pub fn parse<'a>(stream: ParseStream<'a>) -> Result<VElementNode, ParserError> {
    let lexer = Lexer::new(stream);
    let el: VElementNode = PARSER.parse(lexer)?;
    Ok(el)
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_parser() {
        let table = &super::PARSER.table;
        println!("{:?}", table);
    }
}