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
            Self::Element(el) => el.mount(parent),
            Self::Text(text) => {
                let mut node = web_sys::Text::new().unwrap();
                node.set_data(text.as_str());
                parent.add_child(node.clone());
                node
            },
            Self::Empty => {
                let node = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("div")
                .unwrap();

                parent.add_child(node.clone());

                node
            }
        }
    }
}
