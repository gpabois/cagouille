use std::{ops::Deref, time::Duration};

use crate::{interface::Signal, Context, Interaction};
use tokio::{sync::watch, time};

/// A measure from the reactor
/// This allows for external systems to receive updated
/// upon reactor's state change
pub struct Measure<D>(watch::Receiver<D>)
where
    D: Sync + Send + 'static;

impl<D> Measure<D>
where
    D: Sync + Send + 'static,
{
    /// Create a new measure.
    pub(crate) fn new<Matter, F>(init: D, f: F, signal: Signal) -> Self
    where
        F: Fn(Context<Matter>) -> D + Sync + Send + 'static,
        Matter: 'static,
    {
        let (sender, recver) = watch::channel(init);

        signal.send(Interaction::new(move |ctx| {
            sender.send(f(ctx)).unwrap();
        }));

        Self(recver)
    }

    pub async fn changed(&mut self) {
        self.0.changed().await.unwrap();
        self.0.borrow_and_update();
    }

    pub async fn changed_or_timeout(&mut self, d: Duration) {
        let sleep = time::sleep(d);
        tokio::pin!(sleep);


        tokio::select! {
            _ = self.changed() => {},
            _ = &mut sleep => {}
        }
    }

    pub fn borrow(&self) -> Ref<'_, D> {
        Ref(self.0.borrow())
    }
}

impl<D> Measure<D>
where
    D: Sync + Send + ToOwned<Owned = D> + 'static,
{
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
