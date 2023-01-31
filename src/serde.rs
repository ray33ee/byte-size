use std::io::Read;
use crate::ir::CodeType;
use crate::tables::{CONTROLS, ONE_BYTE_WONDER, THREE_BYTE_UNCOMMON, TWO_BYTE_COMMON};

impl CodeType {

    const ONE_BYTE_WONDER_COUNT: usize = ONE_BYTE_WONDER.len();
    const REPETITION_COUNT: usize = 32;
    const NUMBER_COUNT: usize = 32;
    const UNICODE_COUNT: usize = 1; //Unicode only takes one value out of the one bytes
    const NON_PRINTABLE_COUNT: usize = CONTROLS.len();
    const TWO_BYTE_COUNT: usize = TWO_BYTE_COMMON.len();
    const THREE_BYTE_COUNT: usize = THREE_BYTE_UNCOMMON.len();

    pub (crate) fn deserialize_from<R: Read>(mut reader: R) -> Self {

        let first: u8 = bincode::deserialize_from(& mut reader).unwrap();

        if first < Self::ONE_BYTE_WONDER_COUNT as u8 {
            CodeType::OneByteWonder(first as usize)
        } else if first == Self::ONE_BYTE_WONDER_COUNT as u8 {
            //Unicode

            let ch: char = bincode::deserialize_from(& mut reader).unwrap();

            CodeType::UnicodeChar(ch)
        } else {
            let obw_index = first as usize - Self::ONE_BYTE_WONDER_COUNT - Self::UNICODE_COUNT;

            let second: u8 = bincode::deserialize_from(& mut reader).unwrap();

            let two_code = obw_index as usize * 256usize + second as usize;

            if two_code < Self::TWO_BYTE_COUNT*2 {
                CodeType::TwoByteCommon(two_code / Self::TWO_BYTE_COUNT != 0, two_code % Self::TWO_BYTE_COUNT)
            } else if two_code < Self::TWO_BYTE_COUNT*2 + Self::REPETITION_COUNT {

                let comb = two_code - Self::TWO_BYTE_COUNT*2;

                let third: u8 = bincode::deserialize_from(& mut reader).unwrap();

                CodeType::Repetitions(comb as u32, third as usize)
            } else if two_code < Self::TWO_BYTE_COUNT*2 + Self::REPETITION_COUNT + Self::NUMBER_COUNT {
                let comb = two_code - Self::TWO_BYTE_COUNT*2 - Self::REPETITION_COUNT;

                let four = comb / 8;
                let len = comb % 8 + 1;

                let mut num = four as u128;

                for i in 0..len {
                    let byte: u8 = bincode::deserialize_from(& mut reader).unwrap();

                    num += (byte as u128) << (i*8+2)
                }

                CodeType::Number(num)
            } else if two_code < Self::TWO_BYTE_COUNT*2 + Self::REPETITION_COUNT + Self::NUMBER_COUNT + Self::NON_PRINTABLE_COUNT {
                let comb = two_code - Self::TWO_BYTE_COUNT*2 - Self::REPETITION_COUNT - Self::NUMBER_COUNT;

                CodeType::Unprintable(comb)
            } else {
                let comb = two_code - Self::TWO_BYTE_COUNT*2 - Self::REPETITION_COUNT - Self::NUMBER_COUNT - Self::NON_PRINTABLE_COUNT;

                let third: u8 = bincode::deserialize_from(& mut reader).unwrap();

                let three_code = comb * 256 + third as usize;

                CodeType::ThreeByteUncommon(three_code / Self::THREE_BYTE_COUNT != 0, three_code % Self::THREE_BYTE_COUNT)
            }

        }


    }

    pub (crate) fn serialize_into(&self, bytes: & mut Vec<u8>) {

        if let CodeType::OneByteWonder(ind) = self {
            bytes.push(*ind as u8);
        } else {
            if let CodeType::UnicodeChar(c) = self {
                bytes.push(Self::ONE_BYTE_WONDER_COUNT as u8);
                bytes.extend_from_slice(c.to_string().as_bytes());
            } else {

                let (n, extra) = match self {
                    CodeType::TwoByteCommon(space, index) => {
                        let n = if *space {Self::TWO_BYTE_COUNT + *index as usize} else {*index as usize};
                        (n, None)
                    }
                    CodeType::Repetitions(count, repeat) => {
                        (*count as usize + Self::TWO_BYTE_COUNT*2, Some(vec![*repeat as u8]))
                    }
                    CodeType::Number(mut num) => {

                        let four = num % 4;

                        num = num >> 2;

                        let mut bytes = Vec::new();

                        while num != 0 {
                            bytes.push((num % 256) as u8);
                            num = num >> 8;
                        }

                        (four as usize * 8 + (bytes.len()-1) + Self::TWO_BYTE_COUNT*2 + Self::REPETITION_COUNT, Some(bytes))
                    }
                    CodeType::Unprintable(ind) => {
                        (*ind as usize + Self::TWO_BYTE_COUNT*2 + Self::REPETITION_COUNT + Self::NUMBER_COUNT, None)
                    }
                    CodeType::ThreeByteUncommon(space, ind) => {
                        let n = if *space {Self::THREE_BYTE_COUNT + *ind as usize} else {*ind as usize};

                        (Self::TWO_BYTE_COUNT*2 + Self::REPETITION_COUNT + Self::NUMBER_COUNT + Self::NON_PRINTABLE_COUNT + n/256, Some(vec![(n % 256) as u8]))
                    }
                    _ => {unreachable!()}
                };

                bytes.push((n / 256 + Self::ONE_BYTE_WONDER_COUNT + Self::UNICODE_COUNT) as u8);
                bytes.push((n % 256) as u8);

                if let Some(mut b) = extra {
                    bytes.append(& mut b);
                }


            }
        }
    }
}




