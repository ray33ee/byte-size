#[cfg(test)]
mod tests {
    use crate::ir::CodeType;
    use crate::iterator::CodeIterator;
    use crate::tables::{THREE_BYTE_UNCOMMON, TWO_BYTE_COMMON};

    fn smaz_compare(string: &str) {
        use smaz::compress;

        let code_len = CodeIterator::new(string).fold(0usize, |sum, x| sum + x.len());
        let smaz_len = compress(string.as_bytes()).len();

        println!("String: '{}' ({:?})", string, string);
        println!("    Compression:           {:?}", CodeIterator::new(string).collect::<Vec<_>>());
        println!("    Compression size:      {} ({}% compression ratio)", code_len, 100f32 - code_len as f32 / string.as_bytes().len() as f32 * 100f32);
        println!("    Smaz Compression size: {} ({}% compression ratio)", smaz_len, 100f32 - smaz_len as f32 / string.as_bytes().len() as f32 * 100f32);

        assert!(code_len <= smaz_len);

    }

    #[test]
    fn number_test() {
        smaz_compare("6709376338672");
    }

    /*#[test]
    fn http1() {
        smaz_compare("http://google.com");
    }

    #[test]
    fn http2() {
        smaz_compare("http://programming.reddit.com");
    }

    #[test]
    fn http3() {
        smaz_compare("http://github.com/antirez/smaz/tree/master");
    }*/

    #[test]
    fn ascii_control_test() {
        smaz_compare("\x01");
    }

    #[test]
    fn unicode_test() {
        smaz_compare("✔️ ❤️ ☆");
    }

    #[test]
    fn fox() {
        smaz_compare("The quick brown fox jumped over the lazy dog");
    }

    #[test]
    fn therefore() {
        smaz_compare("therefore");
    }

    #[test]
    fn download() {
        smaz_compare("download");
    }

    #[test]
    fn smaz1() {
        smaz_compare("This is a small string");
    }

    #[test]
    fn smaz2() {
        smaz_compare("foobar");
    }

    #[test]
    fn smaz3() {
        smaz_compare("the end");
    }

    #[test]
    fn smaz4() {
        smaz_compare("not-a-g00d-Exampl333");
    }

    #[test]
    fn smaz5() {
        smaz_compare("Smaz is a simple compression library");
    }

    #[test]
    fn smaz6() {
        smaz_compare("Nothing is more difficult, and therefore more precious, than to be able to decide");
    }

    #[test]
    fn smaz7() {
        smaz_compare("this is an example of what works very well with smaz");
    }

    #[test]
    fn smaz8() {
        smaz_compare("1000 numbers 2000 will 10 20 30 compress very little");
    }

    #[test]
    fn smaz9() {
        smaz_compare("Smaz is a simple compression library suitable for compressing very short
strings. General purpose compression libraries will build the state needed
for compressing data dynamically, in order to be able to compress every kind
of data. This is a very good idea, but not for a specific problem: compressing
small strings will not work.");
    }

    #[test]
    fn smaz10() {
        smaz_compare("small string shrinker");
    }

    #[test]
    fn smaz11() {
        smaz_compare("their");
    }

    fn serialize(string: &str, bytes: & [u8]) {

        let mut v: Vec<u8> = Vec::new();

        for code in CodeIterator::new(string) {
            print!("{:?}, ", code);
            code.serialize_into(& mut v);
        }

        assert_eq!(v.as_slice(), bytes)
    }

    #[test]
    fn serialize_obw_test1() { serialize("r", ['r' as u8].as_slice()) }

    #[test]
    fn serialize_obw_test2() { serialize("the", [1].as_slice()) }




    #[test]
    fn serialize_tbc_test1() { serialize("that", [241, 0].as_slice()) }

    #[test]
    fn serialize_tbc_test2() { serialize(TWO_BYTE_COMMON[255], [241, 255].as_slice()) }

    #[test]
    fn serialize_tbc_test3() { serialize(TWO_BYTE_COMMON[256], [242, 0].as_slice()) }

    #[test]
    fn serialize_control_test1() { serialize("\x01", [255, 98].as_slice()) }

    #[test]
    fn serialize_tbu_test1() { serialize("trading", [255, 127, 3].as_slice()) }

    #[test]
    fn serialize_tbu_test2() { serialize(THREE_BYTE_UNCOMMON[256], [255, 128, 0].as_slice()) }

    #[test]
    fn serialize_num_test1() { serialize("1023", [255, 90, 255].as_slice()) }

    #[test]
    fn serialize_num_test2() { serialize("6577567", [255, 92, 103, 23, 25].as_slice()) }

    #[test]
    fn serialize_uni_test2() { serialize("❤", [240, 226, 157, 164].as_slice()) }

    fn deserialize(mut bytes: & [u8], string: &str) {
        let code = CodeType::deserialize_from(& mut bytes);
        assert_eq!(code.to_string().as_str(), string);
    }

    #[test]
    fn deserialize_obw1() { deserialize([1].as_slice(), "the") }

    #[test]
    fn deserialize_tbc1() { deserialize([241, 0].as_slice(), "that") }


    fn single_code_ser_deser(string: &str) {
        let code = CodeIterator::new(string).nth(0).unwrap();

        let mut v: Vec<u8> = Vec::new();

        code.serialize_into(& mut v);

        let code2 = CodeType::deserialize_from(&v[..]);

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

    /*#[test]
    fn test_list() {
        use smaz::compress;

        let str = std::fs::read_to_string(".\\.3m.txt").unwrap();

        let max = 10000;
        let mut count = 0;
        let mut total = 0;


        for line in str.lines() {
            let line = line.split_whitespace().nth(0).unwrap();

            let v = CodeIterator::new(line).collect::<Vec<_>>();
            let code_len = v.iter().fold(0usize, |sum, x| sum + x.len());
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

    }*/

    /*
    #[test]
    fn generate_list() {
        let str = std::fs::read_to_string(".\\.3m.txt").unwrap();

        let mut count = 0;

        for line in str.lines() {

            let line = line.split_whitespace().nth(0).unwrap();

            let code_len = CodeIterator::new(line).fold(0usize, |sum, x| sum + x.len());

            if code_len >= 3 {
                print!("{:?}, ", line);
                count += 1;
                if count == 16512 {
                    break;
                }
            }
        }
    }
    */

}
