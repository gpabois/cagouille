use std::borrow::BorrowMut;
use std::fmt::Debug;
use std::ops::DerefMut;
use std::pin::Pin;
use std::sync::Arc;
use async_std::channel::{unbounded, Receiver, Sender};
use async_std::sync::RwLock;
use std::future::Future;
use tokio::task::{JoinError, JoinHandle};

use super::wave::WaveSource;
use super::{Atom, Wave};
use super::interaction::{Interaction, LocalInteraction};
use super::tracker::Tracker;

pub(super) struct Current<Matter>(Arc<Option<Interaction<Matter>>>);

impl<Matter> Clone for Current<Matter> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<Matter> Current<Matter> {
    pub fn new() -> Self {
        Self(Arc::new(None))
    }

    pub fn set_interaction(&self, interaction: Option<Interaction<Matter>>) {
        let const_ptr = self.0.as_ref() as *const Option<Interaction<Matter>>;
        let mut_ptr = const_ptr as *mut Option<Interaction<Matter>>;
        
        unsafe {
            let ref_mut = &mut *mut_ptr;
            *ref_mut = interaction;
        }
    }

    pub fn interaction(&self) -> Option<Interaction<Matter>> {
        self.0.as_ref().as_ref().cloned()
    }
}

pub enum ReactorError {
    JoinError(JoinError),
    Nuked
}

impl Debug for ReactorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nuked => write!(f, "Nuked"),
            Self::JoinError(err) => write!(f, "{err}"),
        }
    }
}

pub enum LocalReaction<Matter> {
    /// Execute an interaction
    Interact(LocalInteraction<Matter>)
}

pub enum Reaction<Matter>{
    /// Execute an interaction
    Interact(Interaction<Matter>),
    /// Stop the reactor
    Nuke
}

pub struct ReactionEmitter(Pin<Box<JoinHandle<()>>>);

impl From<JoinHandle<()>> for ReactionEmitter {
    fn from(value: JoinHandle<()>) -> Self {
        Self(Box::pin(value))
    }
}

impl Future for ReactionEmitter {
    type Output = Result<(), ReactorError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        match self.0.as_mut().poll(cx) {
            std::task::Poll::Ready(val) => match val    {
                Ok(_) =>  std::task::Poll::Ready(Ok(())),
                Err(err) => std::task::Poll::Ready(Err(ReactorError::JoinError(err))),
            },
            _ => std::task::Poll::Pending
        }
    }
}

pub struct Reactor<Matter> { 
    sender: Sender<Reaction<Matter>>,
    current: Current<Matter>,
    join: Option<Arc<RwLock<JoinHandle<()>>>>
}

impl<Matter> Clone for Reactor<Matter> {
    fn clone(&self) -> Self {
        Self { sender: self.sender.clone(), current: self.current.clone(), join: self.join.clone() }
    }
}

pub async fn loop_reactor<Matter, F: (FnOnce(&Reactor<Matter>) -> Matter) + Send + 'static>(init_matter: F, foreign_recv: Receiver<Reaction<Matter>>, reactor: Reactor<Matter>) {
    let current = reactor.current.clone();
    let mut matter = init_matter(&reactor);

    let (local_sender, local_recv) = unbounded::<LocalReaction<Matter>>();
    
    loop {
        tokio::select! {
            Ok(reaction) = foreign_recv.recv() => {
                match reaction {
                    Reaction::Interact(interaction) => {
                        current.set_interaction(Some(interaction.clone()));
                        interaction.call(&mut matter);
                        current.set_interaction(None);
                    },
                    Reaction::Nuke => {
                        return;
                    }
                }
            },
            Ok(reaction) = local_recv.recv() => {

            }
        }
        match foreign_recv.recv().await {
            Ok(cmd) => {
                match cmd {
                    Reaction::Interact(interaction) => {
                        current.set_interaction(Some(interaction.clone()));
                        interaction.call(&mut matter);
                        current.set_interaction(None);
                    },
                    Reaction::Nuke => {
                        return;
                    }
                }
            },
            Err(_) => return,
        }
    }
}

impl<Matter: Send + 'static> Reactor<Matter> {
    pub fn new<F: (FnOnce(&Self) -> Matter) + Send + 'static>(init_matter: F) -> Self {
        let (sender, foreign_recv) = unbounded::<Reaction<Matter>>();
        
        let current = Current::<Matter>::new();

        let mut reactor = Self{
            sender, 
            current: current.clone(),
            join: None
        };

        let reac2 = reactor.clone();

        reactor.join = Some(Arc::new(RwLock::new(tokio::spawn(async move {
            loop_reactor(init_matter, foreign_recv, reac2).await;
        }))));
        
        return reactor;
    }

    fn new_tracker(&self) -> Tracker<Matter> {
        Tracker::new(self.clone())
    }

    /// Returns the current interaction
    pub(super) fn current_interaction(&self) -> Option<Interaction<Matter>> {
        self.current.interaction()
    }

    /// Add a reaction to occur in the reactor at the next possible loop.
    pub(super) fn react(&self, reaction: Reaction<Matter>) -> Result<ReactionEmitter, ReactorError> {
        if self.sender.is_closed() {
            return Err(ReactorError::Nuked);
        }

        let sender = self.sender.clone();

        return Ok(tokio::spawn(async move {
            sender
            .send(reaction)
            .await
            .unwrap();
        }).into())
    }

    /// Creates an interaction, a function rerun upon deps updates.
    pub fn interact<F: Fn(&mut Matter) + 'static + Send + Sync>(&self, interaction: F) -> Result<ReactionEmitter, ReactorError> {
        let interaction = Interaction::new(interaction);
        self.react(Reaction::Interact(interaction))
    }
    
    /// Creates a wave, a computed reactive value.
    pub fn wave<D: 'static, F: Fn(&Matter) -> D>(&self, init: D, f: F) -> Wave<D> {
        let wave_src = WaveSource::new(init, self.new_tracker());
        let wave = wave_src.to_wave();
        let interaction = wave_src.to_wave();
        return wave;
    }

    /// Creates an atom, a reactive value.
    pub fn atom<D>(&self, value: D) -> Atom<Matter, D> {
        Atom::new(value, self.new_tracker())
    }

    /// Stop the reactor
    pub fn nuke(&self) -> Result<ReactionEmitter, ReactorError> {
        self.react(Reaction::Nuke)
    }

    pub async fn wait_for_nuke(&self) {
        self.join
        .clone()
        .unwrap()
        .write()
        .await
        .borrow_mut()
        .deref_mut()
        .await
        .unwrap();
    }
}

