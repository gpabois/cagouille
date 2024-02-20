use std::{rc::Rc, any::Any};
use super::{Interaction, Inner};

#[derive(Clone)]
pub struct AnyInteraction(Rc<dyn Any + 'static>);

impl<Matter> From<Interaction<Matter>> for AnyInteraction
where
    Matter: 'static,
{
    fn from(value: Interaction<Matter>) -> Self {
        Self(value.0)
    }
}

impl PartialEq for AnyInteraction {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::addr_eq(Rc::as_ptr(&self.0), Rc::as_ptr(&other.0))
    }
}

impl AnyInteraction {
    pub fn downcast<Matter>(self) -> Option<Interaction<Matter>>
    where
        Matter: 'static,
    {
        match self.0.downcast::<Inner<Matter>>() {
            Ok(inner) => Some(Interaction(inner)),
            Err(_) => None,
        }
    }
}

