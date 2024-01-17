use std::sync::{Arc, RwLock};

#[derive(Default, Clone)]
/// A reference to a dom node
pub struct DomNodeRef(Arc<RwLock<Inner>>);

impl DomNodeRef {
    /// Returns true, if the ref points to an existing html node.
    pub fn is_bound(&self) -> bool {
        self.0
        .read()
        .unwrap()
        .0
        .is_some()
    }

    /// Bound a dom node
    pub fn bound(&mut self, node: web_sys::Node) {
        self.0.write().unwrap().0 = Some(node)
    }

}

#[derive(Default)]
struct Inner(Option<web_sys::Node>);