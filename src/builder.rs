use crate::ir::CodeType;
use crate::iterator::CodeIterator;

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

    pub fn empty() -> Self {
        Self {
            custom: Vec::new(),
        }
    }

    pub fn set_custom(& mut self, list: Vec<& 'static str>) -> & mut Self {
        self.custom = list;
        self
    }

    pub fn push_custom(& mut self, custom: & 'static str) -> & mut Self {
        self.custom.push(custom);
        self
    }

    pub fn clear_custom(& mut self) -> & mut Self {
        self.custom.clear();
        self
    }

    pub fn compress(&self, string: & str) -> Vec<u8> {
        let mut res = Vec::new();

        for code in CodeIterator::new(string, &self) {
            code.serialize_into(& mut res);
        }

        res
    }

    pub fn decompress(&self, mut bytes: & [u8]) -> String {
        let mut string = String::new();

        while !bytes.is_empty() {
            let code = CodeType::deserialize_from(& mut bytes);

            code.add_to_string(& mut string, &self).unwrap();
        }

        string
    }

}

pub fn compress(string: &str) -> Vec<u8> {
    Builder::default().compress(string)
}

pub fn decompress(bytes: & [u8]) -> String {
    Builder::default().decompress(bytes)
}
