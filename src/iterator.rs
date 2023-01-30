use std::str::from_utf8_unchecked;
use crate::tables::{ONE_BYTE_WONDER, TWO_BYTE_COMMON, THREE_BYTE_UNCOMMON, CONTROLS};
use crate::ir::{CodeType};

pub (crate) struct CodeIterator<'a> {
    main: & 'a [u8],
}

impl<'a> CodeIterator<'a> {

    pub (crate) fn new(s: & 'a str) -> Self {
        Self {
            main: s.as_bytes(),
        }
    }

    fn compare(&self, small: &[u8]) -> bool {
        self.gen_compare(small, 0)
    }

    fn gen_compare(&self, small: &[u8], off: usize) -> bool {
        if self.main.len() < small.len() +off {
            return false
        }
        &self.main[off..small.len()+off] == small
    }

    fn compare_space(&self, small: &[u8]) -> bool {
        self.main[0] == ' ' as u8 && self.gen_compare(small, 1)
    }

    fn is_digit(ch: u8) -> bool {
        ch >= '0' as u8 && ch <= '9' as u8
    }

    fn try_number(&self) -> Option<(u128, usize)> {

        //Attempt to match the largest number we can [0-9]{3,}

        if !Self::is_digit(self.main[0]) {
            return None;
        }

        let mut length = 0;

        for (i, ch) in (&self.main[..]).iter().enumerate() {
            if !Self::is_digit(*ch) || i == self.main.len() - 1 {
                length = i+1;
                break;
            }
        }

        //Attempt to convert this number into a u64.
        let large: u128 = unsafe { std::str::from_utf8_unchecked(&self.main[0..length]) }.parse().ok()?;

        //Since numbers require at least 3 bytes to encode, numbers less than 1000 should not be encoded
        if large < 1000 {
            return None;
        }

        //Make sure it fits in 42 bits
        if large >= (2 << 66) {
            return None;
        }

        //Convert this number back into a string, make sure it is the same as the original
        if large.to_string().as_bytes() != &self.main[..length] {
            return None;
        }

        Some((large, length))
    }

    fn try_wonder(&self) -> Option<(u32, usize, f32, bool)> {
        for (i, entry) in ONE_BYTE_WONDER.iter().enumerate() {
            if i < 32 || i > 126 {
                if self.compare(entry.as_bytes()) {
                    return Some((i as u32, entry.len(), entry.len() as f32, false))
                }
            }
        }

        None
    }

    fn try_lemma_dict(&self, dict: &[&str], cost: f32, space: bool) -> Option<(u32, usize, f32, bool)> {
        let mut largest = None;
        for (i, entry) in dict.iter().enumerate() {
            if self.compare(entry.as_bytes()) {
                match largest {
                    None => {
                        largest = Some((i, entry.len(), false))
                    }
                    Some((_, l_length, _)) => {
                        if entry.len() > l_length {
                            largest = Some((i, entry.len(), false))
                        }
                    }
                }
            }

            if self.compare_space(entry.as_bytes()) && space {
                match largest {
                    None => {
                        largest = Some((i, entry.len()+1, true))
                    }
                    Some((_, l_length, _)) => {
                        if entry.len() > l_length {
                            largest = Some((i, entry.len()+1, true))
                        }
                    }
                }
            }
        }

        largest.map(|(index, length, space)| (index as u32, length, length as f32 / cost, space))
    }

    fn encode_next(&self) -> (usize, CodeType) {

        //Basically the aim of this function is to pick the best way to encode the next chunk of bytes.
        //We use try_wonder, try_common and try_uncommon to create 3 possible types of encoding.
        //We then pick the most compact version (if all 3 work equally well, we pick the version that matches the largest string)

        //Try and match a number. (make sure the number, converted back to a string matches)
        if let Some((number, length)) = self.try_number() {
            return (length, CodeType::Number(number))
        }

        //If we have a non-ascii character, encode as a unicode scalar value
        {
            let s = unsafe { from_utf8_unchecked(self.main) };
            let first = s.chars().nth(0).unwrap();
            if !first.is_ascii() {
                return (first.len_utf8(), CodeType::UnicodeChar(first))
            }
        }

        let list = [self.try_lemma_dict(TWO_BYTE_COMMON.as_slice(), 2f32, true), self.try_lemma_dict(THREE_BYTE_UNCOMMON.as_slice(), 3f32, true), self.try_wonder()];

        let mut largest_ratio_triple = (0f32, 0usize, None, 0u32, false);
        let mut largest_length_triple = (0f32, 0usize, None, 0u32, false);
        let mut same = true;

        for (i, item) in list.iter().enumerate() {
            if let Some((index, length, ratio, space)) = item {
                if *ratio > largest_ratio_triple.0 {
                    largest_ratio_triple = (*ratio, *length, Some(i), *index, *space);
                }

                if *length > largest_length_triple.1 {
                    largest_length_triple = (*ratio, *length, Some(i), *index, *space);
                }
            }
        }

        //Loop over the largest ratio ratio and see if its the same as all the other ratios
        for item in list.iter() {
            if let Some((_, _, ratio, _)) = item {
                same = same && (*ratio == largest_ratio_triple.0)
            }
        }

        //If we have found the largest ratio
        if let Some(_) = largest_ratio_triple.2 {

            //We have a ratio that is strictly larger than all the others
            let chosen_triplet = if !same {
                largest_ratio_triple
            } else {
                largest_length_triple
            };

            match chosen_triplet.2.unwrap() {
                2 => {(chosen_triplet.1, CodeType::OneByteWonder(chosen_triplet.3))},
                0 => {(chosen_triplet.1, CodeType::TwoByteCommon(chosen_triplet.4, chosen_triplet.3))},
                1 => {(chosen_triplet.1, CodeType::ThreeByteUncommon(chosen_triplet.4, chosen_triplet.3))},
                _ => {panic!("Invalid byte index ")}
            }

        } else {
            //It might be a non-printable

            if let Ok(pos) = CONTROLS.binary_search(&self.main[0]) {
                return (1, CodeType::Unprintable(pos as u32))
            }


            //If none of the above encoding schemes work, we just encode a single ascii character
            (1, CodeType::OneByteWonder(self.main[0] as u32))
        }

    }
}

impl Iterator for CodeIterator<'_> {
    type Item = CodeType;

    fn next(&mut self) -> Option<Self::Item> {

        if self.main.len() == 0 {
            return None;
        }

        let (length, t) = self.encode_next();

        self.main = &self.main[length..];

        Some(t)
    }
}

