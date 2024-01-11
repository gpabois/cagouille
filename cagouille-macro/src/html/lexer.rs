use proc_macro2::Span;
use syn::{parse::ParseStream, Token, spanned::Spanned};
use yalp::{lexer::{LexerError, traits::LexerSymbol}, parser::traits::TerminalSymbol};

pub struct VNodeLexer<'a> {
    input: ParseStream<'a>,
    exhausted: bool
}

impl<'a> VNodeLexer<'a> {
    pub fn new(input: ParseStream) -> Self {
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

#[derive(Clone, PartialEq)]
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

impl TerminalSymbol for VNodeToken {
    type Type = VNodeTokenType;
}

impl LexerSymbol for VNodeToken {
    type Type = VNodeTokenType;

    fn span(&self) -> yalp::lexer::TokenSpan {
        match self {
            VNodeToken::LeftAngle(tok) => tok.span.into(),
            VNodeToken::ClosingLeftAngle(tok, _) => tok.span.into(),
            VNodeToken::SingleRightAngle(tok, _) => tok.span.into(),
            VNodeToken::RightAngle(tok) => tok.span.into(),
            VNodeToken::Lit(tok) => tok.span().into(),
            VNodeToken::Ident(tok) => tok.span().into(),
            VNodeToken::Equal(tok) => tok.span.into(),
            VNodeToken::Block(tok) => tok.span().into(),
            VNodeToken::EOS => Span::call_site().into()
        }
    }
}

impl VNodeToken {
    pub fn expect_ident(self) -> syn::Ident {
        match self {
            Self::Ident(ident) => ident,
            _ => unreachable!("expecting ident")
        }
    }

    pub fn expect_lit(self) -> syn::Lit {
        match self {
            Self::Lit(lit) => lit,
            _ => unreachable!("expecting block")
        }
    }

    pub fn expect_block(self) -> syn::Block {
        match self {
            Self::Block(block) => block,
            _ => unreachable!("expecting block")
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