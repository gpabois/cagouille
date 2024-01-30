pub mod error;
pub mod event;
pub mod vdom;
pub mod component;
pub mod futures;
pub mod df;
pub mod utils;
pub mod reactor;

pub mod prelude {
    pub use cagouille_macro::component;
    pub use cagouille_macro::render;
    pub use cagouille_macro::Differentiable;

    pub(crate) use cagouille_macro::Self_Differentiable;
}