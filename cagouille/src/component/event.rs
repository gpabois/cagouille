
use crate::event::{EventSlot, traits::Event};


/// Component's state has been updated.
pub struct ComponentUpdated;
impl Event for ComponentUpdated {
    type Payload = ();
}

pub struct CommonComponentEvents<'events> {
    updated: Option<EventSlot<'events, ComponentUpdated>>
}