use futures::{future::LocalBoxFuture, AsyncWrite, AsyncWriteExt};

use super::traits::RenderToStream;

#[derive(Clone)]
pub enum AttributeValue {
    String(String),
    Boolean(bool),
}

impl From<bool> for AttributeValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<String> for AttributeValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for AttributeValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl AttributeValue {
    pub fn is_literal(&self) -> bool {
        match self {
            Self::String(_) => true,
            Self::Boolean(_) => true,
        }
    }
}

impl<'a> RenderToStream<'a> for &'a AttributeValue {
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
                AttributeValue::Boolean(b) => {
                    stream.write_all(format!("\"{b}\"").as_bytes()).await?;
                }
                AttributeValue::String(ref_str) => {
                    let str = ref_str.replace("\"", "\\\"");
                    stream.write_all(format!("\"{}\"", str).as_bytes()).await?;
                }
            }

            Ok(())
        })
    }
}
