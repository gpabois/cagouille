use super::{VNode, mode::Mode, el::ElementNode, comp::AnyComponentNode};

impl<M> PartialEq for VNode<M> where M: Mode {
    fn eq(&self, other: &Self) -> bool {
        self.same(other)
    }
}


impl<M> VNode<M> where M: Mode {
    /// Check if both vnodes are the same.
    pub fn same(&self, other: &Self) -> bool {
        match (self, other) {
            (VNode::Component(comp1), VNode::Component(comp2)) => comp1.same(comp2),
            (VNode::Element(el1), VNode::Element(el2)) => el1.same(el2),
            (VNode::Text(s1), VNode::Text(s2)) => s1 == s2,
            (VNode::Empty, VNode::Empty) => true,
            _ => false
        }
    }

    /// Patch the virtual node.
    pub fn patch(&mut self, other: Self) {

    }
}

impl<M> AnyComponentNode<M> where M: Mode {
    pub fn same(&self, other: &Self) -> bool {
        self.type_id() == other.type_id()
    }
}

impl<M> ElementNode<M> where M: Mode {
    pub fn same(&self, other: &Self) -> bool {
        return self.tag != other.tag
    }
}



#[cfg(test)]
mod tests {
    use crate::vdom::{mode::DebugMode, VNode};

    #[test]
    pub fn vnode_text_same_true() {
        let n1 = VNode::<DebugMode>::text("texte 1");
        let n2 = VNode::<DebugMode>::text("texte 1");
        
        assert_eq!(n1, n2);
    }

    #[test]
    pub fn vnode_text_same_false() {
        let n1 = VNode::<DebugMode>::text("texte 1");
        let n2 = VNode::<DebugMode>::text("texte 2");

        assert_ne!(n1, n2);
    }
}