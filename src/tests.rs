#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::builder::Builder;
    use crate::engine::Engine;
    use crate::ir::CodeType;
    use crate::iterator::CodeIterator;
    use crate::matcher::Matcher;
    use crate::tables::{THREE_BYTE_UNCOMMON, TWO_BYTE_COMMON};

    fn full_ser_deser_builder(string: &str, engine: & Engine) {
        use smaz::compress;

        let bytes = engine.compress(string);

        let x = engine.decompress(bytes.as_slice()).unwrap();

        let smaz_len = compress(string.as_bytes()).len();
        let code_len = bytes.len();

        println!("String: '{}' ({:?})", string, string);
        println!("    Compression:           {:?}", bytes);
        println!("    Code Points:           {:?}", CodeIterator::new(string, &engine).collect::<Vec<_>>());
        println!("    Compression size:      {} ({}% compression ratio)", code_len, 100f32 - code_len as f32 / string.as_bytes().len() as f32 * 100f32);
        println!("    Smaz Compression size: {} ({}% compression ratio)", smaz_len, 100f32 - smaz_len as f32 / string.as_bytes().len() as f32 * 100f32);

        assert_eq!(string, x.as_str());

        assert!(code_len <= smaz_len)

    }

    fn full_ser_deser(string: & str) {
        full_ser_deser_builder(string, &Builder::default().engine())
    }

    #[test]
    fn number_test() {
        full_ser_deser("6709376338672");
    }

    #[test]
    fn http1() {
        full_ser_deser("http://google.com");
    }

    #[test]
    fn http2() {
        full_ser_deser("http://programming.reddit.com");
    }

    #[test]
    fn http3() {
        full_ser_deser("http://github.com/antirez/smaz/tree/master");
    }

    #[test]
    fn ascii_control_test() {
        full_ser_deser("\x01");
    }

    #[test]
    fn message_test() {
        full_ser_deser("yeh thats fine mate 🙂");
    }

    #[test]
    fn unicode_test() {
        full_ser_deser("✔️ ❤️ ☆");
    }

    #[test]
    fn fox() {
        full_ser_deser("The quick brown fox jumped over the lazy dog");
    }

    #[test]
    fn therefore() {
        full_ser_deser("therefore");
    }

    #[test]
    fn download() {
        full_ser_deser("download");
    }

    #[test]
    fn smaz1() {
        full_ser_deser("This is a small string");
    }

    #[test]
    fn smaz2() {
        full_ser_deser("foobar");
    }

    #[test]
    fn smaz3() {
        full_ser_deser("the end");
    }

    #[test]
    fn smaz4() {
        full_ser_deser("not-a-g00d-Exampl333");
    }

    #[test]
    fn smaz5() {
        full_ser_deser("Smaz is a simple compression library");
    }

    #[test]
    fn smaz6() {
        full_ser_deser("Nothing is more difficult, and therefore more precious, than to be able to decide");
    }

    #[test]
    fn smaz7() {
        full_ser_deser("this is an example of what works very well with smaz");
    }

    #[test]
    fn smaz8() {
        full_ser_deser("1000 numbers 2000 will 10 20 30 compress very little");
    }

    #[test]
    fn smaz9() {
        full_ser_deser("Smaz is a simple compression library suitable for compressing very short
strings. General purpose compression libraries will build the state needed
for compressing data dynamically, in order to be able to compress every kind
of data. This is a very good idea, but not for a specific problem: compressing
small strings will not work.");
    }

    #[test]
    fn smaz10() {
        full_ser_deser("small string shrinker");
    }

    #[test]
    fn smaz11() {
        full_ser_deser("their");
    }

    fn serialize(string: &str, bytes: & [u8]) {

        let mut v: Vec<u8> = Vec::new();
        let engine = Builder::default().engine();

        for code in CodeIterator::new(string, &engine) {
            print!("{:?}, ", code);
            code.serialize_into(& mut v, &engine);
        }

        assert_eq!(v.as_slice(), bytes)
    }

    #[test]
    fn serialize_obw_test1() { serialize("r", ['r' as u8].as_slice()) }

    #[test]
    fn serialize_obw_test2() { serialize("the", [1].as_slice()) }




    #[test]
    fn serialize_tbc_test1() { serialize("that", [209, 116].as_slice()) }

    #[test]
    fn serialize_tbc_test10() { serialize(" that", [248, 1].as_slice()) }

    #[test]
    fn serialize_tbc_test2() { serialize(TWO_BYTE_COMMON[255], [241, 255].as_slice()) }

    #[test]
    fn serialize_tbc_test3() { serialize(TWO_BYTE_COMMON[256], [242, 0].as_slice()) }

    #[test]
    fn serialize_control_test1() { serialize("\x01", [255, 98].as_slice()) }

    #[test]
    fn serialize_tbu_test1() { serialize("trading", [255, 127, 19].as_slice()) }

    #[test]
    fn serialize_tbu_test2() { serialize(THREE_BYTE_UNCOMMON[256], [255, 128, 0].as_slice()) }

    #[test]
    fn serialize_num_test1() { serialize("1023", [255, 90, 255].as_slice()) }

    #[test]
    fn serialize_num_test2() { serialize("6577567", [255, 92, 103, 23, 25].as_slice()) }

    #[test]
    fn serialize_uni_test2() { serialize("❤", [240, 226, 157, 164].as_slice()) }

    fn single_code_ser_deser(string: &str) {
        let code = CodeIterator::new(string, &Builder::default().engine()).nth(0).unwrap();

        let mut v: Vec<u8> = Vec::new();

        code.serialize_into(& mut v, &Builder::default().engine());

        println!("Bytes: {:?}", v);

        let code2 = CodeType::deserialize_from(&v[..], &Builder::default().engine()).unwrap();

        assert_eq!(code, code2)

    }

    #[test]
    fn test_single_the() { single_code_ser_deser("the") }

    #[test]
    fn test_single_sexes() { single_code_ser_deser("sexes") }

    #[test]
    fn test_single_unicode() { single_code_ser_deser("❤") }

    #[test]
    fn test_single_unprintable() { single_code_ser_deser("\x01") }

    #[test]
    fn test_single_tbu() { single_code_ser_deser("trading") }

    #[test]
    fn test_single_num() { single_code_ser_deser("1000") }

    #[test]
    fn test_single_num1() { single_code_ser_deser("100000000") }

    #[test]
    fn test_single_num2() { single_code_ser_deser("1000000000000000000") }

    #[test]
    fn test_single_tbc1() { single_code_ser_deser(TWO_BYTE_COMMON[0]) }

    #[test]
    fn test_single_tbc2() { single_code_ser_deser(TWO_BYTE_COMMON[TWO_BYTE_COMMON.len()-1]) }

    #[test]
    fn test_single_tbu1() { single_code_ser_deser(THREE_BYTE_UNCOMMON[0]) }

    #[test]
    fn test_single_tbu2() { single_code_ser_deser(THREE_BYTE_UNCOMMON[THREE_BYTE_UNCOMMON.len()-1]) }

    #[test]
    fn test_single_trading() { single_code_ser_deser("trading") }

    #[test]
    fn test() {
        let bytes: [u8; 2] = [255, 98];
        let code = CodeType::deserialize_from(bytes.as_slice(), &Builder::default().engine()).unwrap();
        println!("{:?} {}", code, "\x01");
        assert_eq!(code, CodeType::Unprintable(0))
    }

    #[test]
    #[should_panic]
    fn test_bad_double() {
        crate::engine::decompress([255].as_slice()).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_bad_unicode() {
        crate::engine::decompress([240, 0x80, 0x81].as_slice()).unwrap();
    }

    #[test]
    fn test_custom_space() {
        full_ser_deser_builder(" customstringspacetest", &Builder::default().set_custom_spaces(true).push_custom("customstringspacetest").engine())
    }

    #[test]
    fn test_custom_space2() {
        full_ser_deser_builder(" customstringspacetest", &Builder::default().set_custom_spaces(false).push_custom("customstringspacetest").engine())
    }

    #[test]
    fn test_custom_space3() {
        full_ser_deser_builder("customstringspacetest", &Builder::default().set_custom_spaces(false).push_custom("customstringspacetest").engine())
    }

    fn matcher_test<M: Matcher>(matcher: M) {
        use crate::matcher::Match;

        for _ in 0..1000 {
            let m = matcher.try_match_largest(false, b"address are in this thingy").unwrap();
            assert_eq!(m, Match {
                index: 207,
                length: 7,
                space: false
            });

            let m = matcher.try_match_largest(true, b" address are in this thingy").unwrap();
            assert_eq!(m, Match {
                index: 207,
                length: 8,
                space: true
            });

            let m = matcher.try_match_largest(true, b"address are in this thingy").unwrap();
            assert_eq!(m, Match {
                index: 207,
                length: 7,
                space: false
            });

            let m = matcher.try_match_largest(true, b"library this thingy").unwrap();
            assert_eq!(m, Match {
                index: 419,
                length: 7,
                space: false
            });

            assert_eq!(matcher.try_match_largest(true, b"n this thingy"), None);
        }
    }

    #[test]
    fn linear_search_matching() {

        let slice = TWO_BYTE_COMMON.as_slice();

        matcher_test(slice);
    }

    #[test]
    fn hash_map_matching() {

        let mut pairs = HashMap::new();

        for (i, lemma) in TWO_BYTE_COMMON.iter().enumerate() {
            let bytes = lemma.as_bytes();
            let len = bytes.len();

            if !pairs.contains_key(&len) {
                pairs.insert(len, HashMap::new());
            }

            let map = pairs.get_mut(&len).unwrap();

            map.insert(bytes, i);

        }

        let mut v: Vec<_> = pairs.iter().map(|(length, map)| (*length, map.clone())).collect();
        v.sort_by_key(|a| a.0);
        v.reverse();

        println!("{}", TWO_BYTE_COMMON[105]);

        matcher_test(v.as_slice());
    }

    #[test]
    #[ignore]
    fn test_list() {
        use smaz::compress;

        let str = std::fs::read_to_string(".\\.3m.txt").unwrap();

        let max = 10000;
        let mut count = 0;
        let mut total = 0;


        for line in str.lines() {
            let line = line.split_whitespace().nth(0).unwrap();

            let code_len = crate::engine::compress(line).len();

            let smaz_len = compress(line.as_bytes()).len();

            if code_len > smaz_len {
                count += 1;
            }

            total += 1;
            if total >= max {
                println!("Scanned the first {} words out of .3m.txt, with {} times smaz was better ({}%)", total, count, count as f32 / total as f32 * 100f32);
                break;
            }
        }

    }

    #[test]
    #[ignore]
    fn generate_list() {
        let str = std::fs::read_to_string(".\\.3m.txt").unwrap();

        let mut count = 0;

        for line in str.lines() {

            let line = line.split_whitespace().nth(0).unwrap();

            let code_len = crate::engine::compress(line).len();

            if code_len >= 3 {
                print!("{:?}, ", line);
                count += 1;
                if count == 16512 {
                    break;
                }
            }
        }
    }


}
