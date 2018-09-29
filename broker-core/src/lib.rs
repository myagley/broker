extern crate bytes;

use bytes::{BufMut, Bytes};

mod util;

use self::util::{VarintPut, VarintSize};

const NULL_VARINT_SIZE: usize = 1;
const NULL_LENGTH: i32 = -1;

#[derive(Default)]
pub struct Headers;

impl Headers {
    pub fn size(&self) -> usize {
        0
    }
}

#[derive(Default)]
pub struct Record {
    length: i32,
    attributes: u8,
    offset: i64,
    timestamp: i64,
    sequence: i32,
    key: Option<Bytes>,
    value: Option<Bytes>,
    headers: Headers,
}

impl Record {
    pub fn new() -> Record {
        Default::default()
    }

    pub fn length(&self) -> i32 {
        self.length
    }

    pub fn attributes(&self) -> u8 {
        self.attributes
    }

    pub fn offset(&self) -> i64 {
        self.offset
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn sequence(&self) -> i32 {
        self.sequence
    }

    pub fn key(&self) -> Option<&Bytes> {
        self.key.as_ref()
    }

    pub fn value(&self) -> Option<&Bytes> {
        self.value.as_ref()
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn with_attributes(mut self, attributes: u8) -> Record {
        self.attributes = attributes;
        self
    }

    pub fn with_offset(mut self, offset: i64) -> Record {
        self.offset = offset;
        self
    }

    pub fn with_timestamp(mut self, timestamp: i64) -> Record {
        self.timestamp = timestamp;
        self
    }

    pub fn with_sequence(mut self, sequence: i32) -> Record {
        self.sequence = sequence;
        self
    }

    pub fn with_key(mut self, key: Bytes) -> Record {
        self.key = Some(key);
        self
    }

    pub fn with_value(mut self, value: Bytes) -> Record {
        self.value = Some(value);
        self
    }

    pub fn with_headers(mut self, headers: Headers) -> Record {
        self.headers = headers;
        self
    }

    pub fn put<B>(self, buf: &mut B)
        where B: BufMut
    {
        let length = self.body_length();
        buf.put_usize_varint(length);
        buf.put_u8(self.attributes);
        buf.put_i64_varint(self.timestamp);
        buf.put_i64_varint(self.offset);
        if let Some(key) = self.key {
            buf.put_usize_varint(key.len());
            buf.put(key);
        } else {
            buf.put_i32_varint(NULL_LENGTH);
        }
        if let Some(value) = self.value {
            buf.put_usize_varint(value.len());
            buf.put(value);
        } else {
            buf.put_i32_varint(NULL_LENGTH);
        }
        buf.put_usize_varint(self.headers.size());
    }

    fn body_length(&self) -> usize {
        let mut size = 1; // always one byte for attributes
        size += self.offset.varint_size();
        size += self.timestamp.varint_size();
        size += self.key
            .as_ref()
            .map(|k| k.len() + k.len().varint_size())
            .unwrap_or_else(|| NULL_VARINT_SIZE);
        size += self.value
            .as_ref()
            .map(|v| v.len() + v.len().varint_size())
            .unwrap_or_else(|| NULL_VARINT_SIZE);
        size += self.headers.size().varint_size();
        size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use bytes::{Bytes, BytesMut};

    #[test]
    pub fn body_length_min() {
        let size = Record::new().body_length();
        assert_eq!(6, size);
    }

    #[test]
    pub fn body_length_value() {
        let size = Record::new()
            .with_timestamp(0)
            .with_offset(0)
            .with_value(Bytes::from_static(b"message1"))
            .body_length();
        assert_eq!(14, size);
    }

    #[test]
    pub fn test_put() {
        let mut bytes = BytesMut::new();
        Record::new()
            .with_timestamp(0)
            .with_offset(0)
            .with_value(Bytes::from_static(b"message1"))
            .put(&mut bytes);

        let expected = [0x1C, 0x00, 0x00, 0x00, 0x01, 0x10, 0x6D, 0x65, 0x73, 0x73, 0x61, 0x67,
                        0x65, 0x31, 0x00];
        assert_eq!(&expected, bytes.as_ref());
    }
}
