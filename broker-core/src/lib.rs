extern crate bytes;

use bytes::Bytes;

mod util;

#[derive(Default)]
pub struct Headers;

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

    pub fn with_attributes<'a>(&'a mut self, attributes: u8) -> &'a mut Record {
        self.attributes = attributes;
        self
    }

    pub fn with_offset<'a>(&'a mut self, offset: i64) -> &'a mut Record {
        self.offset = offset;
        self
    }

    pub fn with_timestamp<'a>(&'a mut self, timestamp: i64) -> &'a mut Record {
        self.timestamp = timestamp;
        self
    }

    pub fn with_sequence<'a>(&'a mut self, sequence: i32) -> &'a mut Record {
        self.sequence = sequence;
        self
    }

    pub fn with_key<'a>(&'a mut self, key: Bytes) -> &'a mut Record {
        self.key = Some(key);
        self
    }

    pub fn with_value<'a>(&'a mut self, value: Bytes) -> &'a mut Record {
        self.value = Some(value);
        self
    }

    pub fn with_headers<'a>(&'a mut self, headers: Headers) -> &'a mut Record {
        self.headers = headers;
        self
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
