use encode::Encodable;
use decode::Decodable;

use std::io::{Write, Read};
use std::iter::repeat;

use error::{Error, Result};

use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

pub fn encode_zig_zag(num: i64) -> u64 {
    if num < 0 {
        !((num as u64) << 1)
    } else {
        (num as u64) << 1
    }
}

pub fn encode_var_len_u64<W: Write>(writer: &mut W, mut num: u64) -> Result<()> {
    loop {
        let mut b = (num & 0b0111_1111) as u8;
        num >>= 7;
        if num == 0 {
            try!(writer.write(&[b]));
            break;
        }
        b |= 0b1000_0000;
        try!(writer.write(&[b]));
    }
    Ok(())
}

impl Encodable for bool {
    fn encode<W: Write>(self, writer: &mut W) -> Result<()> {
        if self {
            try!(writer.write(&[0x01]))
        } else {
            try!(writer.write(&[0x00]))
        };
        Ok(())
    }
}

impl Encodable for i32 {
    fn encode<W: Write>(self, writer: &mut W) -> Result<()> {
        let zigzag = encode_zig_zag(self as i64);
        try!(encode_var_len_u64(writer, zigzag));
        Ok(())
    }
}

impl Encodable for i64 {
    fn encode<W: Write>(self, writer: &mut W) -> Result<()> {
        let zigzag = encode_zig_zag(self);
        try!(encode_var_len_u64(writer, zigzag));
        Ok(())
    }
}

impl Encodable for f32 {
    fn encode<W: Write>(self, writer: &mut W) -> Result<()> {
        try!(writer.write_f32::<BigEndian>(self));
        Ok(())
    }
}


impl Encodable for f64 {
    fn encode<W: Write>(self, writer: &mut W) -> Result<()> {
        try!(writer.write_f64::<BigEndian>(self));
        Ok(())
    }
}

impl Encodable for Vec<u8> {
    fn encode<W: Write>(self, writer: &mut W) -> Result<()> {
        let zigzag = encode_zig_zag(self.len() as i64);
        try!(encode_var_len_u64(writer, zigzag));
        try!(writer.write_all(&self[..]));
        Ok(())
    }
}


pub fn decode_zig_zag(num: u64) -> i64 {
    if num & 1 == 1 {
        !(num >> 1) as i64
    } else {
        (num >> 1) as i64
    }
}

pub fn decode_var_len_u64<R: Read>(reader: &mut R) -> Result<u64> {
    let mut num = 0;
    let mut i = 0;
    loop {
        let mut buf = [0u8; 1];
        try!(reader.read_exact(&mut buf));
        // If the 10th byte has any of bits 1 to 6 set or the high bit set, report an error
        if i >= 9 && buf[0] & 0b1111_1110 != 0 {
            // 10th byte
            return Err(Error::IntegerOverflow);
        }
        num |= (buf[0] as u64 & 0b0111_1111) << (i * 7);
        if buf[0] & 0b1000_0000 == 0 {
            break;
        }
        i += 1;
    }
    Ok(num)
}

impl Decodable for bool {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        let data = try!(try!(reader.bytes().next().ok_or(Error::InvalidBool)));

        match data & 0xFF {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::InvalidBool),
        }
    }
}

impl Decodable for i32 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        let decoded = decode_zig_zag(try!(decode_var_len_u64(reader))) as Self;
        Ok(decoded)
    }
}

impl Decodable for i64 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        let decoded = decode_zig_zag(try!(decode_var_len_u64(reader)));
        Ok(decoded)
    }
}

impl Decodable for f32 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        let decoded = try!(reader.read_f32::<BigEndian>());
        Ok(decoded)
    }
}

impl Decodable for f64 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        let decoded = try!(reader.read_f64::<BigEndian>());
        Ok(decoded)
    }
}

impl Decodable for Vec<u8> {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        let len = decode_zig_zag(try!(decode_var_len_u64(reader)));
        let mut decoded: Vec<u8> = repeat(0).take(len as usize).collect();
        try!(reader.read_exact(&mut decoded));
        Ok(decoded)
    }
}


#[cfg(test)]
mod test {
    #![allow(unused_variables, unused_must_use)]
    use decode::Decodable;
    use error::Error;
    use encode::Encodable;

    #[test]
    fn decode_bool() {
        let b: Vec<u8> = vec![0x01];
        assert_eq!(true, bool::decode(&mut &b[..]).unwrap());

        let b: Vec<u8> = vec![0x00];
        assert_eq!(false, bool::decode(&mut &b[..]).unwrap());

        let b: Vec<u8> = vec![0x0A];
        if let Err(e) = bool::decode(&mut &b[..]) {
            match e {
                Error::InvalidBool => assert!(true),
                _ => assert!(false),
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn encode_decode_bool() {
        let mut b: Vec<u8> = Vec::new();
        true.encode(&mut b);
        assert_eq!(true, bool::decode(&mut &b[..]).unwrap());

        // TODO: Buffer won't be overwritten in case you use
        // previous buffer
        let mut b: Vec<u8> = Vec::new();
        false.encode(&mut b);
        assert_eq!(false, bool::decode(&mut &b[..]).unwrap());
    }

    #[test]
    fn encode_decode_i32() {
        let to_encode = vec![100, -100, 1000, -1000];

        for v in to_encode {
            let mut e: Vec<u8> = Vec::new();
            v.encode(&mut e);
            let d = i32::decode(&mut &e[..]).unwrap();
            assert_eq!(v, d);
        }
    }

    #[test]
    fn encode_decode_i64() {
        let to_encode = vec![100, -100, 1000, -1000];

        for v in to_encode {
            let mut e: Vec<u8> = Vec::new();
            v.encode(&mut e);
            let d = i64::decode(&mut &e[..]).unwrap();
            assert_eq!(v, d);
        }
    }

    #[test]
    fn encode_decode_f32() {
        let to_encode = vec![100.1, -100.2, 1000.3, -1000.4];

        for v in to_encode {
            let mut e: Vec<u8> = Vec::new();
            v.encode(&mut e);
            let d = f32::decode(&mut &e[..]).unwrap();
            assert_eq!(v, d);
        }
    }

    #[test]
    fn encode_decode_f64() {
        let to_encode = vec![100.1, -100.2, 1000.3, -1000.4];

        for v in to_encode {
            let mut e: Vec<u8> = Vec::new();
            v.encode(&mut e);
            let d = f64::decode(&mut &e[..]).unwrap();
            assert_eq!(v, d);
        }
    }

    #[test]
    fn encode_decode_byte_array() {
        let input: Vec<u8> = vec![1, 2, 3, 4];

        let mut e: Vec<u8> = Vec::new();
        input.clone().encode(&mut e);

        let d = Vec::<u8>::decode(&mut &e[..]).unwrap();
        assert_eq!(input, d);
    }
}
