use std::{pin::Pin, rc::Rc, sync::Arc};


use super::{interaction::LocalInteraction, tracker::Tracker, Interaction};

pub struct WaveSource<Matter, D> {
    pub(super) value: Pin<Arc<D>>,
    pub(super) tracker: Tracker<Matter>  
}

impl<Matter, D> WaveSource<Matter, D> 
where Matter: Send + 'static, D: 'static
{
    pub fn to_wave(&self) -> Wave<D> {
        Wave(self.value.clone())
    }

    pub fn into_interaction<F: (Fn(&Matter) -> D) + Send + Sync + 'static>(self, f: F) -> LocalInteraction<Matter> {
        LocalInteraction::new(move |matter| {
            let new_value  = f(matter);
            
            let const_ptr = self.value.as_ref().get_ref() as *const D;
            let mut_ptr = const_ptr as *mut D;
            let mut_ref_ptr = unsafe{&mut *mut_ptr};
            *mut_ref_ptr = new_value;

            self.tracker.trigger();
        })
    }
}

pub struct Wave<D>(Pin<Arc<D>>);

impl<Matter, D> WaveSource<Matter, D> {
    pub fn new(init: D, tracker: Tracker<Matter>) -> Self {
        Self {
            value: Arc::pin(init), tracker
        }
    }
}