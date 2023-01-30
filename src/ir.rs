
use crate::tables::{ONE_BYTE_WONDER, TWO_BYTE_COMMON, THREE_BYTE_UNCOMMON};

pub (crate) enum CodeType {
    OneByteWonder(u32),
    TwoByteCommon(bool, u32),
    ThreeByteUncommon(bool, u32),
    UnicodeChar(char),
    Number(u128),
}

impl CodeType {
    pub fn len(&self) -> usize {
        match self {
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
        }
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
        }
    }
}