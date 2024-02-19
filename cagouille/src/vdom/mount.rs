use super::{VNode, VNodeData};
use crate::dom::Node;

impl VNode {
    pub fn mount(&mut self, parent: &Node) {
        let node = self.data.mount(parent);
        self.scope.node = Some(node);
    }
}

impl VNodeData {
    pub fn mount(&mut self, parent: &Node) -> Node {
        match self {
            Self::Component(comp) => comp.mount(parent),
            Self::Element(el) => comp.mount(parent),
            Self::Text(text) => {
                let node = web_sys::Text::new(text).unwrap();
                parent.add_child()
            }
        }
    }
}
