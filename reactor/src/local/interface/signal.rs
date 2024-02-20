use tokio::sync::mpsc;
use crate::local::{Reaction, AnyReaction};

pub struct SignalRx(mpsc::UnboundedReceiver<AnyReaction>);

impl SignalRx {
    /// Poll reaction
    pub async fn poll(&mut self) -> Option<AnyReaction> {
        self.0.recv().await
    }

    pub async fn poll_downcast<Matter>(&mut self) -> Option<Reaction<Matter>> 
    where Matter:'static
    {
        if let Some(any) = self.poll().await {
            return any.downcast::<Matter>()
        }

        None
    }
}

#[derive(Clone)]
/// Type-erased signal
pub struct Signal(mpsc::UnboundedSender<AnyReaction>);

impl Signal {
    pub fn create() -> (Signal, SignalRx) {
        let (tx, rx) = mpsc::unbounded_channel::<AnyReaction>();
        (Signal(tx), SignalRx(rx))
    }
}

impl PartialEq for Signal {
    fn eq(&self, other: &Self) -> bool {
        self.0.same_channel(&other.0)
    }
}

impl Signal {
    /// Send a reaction to the reactor
    pub fn send<I: Into<AnyReaction>>(&self, into_reaction: I) {
        self.0.send(into_reaction.into()).unwrap();
    }
}