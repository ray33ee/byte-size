use std::ops::Deref;
use bincode::ErrorKind;
use thiserror::Error;

///Result using the [enum@Error]
pub type Result<T> = std::result::Result<T, Error>;

///Enum representing decompression errors
#[derive(Error, Debug)]
pub enum Error {

    ///Error created by the `write` macro from [std::fmt::Error] when converting a code into a string
    ///
    /// (Not really sure if this is needed as the errors from std::fmt are not well defined
    #[error("Error converting IR to string")]
    Format(#[from] std::fmt::Error),

    ///Raised when the deserializer expects more bytes than it gets.
    ///
    /// For example, a code starting with 240 (unicode) expects at least one byte to follow. If it doesn't, this error will be raised.
    #[error("Unexpected end of bytes. Deserialiser expected more bytes in the decompress slice")]
    UnexpectedEndOfBytes,

    ///If an invalid unicode sequence is detected by the deserializer
    #[error("Could not deserialize invalid unicode scalar value")]
    InvalidUnicodeChar,

    ///This error encompasses other [bincode] errors that are impossible or unlikely
    #[error("Unexpected bincode error")]
    OtherBincode(bincode::Error),
}

impl From<bincode::Error> for Error {
    fn from(value: bincode::Error) -> Self {
        match value.deref() {
            ErrorKind::InvalidCharEncoding => {
                Error::InvalidUnicodeChar
            }
            ErrorKind::SizeLimit => {
                Error::UnexpectedEndOfBytes
            }
            ErrorKind::Io(e) => {
                match e.kind() {
                    std::io::ErrorKind::UnexpectedEof => {
                        Error::UnexpectedEndOfBytes
                    }
                    _ => {
                        Error::OtherBincode(value)
                    }
                }
            }
            _ =>  {
                Error::OtherBincode(value)
            }
        }
    }
}