use crate::dom::Node;
use seeded_random::{Random, Seed};

/// Node's scope
pub struct Scope {
    /// Node key
    pub key: super::VNodeKey,
    // DOM node
    pub node: Option<Node>,
    /// The rng generator for children node keys.
    pub(self) rng: Random,
}

impl Default for Scope {
    fn default() -> Self {
        Self::new_root()
    }
}

impl Clone for Scope {
    fn clone(&self) -> Self {
        Self::new(self.key.0)
    }
}

impl Scope {
    /// New root scope
    pub fn new_root() -> Self {
        Self {
            key: Default::default(),
            el: None,
            rng: Random::from_seed(Seed::unsafe_new(0)),
        }
    }

    /// Create a new rendering scope
    pub fn new(key: u32) -> Self {
        Self {
            key: key.into(),
            el: None,
            rng: Random::from_seed(Seed::unsafe_new(key.into())),
        }
    }

    /// Create a child scope
    pub fn child(&self) -> Scope {
        Self::new(self.rng.u32())
    }
}
