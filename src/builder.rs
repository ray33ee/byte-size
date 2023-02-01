
use crate::engine::Engine;

///Compress/decompress with specific options
///
/// Use the builder class to compress and decompress using specific options.
///
/// If you're not sure about the options, either use [Builder::default] or use the convenience functions [Engine::compress] and [Engine::decompress]
pub struct Builder {
    custom: Vec<& 'static str>,
    custom_spaces: bool,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            custom: vec!["http://", "https://", ".com", "\n\r\n", "\r\n\r", "C:\\", ".co.uk"],
            custom_spaces: false,
        }
    }
}

impl Builder {

    ///Create a builder with an empty custom list
    pub fn empty() -> Self {
        Self {
            custom: Vec::new(),
            custom_spaces: false,
        }
    }



    ///Move in a new list of custom words
    ///
    /// sss supports the use of 32 custom strings which are encoded as two bytes. This list can be replaced completely with this [Builder::set_custom] function,
    /// or it can be modified with the [Builder::push_custom] or [Builder::clear_custom] functions.
    ///
    /// Note: The protocol only supports 32 custom strings, so only the first 32 strings will be used in the custom vector. Adding more than 32 is not an error, but these extra strings will not be used.
    pub fn set_custom(& mut self, list: Vec<& 'static str>) -> & mut Self {
        self.custom = list;
        self
    }

    ///Determines whether the custom words will automatically support space prefixes.
    ///
    /// If true, the maximum number of possible custom strings in the table is halved from 32 to 16.
    pub fn set_custom_spaces(& mut self, spaces: bool) -> & mut Self {
        self.custom_spaces = spaces;
        self
    }

    ///Appends a single string to the custom list. See the [Builder::set_custom] for more information on custom strings.
    pub fn push_custom(& mut self, custom: & 'static str) -> & mut Self {
        self.custom.push(custom);
        self
    }

    ///Clears the custom list. See the [Builder::set_custom] for more information on custom strings.
    pub fn clear_custom(& mut self) -> & mut Self {
        self.custom.clear();
        self
    }

    ///Returns the current length of the custom string list
    pub fn len_custom(& self) -> usize {
        self.custom.len()
    }

    ///Converts the builder into an [Engine]
    pub fn engine(&self) -> Engine {

        let max_len = if self.custom_spaces { 16 } else { 32 };

        let v = if self.custom.len() <= max_len { self.custom.clone() } else { (&self.custom[0..max_len]).to_vec() };

        Engine {
            custom: v,
            custom_spaces: self.custom_spaces,
        }
    }

}
