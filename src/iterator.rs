use std::str::from_utf8_unchecked;
use crate::engine::Engine;
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
            if Self::is_digit(*ch) /*|| i == self.main.len() - 1*/ {
                length = i+1;
            }
            if !Self::is_digit(*ch) || i == self.main.len() - 1 {
                break;
            }
        }

        //If the number is only 1, 2 or 3 bytes as a string, the conversion is not worth it
        if length <= 3 {
            return None;
        }

        println!("here '{}'", unsafe { std::str::from_utf8_unchecked(&self.main[0..length]) });

        //Attempt to convert this number into a u64.
        let large: u128 = unsafe { std::str::from_utf8_unchecked(&self.main[0..length]) }.parse().ok()?; //The bytes in &self.main[0..length] are all ascii numbers, so this unchecked is ok

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

    fn match_map(string: &[u8], space: bool, map: &phf::OrderedMap<& 'static [u8], usize>, length: usize) -> Option<Match> {
        if string.len() >= length + 1 {
            if space && string[0] == ' ' as u8 {
                if let Some(index) = map.get(&string[1..length+1]) {
                    return Some(Match {
                        index: *index,
                        length: length+1,
                        space: true,
                    })
                }
            }
        }

        if string.len() >= length {
            if let Some(index) = map.get(&string[..length]) {
                return Some(Match {
                    index: *index,
                    length,
                    space: false,
                })
            }
        }

        None
    }

    fn try_repetitions(&self) -> Option<(usize, Match)> {

        for length in crate::map::REPETITION_LENGTHS {
            let mut sub = self.main;

            let mut count = 0usize;
            let mut ind = None;

            loop {

                if sub.len() < length {
                    break;
                }

                if let Some(index) = crate::map::Repetitions::get_map().get(&sub[0..length]) {

                    if let Some(rolling) = ind {
                        if index != rolling {
                            break;
                        }
                    } else {
                        ind = Some(index);
                    }

                    count += 1;

                } else {
                    break;
                }


                sub = &sub[length..];
            }

            if count > 3 {
                println!("rep: {} count: {}", unsafe { from_utf8_unchecked(self.main) }, count);
                return Some((count, Match {
                    index: *ind.unwrap(),
                    length: count * length,
                    space: false
                }));
            }
        }

        None
    }

    fn encode_next(&self) -> (usize, CodeType) {

        //Basically the aim of this function is to pick the best way to encode the next chunk of bytes.
        //We use try_wonder, try_common and try_uncommon to create 3 possible types of encoding.
        //We then pick the most compact version (if all 3 work equally well, we pick the version that matches the largest string)


        //4. Try and match a custom string first, as custom strings should be chosen such that they are better stored as a two byte custom than as other forms
        if let Some(m) = self.engine.custom.as_slice().try_match_largest(self.engine.custom_spaces, self.main) {
            return (m.length, CodeType::Custom(m.space, m.index))
        }

        //1. Try match a repetition
        if let Some((count, m)) = self.try_repetitions() {
            return (m.length, CodeType::Repetitions(count as u32, m.index));
        }

        //2. Try and match a number. (make sure the number, converted back to a string matches)
        if let Some((number, length)) = self.try_number() {
            return (length, CodeType::Number(number))
        }

        // We try and match from all 3 maps together, starting from the largest length
        // We start with the largest length and the smallest map, this should mean
        // our result has the best length/cost ratio
        for length in crate::map::TOTAL_LENGTHS {

            if let Some(m) = Self::match_map(self.main, false, crate::map::OneByteMap::get_map(), length) {
                return (m.length, CodeType::OneByteWonder(m.index))
            }

            if let Some(m) = Self::match_map(self.main, true, crate::map::TwoByteMap::get_map(), length) {
                return (m.length, CodeType::TwoByteCommon(m.space, m.index))
            }

            if let Some(m) = Self::match_map(self.main, true, crate::map::ThreeByteMap::get_map(), length) {
                return (m.length, CodeType::ThreeByteUncommon(m.space, m.index))
            }

        }

        //3. Try and match a unicode character
        {
            let s = unsafe { from_utf8_unchecked(self.main) }; //We can use unchecked here because we will always start at a unicode boundary
            if let Some(first) = s.chars().nth(0) {
                if !first.is_ascii() {
                    return (first.len_utf8(), CodeType::UnicodeChar(first))
                }
            }
        }


        //8. Try and match one of the non-printables
        if let Some(index) = crate::map::Controls::get_map().get(& [self.main[0]]) {
            return (1, CodeType::Unprintable(*index))
        }

        //If none of the above encoding schemes work, we just encode a single ascii character
        (1, CodeType::OneByteWonder(self.main[0] as usize))


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

