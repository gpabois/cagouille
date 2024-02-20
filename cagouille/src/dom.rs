pub enum Node {
    Text(web_sys::Text),
    Element(web_sys::HtmlElement),
}

impl Node {
    pub fn add_child<IntoNode: Into<Node>>(&self, child: IntoNode) {
        match self {
            Node::Text(node) => {}
            Node::Element(node) => node.add_child(child.into()),
        }
    }
}

impl From<web_sys::Text> for Node {
    fn from(value: web_sys::Text) -> Self {
        Self::Text(value)
    }
}

impl From<web_sys::HtmlElement> for Node {
    fn from(value: web_sys::HtmlElement) -> Self {
        Self::Element(value)
    }
}
