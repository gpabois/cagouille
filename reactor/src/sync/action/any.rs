use std::any::Any;
use super::Action;

pub struct AnyAction(pub(super) Box<dyn Any + Sync + Send>);

impl AnyAction {
    pub fn downcast<Matter>(self) -> Option<Action<Matter>>
    where
        Matter: 'static,
    {
        match self.0.downcast::<Action<Matter>>() {
            Ok(boxed_action) => Some(*boxed_action),
            Err(_) => None,
        }
    }
}

impl<Matter> From<Action<Matter>> for AnyAction 
where Matter: 'static
{
    fn from(value: Action<Matter>) -> Self {
        Self(Box::new(value))
    }
}
