use futures::{AsyncWriteExt, AsyncWrite, future::LocalBoxFuture};
use super::{attr::AttributeValue, VNode, traits::RenderToStream, Scope, mode::Mode, node_key::VNodeKey};

pub mod event;

/// Virtual HTML element
pub struct ElementNode<M> where M: Mode {
    pub(super) scope:      Scope,
    pub(super) tag:        String,
    pub(super) attributes: ElementAttributes,
    pub(super) children:   Vec<VNode<M>>
}

impl<M> Default for ElementNode<M> where M: Mode {
    fn default() -> Self {
        Self { scope: Default::default(), tag: Default::default(), attributes: Default::default(), children: Default::default() }
    }
}

impl<M> Into<VNode<M>> for ElementNode<M> where M: Mode {
    fn into(self) -> VNode<M> {
        VNode::Element(self)
    }
}

impl<M> Into<Result<VNode<M>, crate::error::Error>> for ElementNode<M>  where M: Mode{
    fn into(self) -> Result<VNode<M>, crate::error::Error> {
        Ok(self.into())
    }
}

impl<M> ElementNode<M> where M: Mode {
    pub fn new<'a, Str: Into<String>>(parent: &Scope, tag: Str) -> Self 
    {
        Self {
            scope: parent.new_child_scope(),
            tag: tag.into(),
            attributes: ElementAttributes::default(),
            children: Default::default()
        }
    }

    pub fn id(&self) -> &VNodeKey {
        &self.scope.id
    }

    pub fn set_attribute<IntoStr: Into<String>, IntoVal: Into<AttributeValue>>(&mut self, name: IntoStr, value: IntoVal) -> &mut Self {
        self.attributes.set(name, value);
        self
    }

    pub fn extend_child<Iter: IntoIterator<Item=VNode<M>>>(&mut self, children: Iter) -> &mut Self {
        self.children.extend(children);
        self
    }

    pub fn append_child<IntoVNode: Into<VNode<M>>>(&mut self, child: IntoVNode) -> &mut Self {
        self.children.push(child.into());
        self
    }

    pub fn iter_children(&self) -> impl Iterator<Item = &VNode<M>> {
        self.children.iter()
    }

    /// Consume the mutable reference, replace its content with default value, and returns the value
    pub fn consume(&mut self) -> Self {
        std::mem::replace(self, Self::default())
    }   
}

impl<'a, M> RenderToStream<'a> for &'a ElementNode<M> where M: Mode {
    fn render_to_stream<'stream, 'fut, W: AsyncWrite + AsyncWriteExt + Unpin>(self, stream: &'stream mut W) -> LocalBoxFuture<'fut, Result<(), std::io::Error>>
    where 'a: 'fut, 'stream: 'fut {
        Box::pin(async {
            stream.write_all("<".as_bytes()).await?;
            stream.write_all(self.tag.as_bytes()).await?;
            self.attributes.render_to_stream(stream).await?;
            stream.write_all(">".as_bytes()).await?;

            for child in self.children.iter() {
                child.render_to_stream(stream).await?;
            }

            stream.write_all("</".as_bytes()).await?;
            stream.write_all(self.tag.as_bytes()).await?;
            stream.write_all(">".as_bytes()).await?;

            Ok(())
        })
    }
}

#[derive(Default, Clone)]
pub struct ElementAttributes(std::collections::HashMap<String, AttributeValue>);

impl ElementAttributes {
    pub fn set<IntoStr: Into<String>, IntoValue: Into<AttributeValue>>(&mut self, name: IntoStr, value: IntoValue) {
        self.0.insert(name.into(), value.into());
    }
}

impl<'a> FromIterator<(&'a str, AttributeValue)> for ElementAttributes {
    fn from_iter<T: IntoIterator<Item = (&'a str, AttributeValue)>>(iter: T) -> Self {
        Self(iter.into_iter().map(|(name, val)| (name.to_owned(), val)).collect())
    }
}

impl FromIterator<(String, AttributeValue)> for ElementAttributes {
    fn from_iter<T: IntoIterator<Item = (String, AttributeValue)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> RenderToStream<'a> for &'a ElementAttributes {
    fn render_to_stream<'stream, 'fut, W: AsyncWrite + AsyncWriteExt + Unpin>(self, stream: &'stream mut W) -> LocalBoxFuture<'fut, Result<(), std::io::Error>>
    where 'a: 'fut, 'stream: 'fut 
    {
        Box::pin(async {
            
            for (attr_name, attr_value) in self.0.iter().filter(|(_, attr_value)| attr_value.is_literal()) {
                stream.write(" ".as_bytes()).await?;
                stream.write_all(attr_name.as_bytes()).await?;
                stream.write_all("=".as_bytes()).await?;
                attr_value.render_to_stream(stream).await?;
            }

            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::vdom::{traits::RenderToStream, VNode, Scope, mode::DebugMode};

    #[tokio::test]
    async fn test_render_to_stream() {
        let root_scope = Scope::new_root();
        let el: VNode<_> = VNode::<DebugMode>::element(&root_scope, "div")
        .set_attribute("class", "px-2")
        .append_child(VNode::element(&root_scope, "p"))
        .consume()
        .into();

        let str = el.render_to_string().await.unwrap();
        
        assert_eq!(str, "<div class=\"px-2\"><p></p></div>");
    }
}