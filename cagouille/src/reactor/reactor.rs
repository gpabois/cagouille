use async_std::sync::{RwLock, Mutex, MutexGuard};
use std::sync::Arc;

use super::effect::Effect;


struct Inner<'comp> {
    current_effect: RwLock<Option<Effect<'comp>>>,
    current_effect_mutex: Mutex<()>
}

impl<'comp> Inner<'comp> {
    pub fn new() -> Self {
        Self {
            current_effect: Default::default(),
            current_effect_mutex: Mutex::new(())
        }
    }
}

#[derive(Clone)]
pub struct EffectLock<'a>(Arc<MutexGuard<'a, ()>>);

impl<'a> From<MutexGuard<'a, ()>> for EffectLock<'a> {
    fn from(val: MutexGuard<'a, ()>) -> Self {
        Self(Arc::new(val))
    }
}

#[derive(Clone)]
pub struct Reactor<'comp>(Arc<Inner<'comp>>);

impl<'comp> Reactor<'comp> {
    pub fn new() -> Self {
        Self(Arc::new(Inner::new()))
    }

    /// Lock the current effect until it is dropped.
    pub async fn lock_current_effect(&self, effect: &Effect<'comp>) -> EffectLock<'_> {
        let lock: EffectLock<'_> = self.0.lock_effect.lock().await.into();
        *self.0.current_effect.write().await = Some(effect.clone());
        return lock;
    }

    pub async fn current_effect(&self) -> Option<Effect<'comp>> {
        self.0.current_effect.read().await.clone()
    }
}

