#[derive(Clone, Debug)]
pub enum Span {
    ProcMacroSpan(proc_macro2::Span),
    Interval{from: usize, to: usize}
}

impl Into<proc_macro2::Span> for Span {
    fn into(self) -> proc_macro2::Span {
        match self {
            Self::ProcMacroSpan(span) => span,
            _ => unreachable!("not a macro span")
        }
    }
}

impl From<proc_macro2::Span> for Span {
    fn from(value: proc_macro2::Span) -> Self {
        Self::ProcMacroSpan(value)
    }
}

impl From<usize> for Span {
    fn from(value: usize) -> Self {
        Self::Interval{from: value, to: value}
    }
}