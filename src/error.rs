use std::result;
use std::io;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Avro,
    Io(io::Error),
    InvalidBool,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}
