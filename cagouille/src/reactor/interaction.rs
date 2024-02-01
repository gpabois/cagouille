use std::{rc::Rc, sync::Arc};

pub struct LocalInteraction<Matter>(Rc<dyn Fn(&mut Matter) + 'static>);

impl<Matter> LocalInteraction<Matter> {
    pub fn new<F: Fn(&mut Matter) + 'static>(value: F) -> Self {
        Self(Rc::new(value))
    }
}

pub struct Interaction<Matter>(Arc<dyn Fn(&mut Matter) + Sync + Send + 'static>);

impl<Matter> Clone for Interaction<Matter>
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<Matter> Interaction<Matter> {
    pub fn new<F: Fn(&mut Matter) + Sync + Send + 'static>(value: F) -> Self {
        Self(Arc::new(value))
    }

    pub fn call(&self, matter: &mut Matter) {
        self.0(matter)
    }
}