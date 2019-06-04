use std::vec;

use atoi::atoi;
use bytes::{BufMut, BytesMut};
use tokio::codec::{Decoder, Encoder};
use tokio::prelude::*;

use super::Bytes;

struct RespError {
    reason: std::io::Error,
    text: String,
}

#[derive(Debug)]
pub struct Array {
    pub size: usize,
    pub elems: vec::Vec<Kind>,
}

#[derive(Debug)]
pub enum Kind {
    Integer(i64),
    Error(Bytes),
    SimpleString(Bytes),
    BulkString(Option<Bytes>),
    Array(Array),
    Inline(Bytes),
}

impl Kind {
    pub fn marshal(&self, buf: &mut BytesMut) {
        match self {
            Kind::Integer(v) => {
                buf.reserve(":\r\n".len() + v.to_string().len());
                buf.put(b':');
                buf.put(v.to_string());
                buf.put("\r\n");
            }
            Kind::SimpleString(s) => {
                buf.reserve("+\r\n".len() + s.len());
                buf.put(b'+');
                buf.put(s);
                buf.put("\r\n");
            }
            Kind::BulkString(s) => {
                buf.reserve(1);
                buf.put(b'$');
                if let Some(s) = s {
                    buf.reserve("\r\n".len() + s.len() + s.len().to_string().len());
                    buf.put(s.len().to_string());
                    buf.put("\r\n");
                    buf.put(s);
                } else {
                    buf.reserve(2);
                    buf.put((-1).to_string());
                }
                buf.reserve(2);
                buf.put("\r\n");
            }
            Kind::Error(e) => {
                buf.reserve("-\r\n".len() + e.len());
                buf.put(b'-');
                buf.put(e);
                buf.put("\r\n");
            }
            Kind::Array(a) => {
                // marshal the array header and return, the caller
                // should marshal its elements later
                buf.reserve("*\r\n".len() + a.size.to_string().len());
                buf.put(b'*');
                buf.put(a.size.to_string());
                buf.put("\r\n");
                // TODO marshal elements here
            }
            Kind::Inline(s) => {
                buf.reserve("\r\n".len() + s.len());
                buf.put(s);
                buf.put("\r\n");
            }
        }
    }
    pub fn unmarshal(buf: &mut BytesMut) -> Result<Option<Kind>, std::io::Error> {
        if buf.len() == 0 {
            return Ok(None);
        }

        let offset = buf.iter().position(|b| *b == b'\n');
        if offset == None {
            return Ok(None);
        }
        let offset = offset.unwrap();
        if offset < "0\r".len() {
            return Ok(None);
        }

        let kind = buf[0];
        match kind {
            b'+' => {
                let line = buf.split_to(offset + 1);
                Ok(Some(Kind::SimpleString(Bytes::from(
                    &line[1..line.len() - 2],
                ))))
            }
            b'-' => {
                let line = buf.split_to(offset + 1);
                Ok(Some(Kind::Error(Bytes::from(&line[1..line.len() - 2]))))
            }
            b':' => {
                let line = buf.split_to(offset + 1);
                let val: i64 = atoi(&line[1..line.len() - 2]).unwrap();
                Ok(Some(Kind::Integer(val)))
            }
            b'$' => {
                let line = buf.split_to(offset + 1);
                let len: i64 = atoi(&line[1..line.len() - 2]).unwrap();
                let next_offset = buf.iter().position(|b| *b == b'\n');
                match next_offset {
                    Some(end) => {
                        if end - 1 != len as usize {
                            // handle error
                        }
                        let line = buf.split_to(end + 1);
                        Ok(Some(Kind::BulkString(Some(Bytes::from(
                            &line[..line.len() - 2],
                        )))))
                    }
                    None => Ok(None),
                }
            }
            b'*' => {
                let line = buf.split_to(offset + 1);
                let len: i64 = atoi(&line[1..line.len() - 2]).unwrap();
                Ok(Some(Kind::Array(Array {
                    size: len as usize,
                    elems: vec::Vec::new(),
                })))
            }
            _ => {
                // inline command which is a plain text like: foo\r\n
                let line = buf.split_to(offset + 1);
                let last = line.len() - 1;
                if line[last - 1] == b'\r' {
                    Ok(Some(Kind::Inline(Bytes::from(&line[0..line.len() - 2]))))
                } else {
                    Ok(Some(Kind::Inline(Bytes::from(&line[0..line.len() - 1]))))
                }
            }
        }
    }
}

pub struct Codec {
    arrays: vec::Vec<Array>,
}

impl Codec {
    pub fn new() -> Codec {
        Codec {
            arrays: vec::Vec::new(),
        }
    }
}

impl Encoder for Codec {
    type Item = Kind;
    type Error = std::io::Error;

    fn encode(&mut self, v: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        v.marshal(buf);
        Ok(())
    }
}

impl Decoder for Codec {
    type Item = Kind;
    type Error = std::io::Error;
    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        loop {
            match Kind::unmarshal(buf)? {
                Some(Kind::Array(a)) => {
                    self.arrays.push(a);
                }
                Some(x) => match self.arrays.pop() {
                    Some(mut a) => {
                        if a.size > 0 {
                            a.size -= 1;
                            a.elems.push(x);
                        }
                        if a.size == 0 {
                            a.size = a.elems.len();
                            break Ok(Some(Kind::Array(a)));
                        }
                        self.arrays.push(a);
                    }
                    None => break Ok(Some(x)),
                },
                None => break Ok(None),
            }
        }
    }
}
