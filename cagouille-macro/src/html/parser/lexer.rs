use syn::{parse::ParseStream, Token};
use yalp::{lexer::LexerError, symbol::Sym};

use crate::html::parser::symbols::Value;

use super::{ParserDef, symbols::Class};

pub type LexerDef = ParserDef;

pub(super) struct Lexer<'a> {
    input: ParseStream<'a>,
    exhausted: bool
}

impl<'a> Lexer<'a> {
    pub fn new(input: ParseStream<'a>) -> Self {
        Self{input, exhausted: false}
    }

    fn consume(&mut self) -> Result<(Class, Value), LexerError> {
        VNodeToken::next(self.input)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Sym<LexerDef>, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {

        if self.exhausted {
            return None;
        }

        match self.consume() {
            Ok((Class::EOS, _)) => {
                return Some(Ok(Sym::new(
                    self.input.span(),
                    Class::EOS,
                    Value::EOS
                )))
            },
            Err(err) => {
                self.exhausted = true;
                return Some(Err(err));
            },
            Ok((class, value)) => {
                Some(Ok(Sym::new(
                    self.input.span(),
                    class,
                    value
                )))
            }
        }
    }
}

#[derive(Clone)]
pub struct VNodeToken;

impl VNodeToken {
    pub fn next(input: ParseStream) -> Result<(Class, Value), LexerError> {
        if input.is_empty() {
            return Ok((Class::EOS, Value::EOS));
        }

        if input.peek(Token![<]) && input.peek2(Token![/]) {
            return Ok((
                Class::ClosingLeftAngle, 
                Value::ClosingLeftAngle ( input.parse()?, input.parse()?)
            ));
        }

        if input.peek(Token![<]) {
            return Ok((
                Class::LeftAngle, 
                Value::LeftAngle(input.parse()?)
            ));
        }

        if input.peek(Token![/]) && input.peek2(Token![>]) {
            return Ok(
                (Class::SingleRightAngle, Value::SingleRightAngle(input.parse()?, input.parse()?))
            );
        }

        if input.peek(Token![>]) {
            return Ok(
                (Class::RightAngle, Value::RightAngle(input.parse()?))
            );
        }
        
        if let Ok(path) = input.fork().parse::<syn::Path>() {
            if let Some(ident) = path.get_ident().cloned() {
                return Ok(
                    (Class::Ident, Value::Ident(ident))
                );
            }

            return Ok(
                (Class::Path, Value::Path(path))
            );
        }
        
        if input.peek(Token![=]) {
            return Ok(
                (Class::Equal, Value::Equal(input.parse()?))
            );
        }

        if input.peek(syn::Lit) {
            return Ok(
                (Class::Lit, Value::Lit(input.parse()?))
            );
        }

        if input.fork().parse::<syn::Block>().is_ok() {
            return Ok(
                (Class::Block, Value::Block(input.parse()?))
            );
        }

        Err(
            LexerError {
                span: input.span().into(),
                message: "unknown symbol".to_string()
            }
        )
    }
}