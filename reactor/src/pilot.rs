
use tokio::sync::watch;
use tokio::task::JoinHandle;

use crate::interface::Signal;

/// The reactor's core pilot
pub struct Pilot<Matter> {
    /// The reaction  to send to the core
    signal: Signal<Matter>, 
    /// The shutdown button
    shutdown: watch::Sender<bool>,
    /// Join handle for the core's loop.
    join: Option<JoinHandle<()>>
}

impl<Matter> Pilot<Matter> {
    /// Get a signal
    pub fn get_signal(&self) -> Signal<Matter> {
        self.signal.clone()
    }
    
    /// Shutdown the reactor, panics if it has already be shutdown
    pub async fn shutdown(mut self) {
        self.shutdown.send(true).unwrap();
        let mut join: Option<JoinHandle<()>> = None;
        std::mem::swap(&mut self.join, &mut join);
        if let Some(join) = join {
            join.await.unwrap();
        }
        drop(self);
    }
}

impl<Matter> Drop for Pilot<Matter> {
    fn drop(&mut self) {
        if self.shutdown.is_closed() {
            return;
        }
        self.shutdown.send(true).unwrap(); 
    }
}

impl<Matter> Pilot<Matter> {
    pub fn new(join: JoinHandle<()>, 
              signal: Signal<Matter>, 
               shutdown: watch::Sender<bool>) -> Self {
        Self{join: Some(join), signal, shutdown}
    }
}
