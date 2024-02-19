use super::{comp::ComponentNode, el::ElementNode, VNode, VNodeData};

impl PartialEq for VNode {
    fn eq(&self, other: &Self) -> bool {
        self.same(other)
    }
}

impl VNodeData {
    pub fn same(&self, other: &Self) -> bool {
        match (self, other) {
            (VNodeData::Component(comp), VNodeData::Component(other)) => comp.same(other),
            (VNodeData::Element(el), VNodeData::Element(other)) => el.same(other),
            (VNodeData::Text(string), VNodeData::Text(other)) => string == other,
            (VNodeData::Empty, VNodeData::Empty) => true,
            _ => false,
        }
    }
}

impl VNode {
    /// Check if both vnodes are the same.
    pub fn same(&self, other: &Self) -> bool {
        self.data.same(&other.data)
    }

    /// Patch the virtual node.
    pub fn patch(&mut self, other: Self) {}
}

impl ComponentNode {
    pub fn same(&self, other: &Self) -> bool {
        self.type_id() == other.type_id()
    }
}

impl ElementNode {
    pub fn same(&self, other: &Self) -> bool {
        return self.tag != other.tag;
    }
}

#[cfg(test)]
mod tests {
    use crate::vdom::{mode::DebugMode, VNode};

    #[test]
    pub fn vnode_text_same_true() {
        let n1 = VNode::text("texte 1");
        let n2 = VNode::text("texte 1");

        assert_eq!(n1, n2);
    }

    #[test]
    pub fn vnode_text_same_false() {
        let n1 = VNode::text("texte 1");
        let n2 = VNode::text("texte 2");

        assert_ne!(n1, n2);
    }
}
