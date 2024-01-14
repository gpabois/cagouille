use std::fmt::Debug;

use proc_macro2::Span;
use syn::{parse::ParseStream, Token, spanned::Spanned};
use yalp::{lexer::{LexerError, traits::LexerSymbol}, parser::traits::Terminal, symbol::Symbol};

pub struct VNodeLexer<'a> {
    input: ParseStream<'a>,
    exhausted: bool
}

impl<'a> VNodeLexer<'a> {
    pub fn new(input: ParseStream<'a>) -> Self {
        Self{input, exhausted: false}
    }

    fn consume(&mut self) -> Result<VNodeToken, LexerError> {
        VNodeToken::next(self.input)
    }
}

impl<'a> yalp::lexer::traits::Lexer for VNodeLexer<'a> {
    type Symbol = VNodeToken;
}

impl<'a> Iterator for VNodeLexer<'a> {
    type Item = Result<VNodeToken, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {

        if self.exhausted {
            return None;
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

#[derive(Debug, Clone, PartialEq)]
pub enum VNodeTokenType {
    LeftAngle,
    ClosingLeftAngle,
    SingleRightAngle,
    RightAngle,
    Lit,
    Path,
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
    Path(syn::Path),
    Lit(syn::Lit),
    Ident(syn::Ident),
    Equal(syn::Token![=]),
    Block(syn::Block),
    EOS
}

impl Debug for VNodeToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get_type().fmt(f)
    }
}

impl Terminal for VNodeToken {}

impl LexerSymbol for VNodeToken {
    type Type = VNodeTokenType;

    fn span(&self) -> yalp::lexer::TokenSpan {
        match self {
            VNodeToken::LeftAngle(tok) => tok.span.into(),
            VNodeToken::ClosingLeftAngle(tok, _) => tok.span.into(),
            VNodeToken::SingleRightAngle(tok, _) => tok.span.into(),
            VNodeToken::RightAngle(tok) => tok.span.into(),
            VNodeToken::Lit(tok) => tok.span().into(),
            VNodeToken::Path(tok) => tok.span().into(),
            VNodeToken::Ident(tok) => tok.span().into(),
            VNodeToken::Equal(tok) => tok.span.into(),
            VNodeToken::Block(tok) => tok.span().into(),
            VNodeToken::EOS => Span::call_site().into()
        }
    }
}

impl yalp::symbol::Symbol<VNodeTokenType> for VNodeToken {

    fn get_type(&self) -> VNodeTokenType {
        match self {
            VNodeToken::LeftAngle(_) => VNodeTokenType::LeftAngle,
            VNodeToken::ClosingLeftAngle(_, _) => VNodeTokenType::ClosingLeftAngle,
            VNodeToken::SingleRightAngle(_, _) => VNodeTokenType::SingleRightAngle,
            VNodeToken::RightAngle(_) => VNodeTokenType::RightAngle,
            VNodeToken::Lit(_) => VNodeTokenType::Lit,
            VNodeToken::Ident(_) => VNodeTokenType::Ident,
            VNodeToken::Path(_) => VNodeTokenType::Path,
            VNodeToken::Equal(_) => VNodeTokenType::Equal,
            VNodeToken::Block(_) => VNodeTokenType::Block,
            VNodeToken::EOS => VNodeTokenType::EOS,
        }
    }
}

impl VNodeToken {
    pub fn next(input: ParseStream) -> Result<Self, LexerError> {
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
        
        if let Ok(path) = input.fork().parse::<syn::Path>() {
            if let Some(ident) = path.get_ident().cloned() {
                return Ok(Self::Ident(ident));
            }

            return Ok(Self::Path(path));
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