pub mod error;
pub mod event;
pub mod vdom;
pub mod component;
pub mod futures;

pub mod prelude {
    pub use cagouille_macro::component;
    pub use cagouille_macro::render;
}