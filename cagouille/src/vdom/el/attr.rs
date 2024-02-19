use futures::{future::LocalBoxFuture, AsyncWrite, AsyncWriteExt};

use crate::vdom::{attr::AttributeValue, traits::RenderToStream};

pub type ElementAttribute = (String, AttributeValue);

#[derive(Default, Clone)]
pub struct ElementAttributes(std::collections::HashMap<String, AttributeValue>);

impl ElementAttributes {
    pub fn set<IntoStr: Into<String>, IntoValue: Into<AttributeValue>>(
        &mut self,
        name: IntoStr,
        value: IntoValue,
    ) {
        self.0.insert(name.into(), value.into());
    }
}

impl<'a> FromIterator<(&'a str, AttributeValue)> for ElementAttributes {
    fn from_iter<T: IntoIterator<Item = (&'a str, AttributeValue)>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|(name, val)| (name.to_owned(), val))
                .collect(),
        )
    }
}

impl FromIterator<(String, AttributeValue)> for ElementAttributes {
    fn from_iter<T: IntoIterator<Item = (String, AttributeValue)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> RenderToStream<'a> for &'a ElementAttributes {
    fn render_to_stream<'stream, 'fut, W: AsyncWrite + AsyncWriteExt + Unpin>(
        self,
        stream: &'stream mut W,
    ) -> LocalBoxFuture<'fut, Result<(), std::io::Error>>
    where
        'a: 'fut,
        'stream: 'fut,
    {
        Box::pin(async {
            for (attr_name, attr_value) in self
                .0
                .iter()
                .filter(|(_, attr_value)| attr_value.is_literal())
            {
                stream.write(" ".as_bytes()).await?;
                stream.write_all(attr_name.as_bytes()).await?;
                stream.write_all("=".as_bytes()).await?;
                attr_value.render_to_stream(stream).await?;
            }

            Ok(())
        })
    }
}
