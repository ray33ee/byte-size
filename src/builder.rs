use crate::ir::CodeType;
use crate::iterator::CodeIterator;
use crate::error::Result;

///Compress/decompress with specific options
///
/// Use the builder class to compress and decompress using specific options.
///
/// If you're not sure about the options, either use `Builder::default()` or use the convenience functions `crate::compress` and `crate::decompress`
pub struct Builder {
    pub (crate) custom: Vec<& 'static str>,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            custom: vec!["http://", "https://", ".com", "\n\r\n", "\r\n\r", "C:\\", ".co.uk"],
        }
    }
}

impl Builder {

    ///Create a builder with an empty custom list
    pub fn empty() -> Self {
        Self {
            custom: Vec::new(),
        }
    }

    ///Move in a new list of custom words
    ///
    /// sss supports the use of 32 custom strings which are encoded as two bytes. This list can be replaced completely with this `Builder::set_custom` function,
    /// or it can be modified with the `Builder::push_custom` or `Builder::clear_custom` functions.
    ///
    /// Note: The protocol only supports 32 custom strings, so only the first 32 strings will be used in the custom vector. Adding more than 32 is not an error, but these extra strings will not be used.
    pub fn set_custom(& mut self, list: Vec<& 'static str>) -> & mut Self {
        self.custom = list;
        self
    }

    ///Appends a single string to the custom list. See the `Builder::set_custom` for more information on custom strings.
    pub fn push_custom(& mut self, custom: & 'static str) -> & mut Self {
        self.custom.push(custom);
        self
    }

    ///Clears the custom list. See the `Builder::set_custom` for more information on custom strings.
    pub fn clear_custom(& mut self) -> & mut Self {
        self.custom.clear();
        self
    }

    ///Returns the current length of the custom string list
    pub fn len_custom(& self) -> usize {
        self.custom.len()
    }

    ///Compress the string using the builder options
    pub fn compress(&self, string: & str) -> Vec<u8> {
        let mut res = Vec::new();

        for code in CodeIterator::new(string, &self) {
            code.serialize_into(& mut res);
        }

        res
    }

    ///Tries to decompress the byte slice. If successful, the decompressed string is returned. Otherwise an `crate::error:Error` is returned.
    pub fn decompress(&self, mut bytes: & [u8]) -> Result<String> {
        let mut string = String::new();

        while !bytes.is_empty() {
            let code = CodeType::deserialize_from(& mut bytes)?;

            code.add_to_string(& mut string, &self)?;
        }

        Ok(string)
    }

}

///Convenience function to compress a string using the `Builder::default` options
pub fn compress(string: &str) -> Vec<u8> {
    Builder::default().compress(string)
}

///Convenience function to decompress a byte slice using the `Builder::default` options
pub fn decompress(bytes: & [u8]) -> Result<String> {
    Builder::default().decompress(bytes)
}
