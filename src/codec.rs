//! Encoding/decoding mechanisms for ABCI requests and responses.
//!
//! Implements the [Tendermint Socket Protocol][tsp].
//!
//! [tsp]: https://github.com/tendermint/tendermint/blob/v0.34.x/spec/abci/client-server.md#tsp

use async_iterator::Iterator;
use bytes::{Buf, BufMut, BytesMut};
use prost::Message;
use std::marker::{PhantomData, Unpin};
use tendermint_proto::v0_37::abci::{Request, Response};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use tendermint_abci::Error;

/// The maximum number of bytes we expect in a varint. We use this to check if
/// we're encountering a decoding error for a varint.
pub const MAX_VARINT_LENGTH: usize = 16;

/// The server receives incoming requests, and sends outgoing responses.
pub type ServerCodec<S> = Codec<S, Request, Response>;

/// Allows for iteration over `S` to produce instances of `I`, as well as
/// sending instances of `O`.
pub struct Codec<S, I, O> {
    stream: S,
    // Long-running read buffer
    read_buf: BytesMut,
    // Fixed-length read window
    read_window: Vec<u8>,
    write_buf: BytesMut,
    _incoming: PhantomData<I>,
    _outgoing: PhantomData<O>,
}

impl<S, I, O> Codec<S, I, O>
where
    S: AsyncReadExt + AsyncWriteExt + Unpin,
    I: Message + Default,
    O: Message,
{
    /// Constructor.
    pub fn new(stream: S, read_buf_size: usize) -> Self {
        Self {
            stream,
            read_buf: BytesMut::new(),
            read_window: vec![0_u8; read_buf_size],
            write_buf: BytesMut::new(),
            _incoming: Default::default(),
            _outgoing: Default::default(),
        }
    }
}

// Iterating over a codec produces instances of `Result<I>`.
impl<S, I, O> Iterator for Codec<S, I, O>
where
    S: AsyncReadExt + Unpin,
    I: Message + Default,
{
    type Item = Result<I, Error>;

    async fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Try to decode an incoming message from our buffer first
            match decode_length_delimited::<I>(&mut self.read_buf) {
                Ok(Some(incoming)) => return Some(Ok(incoming)),
                Err(e) => return Some(Err(e)),
                _ => (), // not enough data to decode a message, let's continue.
            }

            // If we don't have enough data to decode a message, try to read
            // more
            let bytes_read = match self.stream.read(self.read_window.as_mut()).await {
                Ok(br) => br,
                Err(e) => return Some(Err(Error::io(e))),
            };
            if bytes_read == 0 {
                // The underlying stream terminated
                return None;
            }
            self.read_buf
                .extend_from_slice(&self.read_window[..bytes_read]);
        }
    }
}

impl<S, I, O> Codec<S, I, O>
where
    S: AsyncWriteExt + Unpin,
    O: Message,
{
    /// Send a message using this codec.
    pub async fn send(&mut self, message: O) -> Result<(), Error> {
        encode_length_delimited(message, &mut self.write_buf)?;
        while !self.write_buf.is_empty() {
            let bytes_written = self
                .stream
                .write(self.write_buf.as_ref())
                .await
                .map_err(Error::io)?;

            if bytes_written == 0 {
                return Err(Error::io(std::io::Error::new(
                    std::io::ErrorKind::WriteZero,
                    "failed to write to underlying stream",
                )));
            }
            self.write_buf.advance(bytes_written);
        }

        self.stream.flush().await.map_err(Error::io)?;

        Ok(())
    }
}

/// Encode the given message with a length prefix.
pub fn encode_length_delimited<M, B>(message: M, mut dst: &mut B) -> Result<(), Error>
where
    M: Message,
    B: BufMut,
{
    let mut buf = BytesMut::new();
    message.encode(&mut buf).map_err(Error::encode)?;

    let buf = buf.freeze();
    prost::encoding::encode_varint(buf.len() as u64, &mut dst);
    dst.put(buf);
    Ok(())
}

/// Attempt to decode a message of type `M` from the given source buffer.
pub fn decode_length_delimited<M>(src: &mut BytesMut) -> Result<Option<M>, Error>
where
    M: Message + Default,
{
    let src_len = src.len();
    let mut tmp = src.clone().freeze();
    let encoded_len = match prost::encoding::decode_varint(&mut tmp) {
        Ok(len) => len,
        // We've potentially only received a partial length delimiter
        Err(_) if src_len <= MAX_VARINT_LENGTH => return Ok(None),
        Err(e) => return Err(Error::decode(e)),
    };
    let remaining = tmp.remaining() as u64;
    if remaining < encoded_len {
        // We don't have enough data yet to decode the entire message
        Ok(None)
    } else {
        let delim_len = src_len - tmp.remaining();
        // We only advance the source buffer once we're sure we have enough
        // data to try to decode the result.
        src.advance(delim_len + (encoded_len as usize));

        let mut result_bytes = BytesMut::from(tmp.split_to(encoded_len as usize).as_ref());
        let res = M::decode(&mut result_bytes).map_err(Error::decode)?;

        Ok(Some(res))
    }
}
