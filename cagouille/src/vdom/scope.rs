use std::sync::Arc;

use async_std::sync::RwLock;
use seeded_random::{Random, Seed};


/// Node's scope
pub struct Scope {
    /// Node key
    pub id: super::VNodeKey,

    /// The rng generator for children node keys.
    pub(self) rng: Random,
}

#[derive(Clone)]
pub struct ScopeRef(pub(super) Arc<RwLock<Scope>>);

impl From<Scope> for ScopeRef {
    fn from(value: Scope) -> Self {
        Self(Arc::new(RwLock::new(value)))
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new_root()
    }
}

impl Clone for Scope {
    fn clone(&self) -> Self {
        Self::new(self.id.0)
    }
}

impl Scope {
    /// New root scope
    pub fn new_root() -> Self {
        Self { id: Default::default(), rng: Random::from_seed(Seed::unsafe_new(0)) }
    }
    
    /// Create a new rendering scope
    pub fn new(id: u32) -> Self {
        Self { 
            id: id.into(), 
            rng: Random::from_seed(Seed::unsafe_new(id.into()))  
        }
    }

    /// Create a child scope
    pub fn new_child_scope(&self) -> Scope {
        Self::new(self.rng.u32())
    }
}