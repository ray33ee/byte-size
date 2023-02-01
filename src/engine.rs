use crate::builder::Builder;
use crate::iterator::CodeIterator;
use crate::error::Result;
use crate::ir::CodeType;

///Used to compress and decompress
///
/// Can only be created with the [Builder] struct via [Builder::engine]
pub struct Engine {
    pub (crate) custom: Vec<& 'static str>,
    pub (crate) custom_spaces: bool,
}

impl Engine {

    ///Compress the string using the builder options
    pub fn compress(&self, string: & str) -> Vec<u8> {
        let mut res = Vec::new();

        for code in CodeIterator::new(string, &self) {
            code.serialize_into(& mut res, &self);
        }

        res
    }

    ///Tries to decompress the byte slice.
    ///
    /// If successful, the decompressed string is returned. Otherwise a [Result] is returned.
    pub fn decompress(&self, mut bytes: & [u8]) -> Result<String> {
        let mut string = String::new();

        while !bytes.is_empty() {
            let code = CodeType::deserialize_from(& mut bytes, &self)?;

            code.add_to_string(& mut string, &self)?;
        }

        Ok(string)
    }

}

///Convenience function to compress a string using the [Builder::default] options
pub fn compress(string: &str) -> Vec<u8> {
    Builder::default().engine().compress(string)
}

///Convenience function to decompress a byte slice using the [Builder::default] options
pub fn decompress(bytes: & [u8]) -> Result<String> {
    Builder::default().engine().decompress(bytes)
}
