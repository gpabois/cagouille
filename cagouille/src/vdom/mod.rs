
use crate::error::Error;

use self::traits::RenderToStream;
use futures::{io::{AsyncWriteExt, BufWriter}, future::LocalBoxFuture, AsyncWrite};

mod attr;
mod element;

pub type VDomResult = Result<VNode, Error>;

pub mod traits {

    use futures::{AsyncWrite, AsyncWriteExt};
    use futures::io::{BufWriter, Error};
    use futures::future::LocalBoxFuture;

    /// Render object in the stream
    pub trait RenderToStream<'a> {
        fn render_to_stream<'stream, 'fut, W: AsyncWrite + AsyncWriteExt + Unpin>(self, stream: &'stream mut W) 
        -> LocalBoxFuture<'fut, Result<(), Error>>
        where 'a: 'fut, 'stream: 'fut;
    }
}

/// Virtual DOM node
pub enum VNode {
    Element(element::ElementNode),
    Text(String)
}

impl<'a> RenderToStream<'a> for &'a VNode {
    fn render_to_stream<'stream, 'fut, W: AsyncWrite + AsyncWriteExt + Unpin>(self, stream: &'stream mut W) 
    -> LocalBoxFuture<'fut, Result<(), std::io::Error>>
    where 'a: 'fut, 'stream: 'fut {
        Box::pin(async move {
            match self {
                VNode::Element(el) => el.render_to_stream(stream).await?,
                VNode::Text(text) =>  stream.write_all(text.as_bytes()).await?
            }

            Ok(())
        })
    }
}