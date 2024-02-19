pub mod component;
pub mod df;
pub mod dom;
pub mod error;
pub mod event;
pub mod futures;
pub mod utils;
pub mod vdom;

pub mod prelude {
    pub use cagouille_macro::component;
    pub use cagouille_macro::render;
    pub use cagouille_macro::Differentiable;

    pub(crate) use cagouille_macro::Self_Differentiable;
}
