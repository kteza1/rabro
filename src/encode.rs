use std::io::{self, Write};
use error::{Error, Result};

pub trait Encodable {
    fn encode<W: Write>(self, writer: &mut W) -> Result<()>;
}

impl Encodable for bool {
    fn encode<W: Write>(self, writer: &mut W) -> Result<()> {
        try!(writer.flush());
        if self {
            try!(writer.write(&[0x01]))
        } else {
            try!(writer.write(&[0x00]))
        };
        Ok(())
    }
}