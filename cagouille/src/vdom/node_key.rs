use std::sync::{Arc, RwLock};


#[derive(Default, Clone, PartialEq)]
/// A reference to a dom node
pub struct VNodeKey(pub(super) u32);

impl From<u32> for VNodeKey {
    fn from(value: u32) -> Self {
        VNodeKey(value.into())
    }
}