use std::io::Write;
use error::Result;

pub trait Encodable {
    fn encode<W: Write>(self, writer: &mut W) -> Result<()>;
}
