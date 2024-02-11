use std::ops::Deref;

use tokio::sync::watch;
use crate::{interface::Signal, Context, Interaction};

/// A measure from the reactor
/// This allows for external systems to receive updated 
/// upon reactor's state change
pub struct Measure<D>(watch::Receiver<D>) where D: Sync + Send + 'static;

impl<D> Measure<D> where D: Sync + Send + 'static {
    /// Create a new measure.
   pub fn new<Matter, F>(init: D, f: F, signal: Signal<Matter>) -> Self 
   where F: Fn(Context<Matter>) -> D  + Sync + Send + 'static
   {
        let (sender, recver) = watch::channel(init);
    
        signal.send(Interaction::new(move |ctx| {
            sender.send(f(ctx)).unwrap();
        }));
        
        Self(recver)
    }

    pub async fn changed(&mut self) {
        self.0.changed().await.unwrap()
    }

    pub fn borrow(&self) -> Ref<'_, D> {
        Ref(self.0.borrow())
    }
}

impl<D> Measure<D> where D: Sync + Send + ToOwned<Owned=D> + 'static {
    pub fn to_owned(&self) -> D {
        self.borrow().deref().to_owned()
    }    
}

pub struct Ref<'a, D>(watch::Ref<'a, D>);

impl<'a, D> Deref for Ref<'a, D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

