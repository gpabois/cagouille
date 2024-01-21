
pub struct EventSlot<'a, E: traits::Event>(Box<dyn Fn(E::Payload) + 'a>);

impl<'a, E: traits::Event> EventSlot<'a, E> {
    pub fn new<F: Fn(E::Payload) + 'a>(f: F) -> Self {
        Self(Box::new(f))
    }

    pub fn received(&self, payload: E::Payload) {
        self.0(payload)
    }
}

pub mod traits {
    pub trait Event {
        type Payload;
    }
    
    pub trait EventSignal<'a, E: Event> {
        fn connect(&mut self, slot: super::EventSlot<'a, E>);
        fn emit(&self, payload: E::Payload);
    }
}
