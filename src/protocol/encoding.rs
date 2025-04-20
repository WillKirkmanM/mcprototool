use crate::protocol::types::VarInt;
use bytes::{Buf, BufMut, BytesMut};
use std::io::{Error as IoError, ErrorKind, Result as IoResult};
use std::string::FromUtf8Error;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use uuid::Uuid;

const MAX_VARINT_SIZE: usize = 5;
const MAX_STRING_LENGTH: usize = 32767;

#[derive(Debug)]
pub enum DecodeError {
    Io(IoError),
    VarIntTooLong,
    InvalidVarInt,
    StringTooLong(usize),
    InvalidUtf8(FromUtf8Error),
    NotEnoughBytes,
}

impl From<IoError> for DecodeError {
    fn from(e: IoError) -> Self {
        DecodeError::Io(e)
    }
}

impl From<FromUtf8Error> for DecodeError {
    fn from(e: FromUtf8Error) -> Self {
        DecodeError::InvalidUtf8(e)
    }
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeError::Io(e) => write!(f, "IO error: {}", e),
            DecodeError::VarIntTooLong => write!(f, "VarInt is too long"),
            DecodeError::InvalidVarInt => write!(f, "Invalid VarInt encoding"),
            DecodeError::StringTooLong(len) => write!(f, "String length {} exceeds maximum", len),
            DecodeError::InvalidUtf8(e) => write!(f, "Invalid UTF-8 sequence: {}", e),
            DecodeError::NotEnoughBytes => write!(f, "Not enough bytes in buffer"),
        }
    }
}

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DecodeError::Io(e) => Some(e),
            DecodeError::InvalidUtf8(e) => Some(e),
            _ => None,
        }
    }
}

pub type EncodeResult<T> = Result<T, IoError>;
pub type DecodeResult<T> = Result<T, DecodeError>;

pub async fn write_varint<W: AsyncWrite + Unpin>(
    writer: &mut W,
    value: VarInt,
) -> EncodeResult<()> {
    let mut val = value.0 as u32;
    loop {
        let mut temp = (val & 0b0111_1111) as u8;
        val >>= 7;
        if val != 0 {
            temp |= 0b1000_0000;
        }
        writer.write_u8(temp).await?;
        if val == 0 {
            break;
        }
    }
    Ok(())
}

pub async fn write_string<W: AsyncWrite + Unpin>(writer: &mut W, value: &str) -> EncodeResult<()> {
    let bytes = value.as_bytes();
    write_varint(writer, VarInt(bytes.len() as i32)).await?;
    writer.write_all(bytes).await?;
    Ok(())
}

pub async fn write_u16<W: AsyncWrite + Unpin>(writer: &mut W, value: u16) -> EncodeResult<()> {
    writer.write_u16(value).await?;
    Ok(())
}

pub async fn write_i64<W: AsyncWrite + Unpin>(writer: &mut W, value: i64) -> EncodeResult<()> {
    writer.write_i64(value).await?;
    Ok(())
}

pub async fn write_bytes_prefixed<W: AsyncWriteExt + Unpin>(
    writer: &mut W,
    data: &[u8],
) -> EncodeResult<()> {
    write_varint(writer, VarInt(data.len() as i32)).await?;
    writer.write_all(data).await?;
    Ok(())
}

pub async fn read_varint<R: AsyncRead + Unpin>(reader: &mut R) -> DecodeResult<VarInt> {
    let mut num_read = 0;
    let mut result = 0i32;
    let mut shift = 0;
    loop {
        let read = reader.read_u8().await?;
        num_read += 1;

        let value = (read & 0b0111_1111) as i32;
        result |= value << shift;

        shift += 7;

        if (read & 0b1000_0000) == 0 {
            break;
        }

        if num_read > MAX_VARINT_SIZE {
            return Err(DecodeError::VarIntTooLong);
        }
    }
    Ok(VarInt(result))
}

pub async fn read_string<R: AsyncRead + Unpin>(reader: &mut R) -> DecodeResult<String> {
    let len_varint = read_varint(reader).await?;
    let len = len_varint.0 as usize;

    if len > MAX_STRING_LENGTH * 4 {
        return Err(DecodeError::StringTooLong(len));
    }

    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf).await?;

    String::from_utf8(buf).map_err(DecodeError::from)
}

pub async fn read_i64<R: AsyncRead + Unpin>(reader: &mut R) -> DecodeResult<i64> {
    Ok(reader.read_i64().await?)
}

