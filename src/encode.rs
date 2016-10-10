use std::io::Write;
use error::Result;

use byteorder::{BigEndian, WriteBytesExt};

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

pub trait Encodable {
    fn encode<W: Write>(self, writer: &mut W) -> Result<()>;
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
