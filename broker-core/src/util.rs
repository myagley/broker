use bytes::BufMut;


pub trait VarintPut {
    fn put_i32_varint(&mut self, n: i32);
    fn put_i64_varint(&mut self, n: i64);
    fn put_usize_varint(&mut self, n: usize);
}

impl<T> VarintPut for T
    where T: BufMut
{
    fn put_i32_varint(&mut self, n: i32) {
        let mut v: i32 = (n << 1) ^ (n >> 31);
        while (v & 0xffff80) != 0 {
            let b = ((v & 0x7f) | 0x80) as i8;
            self.put_i8(b);
            v >>= 7;
        }
        self.put_i8(v as i8);
    }

    fn put_i64_varint(&mut self, n: i64) {
        let mut v: i64 = (n << 1) ^ (n >> 63);
        while (v & 0xffffffffffff80) != 0 {
            let b: i8 = ((v & 0x7f) | 0x80) as i8;
            self.put_i8(b);
            v >>= 7;
        }
        self.put_i8(v as i8);
    }

    fn put_usize_varint(&mut self, n: usize) {
        self.put_i64_varint(n as i64)
    }
}

pub trait VarintSize {
    fn varint_size(&self) -> usize;
}

impl VarintSize for i32 {
    fn varint_size(&self) -> usize {
        let mut v = (*self << 1) ^ (*self >> 31);
        let mut bytes = 1;
        while (v & 0xffff80) != 0 {
            bytes += 1;
            v >>= 7;
        }
        bytes
    }
}

impl VarintSize for i64 {
    fn varint_size(&self) -> usize {
        let mut v = (*self << 1) ^ (*self >> 63);
        let mut bytes = 1;
        while (v & 0xffffffffffff80) != 0 {
            bytes += 1;
            v >>= 7;
        }
        bytes
    }
}

impl VarintSize for usize {
    fn varint_size(&self) -> usize {
        let mut v = (*self << 1) ^ (*self >> 63);
        let mut bytes = 1;
        while (v & 0xffffffffffff80) != 0 {
            bytes += 1;
            v >>= 7;
        }
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::{Buf, BytesMut, IntoBuf};

    #[test]
    fn small_i32() {
        let mut bytes = BytesMut::new();
        bytes.put_i32_varint(14);

        let val = bytes.into_buf().get_i8();
        assert_eq!(0x1c, val);
    }

    #[test]
    fn larger_i32() {
        let mut bytes = BytesMut::new();
        bytes.put_i32_varint(150);

        let val = bytes.into_buf().get_u16_be();
        assert_eq!(44034, val);
    }

    #[test]
    fn small_i64() {
        let mut bytes = BytesMut::new();
        bytes.put_i64_varint(14);

        let val = bytes.into_buf().get_i8();
        assert_eq!(0x1c, val);
    }

    #[test]
    fn larger_i64() {
        let mut bytes = BytesMut::new();
        bytes.put_i64_varint(150);

        let val = bytes.into_buf().get_u16_be();
        assert_eq!(44034, val);
    }

    #[test]
    fn sizeof_i32_null() {
        let val: i32 = -1;
        assert_eq!(1, val.varint_size())
    }

    #[test]
    fn sizeof_i64_null() {
        let val: i64 = -1;
        assert_eq!(1, val.varint_size())
    }

    #[test]
    fn sizeof_i32_small() {
        assert_eq!(1, 14i32.varint_size())
    }

    #[test]
    fn sizeof_i32_larger() {
        assert_eq!(2, 150i32.varint_size())
    }
}
