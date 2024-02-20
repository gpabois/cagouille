use std::{sync::Arc, any::Any};
use super::{Interaction, Inner};

#[derive(Clone)]
pub struct AnyInteraction(Arc<dyn Any + Sync + Send + 'static>);

impl<Matter> From<Interaction<Matter>> for AnyInteraction
where
    Matter: Sync + Send + 'static,
{
    fn from(value: Interaction<Matter>) -> Self {
        Self(value.0)
    }
}

impl PartialEq for AnyInteraction {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::addr_eq(Arc::as_ptr(&self.0), Arc::as_ptr(&other.0))
    }
}

impl AnyInteraction {
    pub fn downcast<Matter>(self) -> Option<Interaction<Matter>>
    where
        Matter: Sync + Send + 'static,
    {
        match self.0.downcast::<Inner<Matter>>() {
            Ok(inner) => Some(Interaction(inner)),
            Err(_) => None,
        }
    }
}

