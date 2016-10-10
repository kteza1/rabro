use std::io::Read;
use error::Result;

pub trait Decodable: Sized {
    fn decode<R: Read>(reader: &mut R) -> Result<Self>;
}
