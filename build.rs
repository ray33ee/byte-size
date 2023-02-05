
use std::collections::{HashSet};
use std::fs::{File, read_to_string};
use std::io::{Write};
use std::env;
use std::path::Path;

fn hash_generate_list<P: AsRef<Path>>(path: P, name: &str, code: & mut String, all_lengths: & mut HashSet<usize>) -> HashSet<usize> {
    use std::fmt::Write;

    let s = read_to_string(path.as_ref()).unwrap();

    //Populate the lengths set with the lengths of all the lemmas in the list, and the phf builder with all (lemma, index) pairs
    let mut lengths = HashSet::new();
    let mut builder = phf_codegen::OrderedMap::new();
    let mut count = 0;

    let mut slices = Vec::new();

    for lemma in s.lines() {
        slices.push(percent_encoding::percent_decode_str(lemma).decode_utf8().unwrap());
    }

    for  (i, lemma) in slices.iter().enumerate() {
        builder.entry(lemma.as_bytes(), i.to_string().as_str());
        lengths.insert(lemma.len());
        all_lengths.insert(lemma.len());
        count += 1;
    }

    write!(code, "
pub (crate) struct {};

impl {} {{

    pub (crate) const fn get_length() -> usize {{
        {}
    }}

    pub (crate) fn get_index(index: usize) -> & 'static str {{
        unsafe {{ std::str::from_utf8_unchecked(Self::get_map().index(index).unwrap().0) }}
    }}

    pub (crate) fn get_map() -> & 'static phf::OrderedMap<&'static [u8], usize> {{

        static MAP: phf::OrderedMap<&'static [u8], usize> = {};

        &MAP
    }}


}}", name, name, count, builder.build()).unwrap();

    lengths
}

fn main() {
    use std::fmt::Write;

    //println!("cargo:rerun-if-changed=obw.txt");
    println!("cargo:rerun-if-changed=tbc.txt");
    println!("cargo:rerun-if-changed=tbu.txt");
    println!("cargo:rerun-if-changed=controls.txt");
    println!("cargo:rerun-if-changed=repetitions.txt");

    let mut all_lengths = HashSet::new();

    //Here we take the two_byte_common.txt and three_byte_uncommon.txt files and convert them into phf tables
    let mut code = String::new();

    hash_generate_list("tbc.txt", "TwoByteMap", & mut code, & mut all_lengths);

    hash_generate_list("tbu.txt", "ThreeByteMap", & mut code, & mut all_lengths);

    hash_generate_list("obw.txt", "OneByteMap", & mut code, & mut all_lengths);

    hash_generate_list("controls.txt", "Controls", & mut code, & mut all_lengths);

    let rep_lengths = hash_generate_list(".\\repetitions.txt", "Repetitions", & mut code, & mut all_lengths);

    let mut rep_lengths: Vec<_> = rep_lengths.iter().collect();
    rep_lengths.sort();
    rep_lengths.reverse();

    //The only time a sequence of length 1 should appear is in the OBW list, and even that is technically just a single char
    all_lengths.remove(&1usize);

    let mut all_lengths: Vec<_> = all_lengths.iter().collect();
    all_lengths.sort();
    all_lengths.reverse();

    write!(& mut code, "pub (crate) const TOTAL_LENGTHS: [usize; {}] = [", all_lengths.len()).unwrap();

    for length in all_lengths {
        write!(& mut code, "{}usize, ", *length).unwrap();
    }

    write!(& mut code, "];\n\n").unwrap();

    write!(& mut code, "pub (crate) const REPETITION_LENGTHS: [usize; {}] = [", rep_lengths.len()).unwrap();

    for length in rep_lengths {
        write!(& mut code, "{}usize, ", *length).unwrap();
    }

    write!(& mut code, "];").unwrap();

    let mut fs = File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("maps.rs")).unwrap();

    fs.write_all(code.as_bytes()).unwrap();
}