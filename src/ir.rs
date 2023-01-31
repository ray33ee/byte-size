use std::fmt::{Formatter};
use crate::tables::{ONE_BYTE_WONDER, TWO_BYTE_COMMON, THREE_BYTE_UNCOMMON, CONTROLS};

#[derive(PartialEq)]
pub (crate) enum CodeType {
    ///Represents all possible values encoded with a single byte.
    /// This includes all ascii characters as well as a bunch of common sequences.
    ///
    /// Based off the `ONE_BYTE_WONDER` list
    OneByteWonder(usize),

    ///Represents all possible values encoded with two bytes.
    ///
    /// See readme for more information
    TwoByteCommon(bool, usize),

    ///Represents all possible values encoded with three bytes.
    ///
    /// See readme for more information
    ThreeByteUncommon(bool, usize),

    ///Represents all unicode scalar values.
    ///
    /// Takes up one more byte than the scalar value itself.
    UnicodeChar(char),

    ///Represents all numbers larger than 9999 (as between 1 and 8 bytes, inclusive, for numbers up to 2^66)
    Number(u128),

    ///Represents the unprintable ascii control bytes.
    ///
    /// Represented as 2 bytes
    Unprintable(usize),

    Repetitions(u32, usize),
}

impl CodeType {
    pub fn len(&self) -> usize {
        let l = match self {
            CodeType::OneByteWonder(_) => {1}
            CodeType::TwoByteCommon(_, _) => {2}
            CodeType::ThreeByteUncommon(_, _) => {3}
            CodeType::Number(n) => {
                let mut point = 1u128 << 10;
                let mut count = 3;
                while *n >= point {
                    point = point << 8;
                    count += 1;
                }
                //println!("Number {} Encoded with {} bytes.", n, count);
                count
            }
            CodeType::UnicodeChar(c) => {
                c.len_utf8()+1
            }
            CodeType::Unprintable(_) => {2}
            CodeType::Repetitions(_, _) => {3}
        };

        let mut v = Vec::new();

        self.serialize_into(& mut v);



        assert_eq!(l, v.len());

        l
    }
}

impl std::fmt::Debug for CodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeType::OneByteWonder(i) => {
                write!(f, "OneByteWonder({:?})", ONE_BYTE_WONDER[*i as usize])
            }
            CodeType::TwoByteCommon(space, i) => {
                write!(f, "TwoByteCommon(\"{}{}\")", if *space { " " } else { "" }, TWO_BYTE_COMMON[*i as usize])
            }
            CodeType::ThreeByteUncommon(space, i) => {
                write!(f, "ThreeByteUncommon(\"{}{}\")", if *space { " " } else { "" }, THREE_BYTE_UNCOMMON[*i as usize])
            }
            CodeType::Number(num) => {
                write!(f, "Number({})", *num)
            }
            CodeType::UnicodeChar(ch) => {
                write!(f, "UnicodeChar({:?})", *ch)
            }
            CodeType::Unprintable(ch) => {
                write!(f, "Unprintable(\'\\x{:02x}\')", CONTROLS[*ch as usize])
            }
            CodeType::Repetitions(_, _) => {
                todo!()
            }
        }
    }
}

impl std::fmt::Display for CodeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeType::OneByteWonder(index) => {
                write!(f, "{}", ONE_BYTE_WONDER[*index])
            }
            CodeType::TwoByteCommon(space, index) => {
                write!(f, "{}{}", if *space { " " } else { "" }, TWO_BYTE_COMMON[*index as usize])
            }
            CodeType::ThreeByteUncommon(space, index) => {
                write!(f, "{}{}", if *space { " " } else { "" }, THREE_BYTE_UNCOMMON[*index as usize])
            }
            CodeType::UnicodeChar(ch) => {
                write!(f, "{}", *ch)
            }
            CodeType::Number(num) => {
                write!(f, "{}", *num)
            }
            CodeType::Unprintable(index) => {
                write!(f, "{}", CONTROLS[*index as usize])
            }
            CodeType::Repetitions(_, _) => {
                todo!()
            }
        }
    }
}
