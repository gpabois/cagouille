use crate::span::Span;

#[derive(Clone, Debug)]
pub struct LexerError {
    pub span:    Span,
    pub message: String
}

impl From<syn::Error> for LexerError {
    fn from(value: syn::Error) -> Self {
        Self {
            span: value.span().into(),
            message:  value.to_string()
        }
    }
}

impl Into<syn::Error> for LexerError {
    fn into(self) -> syn::Error {
        syn::Error::new(self.span.into(), self.message)
    }
}
