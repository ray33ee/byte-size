
use std::collections::{HashSet};
use std::fs::File;
use std::io::{Write};
use std::env;
use std::path::Path;

fn hash_generate_list<'a, I: IntoIterator<Item = & 'a str>>(list: I, name: &str, code: & mut String) {
    use std::fmt::Write;

    //Populate the lengths set with the lengths of all the lemmas in the list, and the phf builder with all (lemma, index) pairs
    let mut lengths = HashSet::new();
    let mut builder = phf_codegen::OrderedMap::new();
    let mut count = 0;

    for (i, lemma) in list.into_iter().enumerate() {
        builder.entry(lemma.as_bytes(), i.to_string().as_str());
        lengths.insert(lemma.len());
        count += 1;
    }

    //Turn the length set into a vector sorted from largest to smallest length
    let mut v: Vec<_> = lengths.iter().map(|x| *x).collect();
    v.sort();
    v.reverse();

    write!(code, "
pub (crate) struct {};

impl {} {{

    pub (crate) fn try_match_largest(space: bool, string: & [u8]) -> Option<crate::matcher::Match> {{
        use crate::matcher::Matcher;
        (Self::get_lemma_lengths(), Self::get_map()).try_match_largest(space, string)
    }}

    pub (crate) const fn get_length() -> usize {{
        {}
    }}

    pub (crate) fn get_index(index: usize) -> & 'static str {{
        unsafe {{ std::str::from_utf8_unchecked(Self::get_map().index(index).unwrap().0) }}
    }}

    fn get_lemma_lengths() -> & 'static [usize] {{
        static LENGTHS: [usize; {}] = [", name, name, count, lengths.len()).unwrap();

    for length in v.iter() {
        write!(code, "{}usize, ", *length).unwrap();
    }

    write!(code, "];
        LENGTHS.as_slice()
    }}

    fn get_map() -> & 'static phf::OrderedMap<&'static [u8], usize> {{

        static MAP: phf::OrderedMap<&'static [u8], usize> = {};

        &MAP
    }}


}}", builder.build()).unwrap();

}

fn main() {
    println!("cargo:rerun-if-changed=src/two_byte_common.txt");
    println!("cargo:rerun-if-changed=src/three_byte_uncommon.txt");


    //Here we take the two_byte_common.txt and three_byte_uncommon.txt files and convert them into phf tables
    let mut code = String::new();

    let s = std::fs::read_to_string(".\\two_byte_common.txt").unwrap();

    hash_generate_list(s.lines(), "TwoByteMap", & mut code);

    let s = std::fs::read_to_string(".\\three_byte_uncommon.txt").unwrap();

    hash_generate_list(s.lines(), "ThreeByteMap", & mut code);




    let mut fs = File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("maps.rs")).unwrap();

    fs.write_all(code.as_bytes()).unwrap();
}