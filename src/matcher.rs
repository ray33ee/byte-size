use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub (crate) struct Match {
    pub (crate) index: usize,
    pub (crate) length: usize,
    pub (crate) space: bool,
}

pub (crate) trait Matcher {
    ///A function that attempts to match &self with the beginning of the `string`, returning the largest match or None if there is no match
    fn try_match_largest(&self, space: bool, string: &[u8]) -> Option<Match>;
}

fn spaceless_match(string: &[u8], m: & [u8]) -> bool {

    if string.len() < m.len() {
        return false;
    }

    &string[..m.len()] == m
}

impl Matcher for & 'static str {
    fn try_match_largest(&self, space: bool, string: &[u8]) -> Option<Match> {
        //If space matching is enabled AND the first char is a space
        if space && string[0] == ' ' as u8 {
            if spaceless_match(&string[1..], self.as_bytes()) {
                return Some(Match {
                    index: 0,
                    length: self.len()+1,
                    space: true
                });
            }
        }

        if spaceless_match(string, self.as_bytes()) {
            Some(Match {
                index: 0,
                length: self.len(),
                space: false
            })
        } else {
            None
        }
    }
}

//Loop over each string in the list, and find the largest match
impl Matcher for & [& 'static str] {
    fn try_match_largest(&self, space: bool, string: &[u8]) -> Option<Match> {

        let mut largest: Option<Match> = None;

        for (i, possible) in (*self).iter().enumerate() {
            if let Some(m) = possible.try_match_largest(space, string) {

                if let Some(l) = &largest {
                    if m.length > l.length {
                        largest = Some(Match {
                            index: i,
                            length: m.length,
                            space: m.space
                        });
                    }
                } else {
                    largest = Some(Match {
                        index: i,
                        length: m.length,
                        space: m.space
                    });
                }


            }
        }

        largest
    }
}

//Takes a list of (length, hashmap) pairs ordered by decreasing length
impl Matcher for & [(usize, HashMap<& [u8], usize>)] {
    fn try_match_largest(&self, space: bool, string: &[u8]) -> Option<Match> {

        for (length, map) in *self {


            if string.len() < *length + if space {1} else {0} {
                continue;
            }

            if space && string[0] == ' ' as u8 {
                if let Some(index) = map.get(&string[1..*length+1]) {
                    return Some(Match {
                        index: *index,
                        length: *length+1,
                        space: true,
                    })
                }
            }

            if let Some(index) = map.get(&string[..*length]) {
                return Some(Match {
                    index: *index,
                    length: *length,
                    space: false,
                })
            }

        }

        None
    }
}
