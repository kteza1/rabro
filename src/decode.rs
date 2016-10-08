use std::io::{self, Read};
use error::{Error, Result};

pub trait Decodable: Sized {
    fn decode<R: Read>(reader: R) -> Result<Self>;
}

impl Decodable for bool {
    fn decode<R: Read>(reader: R) -> Result<Self> {
        let data = try!(try!(reader.bytes().next().ok_or(Error::InvalidBool)));

        match data & 0xFF {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(Error::InvalidBool),
        }
    }
}


#[cfg(test)]
mod test {
    use super::Decodable;
    use error::Error;
    use encode::Encodable;

    #[test]
    fn decode_bool() {
        let b: Vec<u8> = vec![0x01];
        assert_eq!(true, bool::decode(&b[..]).unwrap());

        let b: Vec<u8> = vec![0x00];
        assert_eq!(false, bool::decode(&b[..]).unwrap());

        let b: Vec<u8> = vec![0x0A];
        if let Err(e) = bool::decode(&b[..]) {
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
        assert_eq!(true, bool::decode(&b[..]).unwrap());

        //TODO: Buffer won't be overwritten in case you use
        // previous buffer
        let mut b: Vec<u8> = Vec::new();
        false.encode(&mut b);
        assert_eq!(false, bool::decode(&b[..]).unwrap());
    }
}