use futures::{future::LocalBoxFuture, AsyncWrite, AsyncWriteExt};
use crate::vdom::traits::RenderToStream;
use super::{Value, Attributes};

impl<'a> RenderToStream<'a> for &'a Attributes {
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

impl<'a> RenderToStream<'a> for &'a Value {
    fn render_to_stream<'stream, 'fut, W: AsyncWrite + AsyncWriteExt + Unpin + ?Sized>(
        self,
        stream: &'stream mut W,
    ) -> LocalBoxFuture<'fut, Result<(), std::io::Error>>
    where
        'a: 'fut,
        'stream: 'fut,
    {
        Box::pin(async move {
            match self {
                Value::Boolean(b) => {
                    stream.write_all(format!("\"{b}\"").as_bytes()).await?;
                }
                Value::String(ref_str) => {
                    let str = ref_str.replace("\"", "\\\"");
                    stream.write_all(format!("\"{}\"", str).as_bytes()).await?;
                }
            }

            Ok(())
        })
    }
}

