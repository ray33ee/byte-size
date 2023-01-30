#[cfg(test)]
mod tests {

    use crate::iterator::CodeIterator;

    fn smaz_compare(string: &str) {
        use smaz::compress;

        let code_len = CodeIterator::new(string).fold(0usize, |sum, x| sum + x.len());
        let smaz_len = compress(string.as_bytes()).len();

        println!("String: '{}' ({:?})", string, string);
        println!("    Compression:           {:?}", CodeIterator::new(string).collect::<Vec<_>>());
        println!("    Compression size:      {} ({}% of original size)", code_len, code_len as f32 / string.as_bytes().len() as f32 * 100f32);
        println!("    Smaz Compression size: {} ({}% of original size)", smaz_len, smaz_len as f32 / string.as_bytes().len() as f32 * 100f32);

        assert!(code_len <= smaz_len);

    }

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

    }

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
