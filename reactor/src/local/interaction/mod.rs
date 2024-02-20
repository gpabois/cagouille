use std::rc::Rc;
use crate::local::Context;

mod any;
mod bound;

pub use any::AnyInteraction;
pub use bound::BoundInteraction;

type BoxedFunc<Matter> = Box<dyn Fn(Context<'_, Matter>) + 'static>;
struct Inner<Matter>(BoxedFunc<Matter>);

/// An interaction is a function run by the reactor's core
pub struct Interaction<Matter>(Rc<Inner<Matter>>);

impl<Matter> PartialEq for Interaction<Matter> {
    fn eq(&self, other: &Self) -> bool {
        Rc::as_ptr(&self.0) == Rc::as_ptr(&other.0)
    }
}

impl<Matter> Clone for Interaction<Matter> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<Matter> Interaction<Matter> {
    /// Create a new interaction
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(Context<'_, Matter>) + 'static,
    {
        Self(Rc::new(Inner(Box::new(f))))
    }

    /// Execute the interaction
    pub fn execute(&self, context: Context<'_, Matter>) {
        self.0 .0(context);
    }
}
