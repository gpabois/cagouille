use crate::{component::traits::Component, dom::Node, vdom::traits::Mountable};

use super::{ComponentNode, ConcreteComponentNode};

impl ComponentNode {
    pub fn mount(&mut self, parent: &Node) -> Node {
        self.driver.mount(parent)
    }
}

impl<Comp> Mountable for ConcreteComponentNode<Comp>
where
    Comp: Component,
{
    type Node = Node;

    fn mount(&mut self, parent: &Self::Node) -> Self::Node {
        todo!()
    }
}