pub async fn read_bool<R: AsyncRead + Unpin>(reader: &mut R) -> DecodeResult<bool> {
    Ok(reader.read_u8().await? != 0)
}

pub async fn read_uuid<R: AsyncRead + Unpin>(reader: &mut R) -> DecodeResult<Uuid> {
    let mut bytes = [0u8; 16];
    reader.read_exact(&mut bytes).await?;
    Ok(Uuid::from_bytes(bytes))
}

pub async fn read_bytes_prefixed<R: AsyncRead + Unpin>(reader: &mut R) -> DecodeResult<Vec<u8>> {
    let len_varint = read_varint(reader).await?;
    let len = len_varint.0 as usize;
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf).await?;
    Ok(buf)
}

pub async fn write_packet_frame<W: AsyncWrite + Unpin>(
    writer: &mut W,
    packet_id: VarInt,
    data: &[u8],
) -> EncodeResult<()> {
    let mut id_buf = BytesMut::new();
    write_varint_sync(&mut id_buf, packet_id)?;

    let packet_len = id_buf.len() + data.len();

    write_varint(writer, VarInt(packet_len as i32)).await?;

    writer.write_all(&id_buf).await?;

    writer.write_all(data).await?;

    Ok(())
}

pub async fn read_packet_frame<R: AsyncRead + Unpin>(
    reader: &mut R,
) -> DecodeResult<(VarInt, BytesMut)> {
    let packet_len_varint = read_varint(reader).await?;
    let packet_len = packet_len_varint.0 as usize;

    let mut packet_buf = BytesMut::with_capacity(packet_len);
    let mut temp_vec = vec![0u8; packet_len];
    reader.read_exact(&mut temp_vec).await?;
    packet_buf.put_slice(&temp_vec);

    let packet_id = read_varint_sync(&mut packet_buf)?;

    Ok((packet_id, packet_buf))
}

pub fn write_varint_sync<B: BufMut>(buf: &mut B, value: VarInt) -> EncodeResult<()> {
    let mut val = value.0 as u32;
    loop {
        let mut temp = (val & 0b0111_1111) as u8;
        val >>= 7;
        if val != 0 {
            temp |= 0b1000_0000;
        }
        buf.put_u8(temp);
        if val == 0 {
            break;
        }
    }
    Ok(())
}

pub fn read_varint_sync<B: Buf>(buf: &mut B) -> DecodeResult<VarInt> {
    let mut num_read = 0;
    let mut result = 0i32;
    let mut shift = 0;
    let initial_len = buf.remaining();
    while buf.has_remaining() {
        let read = buf.get_u8();
        num_read += 1;
        let value = (read & 0b0111_1111) as i32;
        result |= value << shift;
        shift += 7;
        if (read & 0b1000_0000) == 0 {
            return Ok(VarInt(result));
        }
        if num_read > MAX_VARINT_SIZE {
            return Err(DecodeError::VarIntTooLong);
        }
    }

    if num_read > 0 && shift <= 35 {
        return Err(DecodeError::InvalidVarInt);
    }
    Err(DecodeError::NotEnoughBytes)
}

pub fn read_string_sync(buf: &mut BytesMut) -> Result<String, DecodeError> {
    let len_varint = read_varint_sync(buf)?;
    let len = len_varint.0 as usize;
    if len > MAX_STRING_LENGTH * 4 {
        return Err(DecodeError::StringTooLong(len));
    }
    if buf.remaining() < len {
        return Err(DecodeError::NotEnoughBytes);
    }
    let str_bytes = buf.split_to(len);
    String::from_utf8(str_bytes.to_vec()).map_err(DecodeError::from)
}

pub fn read_bool_sync(buf: &mut BytesMut) -> Result<bool, DecodeError> {
    if !buf.has_remaining() {
        return Err(DecodeError::NotEnoughBytes);
    }
    Ok(buf.get_u8() != 0)
}

pub fn read_uuid_sync(buf: &mut BytesMut) -> Result<Uuid, DecodeError> {
    if buf.remaining() < 16 {
        return Err(DecodeError::NotEnoughBytes);
    }
    let bytes_slice = buf.copy_to_bytes(16);
    Ok(Uuid::from_bytes(bytes_slice.as_ref().try_into().unwrap()))
}

pub fn read_bytes_prefixed_sync(buf: &mut BytesMut) -> Result<Vec<u8>, DecodeError> {
    let len_varint = read_varint_sync(buf)?;
    let len = len_varint.0 as usize;
    if buf.remaining() < len {
        return Err(DecodeError::NotEnoughBytes);
    }
    Ok(buf.split_to(len).to_vec())
}
