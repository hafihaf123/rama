pub use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

pub mod layer;
pub mod service;

pub trait Stream: AsyncRead + AsyncWrite {}

impl<T> Stream for T where T: AsyncRead + AsyncWrite {}

pub use tokio::io::{AsyncReadExt, AsyncWriteExt};