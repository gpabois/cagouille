use syn::{parse::ParseStream, Token};

pub struct VNodeLexer<'a> {
    input: ParseStream<'a>,
    exhausted: bool
}

impl<'a> VNodeLexer<'a> {
    pub fn new(input: ParseStream) -> Self {
        Self{input, exhausted: false}
    }

    fn consume(&mut self) -> syn::Result<VNodeToken> {
        VNodeToken::next(self.input)
    }
}

impl<'a> yalp::lexer::traits::Lexer for VNodeLexer<'a> {
    type Symbol = VNodeToken;
    type Error = syn::Error;
}

impl<'a> Iterator for VNodeLexer<'a> {
    type Item = syn::Result<VNodeToken>;

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

impl yalp::symbol::Symbol for VNodeToken {
    type Type = VNodeTokenType;

    fn get_type(&self) -> Self::Type {
        match self {
            VNodeToken::LeftAngle(_) => Self::Type::LeftAngle,
            VNodeToken::ClosingLeftAngle(_, _) => Self::Type::ClosingLeftAngle,
            VNodeToken::SingleRightAngle(_, _) => Self::Type::SingleRightAngle,
            VNodeToken::RightAngle(_) => Self::Type::RightAngle,
            VNodeToken::Lit(_) => Self::Type::Lit,
            VNodeToken::Ident(_) => Self::Type::Ident,
            VNodeToken::Equal(_) => Self::Type::Equal,
            VNodeToken::Block(_) => Self::Type::Block,
            VNodeToken::EOS => Self::Type::EOS,
        }
    }
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