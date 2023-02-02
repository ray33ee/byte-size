use std::str::from_utf8_unchecked;
use crate::engine::Engine;
use crate::tables::{ONE_BYTE_WONDER, CONTROLS};
use crate::ir::{CodeType};
use crate::matcher::{Match, Matcher};

pub (crate) struct CodeIterator<'a> {
    main: & 'a [u8],
    engine: & 'a Engine,
}

impl<'a> CodeIterator<'a> {

    pub (crate) fn new(s: & 'a str, engine: & 'a Engine) -> Self {
        Self {
            main: s.as_bytes(),
            engine,
        }
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
            if let Some(first) = s.chars().nth(0) {
                if !first.is_ascii() {
                    return (first.len_utf8(), CodeType::UnicodeChar(first))
                }
            }
        }

        let list = [

            (2f32, self.engine.custom.as_slice().try_match_largest(self.engine.custom_spaces, self.main)),

            (2f32, crate::map::TwoByteMap::try_match_largest(true, self.main)),

            (3f32, crate::map::ThreeByteMap::try_match_largest(true, self.main)),

            (1f32, ONE_BYTE_WONDER.as_slice().try_match_largest(false, self.main)),

        ];

        let mut largest_ratio_triple = (0f32, None, Match { index: 0, length: 0, space: false });
        let mut largest_length_triple = (0f32, None, Match { index: 0, length: 0, space: false });
        let mut same = true;

        for (i, (cost, o_match)) in list.iter().enumerate() {
            if let Some(m) = o_match {
                let ratio = m.length as f32 / cost;

                if ratio > largest_ratio_triple.0 {
                    largest_ratio_triple = (ratio, Some(i), m.clone());
                }

                if m.length > largest_length_triple.2.length {
                    largest_length_triple = (ratio, Some(i), m.clone());
                }
            }
        }

        //Loop over the largest ratio ratio and see if its the same as all the other ratios
        for (cost, o_match) in list.iter() {
            if let Some(m) = o_match {
                let ratio = m.length as f32 / cost;
                same = same && (ratio == largest_ratio_triple.0)
            }
        }

        //If we have found the largest ratio
        if let Some(_) = largest_ratio_triple.1 {

            //We have a ratio that is strictly larger than all the others
            let chosen_triplet = if !same {
                largest_ratio_triple
            } else {
                largest_length_triple
            };

            match chosen_triplet.1.unwrap() { //This unwrap will always be ok because of the if let on largest_ratio_triple.2
                0 => {(chosen_triplet.2.length, CodeType::Custom(chosen_triplet.2.space, chosen_triplet.2.index))},
                3 => {(chosen_triplet.2.length, CodeType::OneByteWonder(chosen_triplet.2.index))},
                1 => {(chosen_triplet.2.length, CodeType::TwoByteCommon(chosen_triplet.2.space, chosen_triplet.2.index))},
                2 => {(chosen_triplet.2.length, CodeType::ThreeByteUncommon(chosen_triplet.2.space, chosen_triplet.2.index))},
                _ => {panic!("Invalid byte index ")}
            }

        } else {
            //It might be a non-printable

            if let Ok(pos) = CONTROLS.binary_search(&self.main[0]) {
                return (1, CodeType::Unprintable(pos))
            }


            //If none of the above encoding schemes work, we just encode a single ascii character
            (1, CodeType::OneByteWonder(self.main[0] as usize))
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

