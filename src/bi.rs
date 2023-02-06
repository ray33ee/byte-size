

//Convenient struct containing information on a match, including the index of the match (in whatever map it matched), the length of the match and whether a space was matched too
#[derive(PartialEq, Debug, Clone)]
pub (crate) struct Match {
    pub (crate) index: usize,
    pub (crate) length: usize,
    pub (crate) space: bool,
}

//Little trait that allow us to work generically over phf maps and the bimap (used for matching custom sequences)
//and match sequences with an optional space
pub (crate) trait Bi {

    fn get(&self, string: &  [u8]) -> Option<usize>;

    fn match_sequence(&self, string: & [u8], length: usize, offset: usize, space: bool) -> Option<Match> {
        if let Some(index) = self.get(&string[offset..length+offset]) {
            return Some(Match {
                index,
                length: length+offset,
                space,
            })
        }
        None
    }

    fn match_spaced_sequence(&self, string: &[u8], space: bool, length: usize) -> Option<Match> {
        if string.len() >= length + 1 {
            if space && string[0] == ' ' as u8 {
                return self.match_sequence(string, length, 1, true);
            }
        }

        if string.len() >= length {
            return self.match_sequence(string, length, 0, false);
        }

        None
    }
}

impl Bi for phf::OrderedMap<& 'static [u8], usize> {
    fn get(&self, string: & [u8]) -> Option<usize> {
        self.get(string).copied()
    }
}

impl Bi for bimap::BiHashMap<& 'static [u8], usize> {
    fn get(&self, string: & [u8]) -> Option<usize> {
        self.get_by_left(string).copied()
    }
}
