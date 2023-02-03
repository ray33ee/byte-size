use crate::engine::Engine;
use crate::tables::{REPETITIONS};
use crate::error::Result;

#[derive(PartialEq)]
pub (crate) enum CodeType {
    ///Represents all possible values encoded with a single byte.
    /// This includes all ascii characters as well as a bunch of common sequences.
    ///
    /// Based off the [ONE_BYTE_WONDER] list
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

    Custom(bool, usize),
}

impl std::fmt::Debug for CodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeType::OneByteWonder(i) => {
                write!(f, "OneByteWonder({:?})", crate::map::OneByteMap::get_index(*i))
            }
            CodeType::TwoByteCommon(space, i) => {
                write!(f, "TwoByteCommon(\"{}{}\")", if *space { " " } else { "" }, crate::map::TwoByteMap::get_index(*i))
            }
            CodeType::ThreeByteUncommon(space, i) => {
                write!(f, "ThreeByteUncommon(\"{}{}\")", if *space { " " } else { "" }, crate::map::ThreeByteMap::get_index(*i))
            }
            CodeType::Number(num) => {
                write!(f, "Number({})", *num)
            }
            CodeType::UnicodeChar(ch) => {
                write!(f, "UnicodeChar({:?})", *ch)
            }
            CodeType::Unprintable(i) => {
                write!(f, "Unprintable({:?})", crate::map::Controls::get_index(*i))
            }
            CodeType::Repetitions(_, _) => {
                todo!()
            }
            CodeType::Custom(space, ind) => {
                write!(f, "Custom({}{})", if *space { " " } else { "" }, *ind)
            }
        }
    }
}

impl CodeType {
    pub fn add_to_string(&self, string: & mut String, engine: & Engine) -> Result<()> {
        use std::fmt::Write;

        match self {
            CodeType::OneByteWonder(index) => {
                write!(string, "{}", crate::map::OneByteMap::get_index(*index))?;
            }
            CodeType::TwoByteCommon(space, index) => {

                write!(string, "{}{}", if *space { " " } else { "" }, crate::map::TwoByteMap::get_index(*index) )?;
            }
            CodeType::ThreeByteUncommon(space, index) => {
                write!(string, "{}{}", if *space { " " } else { "" }, crate::map::ThreeByteMap::get_index(*index))?;
            }
            CodeType::UnicodeChar(ch) => {
                write!(string, "{}", *ch)?;
            }
            CodeType::Number(num) => {
                write!(string, "{}", *num)?;
            }
            CodeType::Unprintable(index) => {
                write!(string, "{}", crate::map::Controls::get_index(*index))?;
            }
            CodeType::Repetitions(count, index) => {
                for _ in 0..*count {
                    write!(string, "{}", REPETITIONS[*index])?;
                }
            }
            CodeType::Custom(space, index) => {
                write!(string, "{}{}", if *space { " " } else { "" }, engine.custom[*index])?;
            }
        }

        Ok(())
    }
}
