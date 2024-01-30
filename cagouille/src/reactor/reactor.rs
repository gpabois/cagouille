
use std::sync::Arc;
use async_std::channel::{unbounded, Sender};

use super::Ray;
use super::interaction::Interaction;
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

pub enum Reaction<Matter>{
    RunInteraction(Interaction<Matter>)
}

#[derive(Clone)]
pub struct Reactor<Matter> { 
    sender: Sender<Reaction<Matter>>,
    current: Current<Matter>,
}

impl<Matter: Send> Reactor<Matter> {
    pub fn new<F: FnOnce(&Self) -> Matter>(init_matter: F) -> Self {
        let (sender, recv) = unbounded::<Reaction<Matter>>();
        
        let current = Current::<Matter>::new();

        let reactor = Self{
            sender, 
            current: current.clone()
        };

        let matter = init_matter(&reactor);

        tokio::spawn(async move {
            loop {
                match recv.recv().await {
                    Ok(cmd) => {
                        match cmd {
                            Reaction::RunInteraction(interaction) => {
                                current.set_interaction(Some(interaction.clone()));
                                interaction.call(&mut matter);
                                current.set_interaction(None);
                            },
                        }
                    },
                    Err(_) => return,
                }
            }
        });
        
        return reactor;
    }

    fn new_tracker(&self) -> Tracker<Matter> {
        Tracker::new(self.current.clone())
    }

    pub fn use_ray<D>(&self, value: D) -> Ray<Matter, D> {
        Ray::new(value, self.new_tracker())
    }
}

