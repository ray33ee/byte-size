#[cfg(test)]
mod tests {
    use crate::builder::Builder;
    use crate::engine::Engine;
    use crate::iterator::CodeIterator;

    fn full_ser_deser_builder(string: &str, engine: & Engine, compressed_size: usize) {
        use smaz::compress;




        let bytes = engine.compress(string);

        let x = engine.decompress(bytes.as_slice()).unwrap();

        let smaz_len = compress(string.as_bytes()).len();
        let code_len = bytes.len();

        println!("String: '{}' ({:?})", string, string);
        println!("    Original Size:         {}", string.len());
        println!("    Compression:           {:?}", bytes);
        println!("    Code Points:           {:?}", CodeIterator::new(string, &engine).collect::<Vec<_>>());
        println!("    Compression size:      {} ({}% compression ratio)", code_len, 100f32 - code_len as f32 / string.as_bytes().len() as f32 * 100f32);
        println!("    Smaz Compression size: {} ({}% compression ratio)", smaz_len, 100f32 - smaz_len as f32 / string.as_bytes().len() as f32 * 100f32);

        //First make sure that the decompression worked correctly
        assert_eq!(string, x.as_str());

        //Next we make sure the size hasn't changed. This means if we change the algorithm, we can catch any changes
        assert_eq!(code_len, compressed_size);

        //Finally we see if the compressed size is smaller than smaz
        assert!(code_len <= smaz_len);

    }

    #[test]
    fn time() {
        for _ in 0..1000 {
            Builder::empty().engine().decompress(Builder::empty().engine().compress("The quick brown fox jumped over the lazy dog").as_slice()).unwrap();
        }
    }

    fn full_ser_deser(string: & str, compressed_size: usize) {
        full_ser_deser_builder(string, &Builder::default().engine(), compressed_size)
    }

    #[test]
    fn number_test() {
        full_ser_deser("6709376338672", 8);
    }

    #[test]
    fn number_test1() {
        full_ser_deser("6709323423423763138672", 13);
    }

    #[test]
    fn number_test2() {
        full_ser_deser("999", 3);
    }

    #[test]
    fn number_test3() {
        full_ser_deser("1000", 3);
    }

    #[test]
    fn number_test4() {
        full_ser_deser("1000 ", 4);
    }

    #[test]
    fn http1() {
        full_ser_deser("http://google.com", 6);
    }

    #[test]
    fn http2() {
        full_ser_deser("http://programming.reddit.com", 11);
    }

    #[test]
    fn http3() {
        full_ser_deser_builder("http://github.com/antirez/smaz/tree/master", &Builder::default().push_custom("http://github.com/").engine() , 16);
    }

    #[test]
    fn patch() {
        full_ser_deser("patch", 3);
    }

    #[test]
    fn ascii_control_test() {
        full_ser_deser("\x01", 2);
    }

    #[test]
    fn repetition_test() {
        full_ser_deser("hehehehe", 3);
    }

    #[test]
    fn repetition_test1() {
        full_ser_deser("he he he he ", 3);
    }

    #[test]
    fn repetition_test2() {
        full_ser_deser("oohe he he he he he ", 7);
    }

    #[test]
    fn repetition_test3() {
        full_ser_deser("hhhhhhhhhhhhhhohhhhhhhhhhh", 8);
    }

    #[test]
    fn ascii_lf() {
        full_ser_deser("\n", 1);
    }

    #[test]
    fn ascii_crlf() {
        full_ser_deser("\r\n", 1);
    }

    #[test]
    fn message_test() {
        full_ser_deser("yeh thats fine mate 🙂", 17);
    }

    #[test]
    fn unicode_test() {
        full_ser_deser("✔️ ❤️ ☆", 22);
    }

    #[test]
    fn fox() {
        full_ser_deser("The quick brown fox jumped over the lazy dog", 20);
    }

    #[test]
    fn therefore() {
        full_ser_deser("therefore", 2);
    }

    #[test]
    fn download() {
        full_ser_deser("download", 2);
    }

    #[test]
    fn smaz1() {
        full_ser_deser("This is a small string", 9);
    }

    #[test]
    fn smaz2() {
        full_ser_deser("foobar", 4);
    }

    #[test]
    fn smaz3() {
        full_ser_deser("the end", 3);
    }

    #[test]
    fn smaz4() {
        full_ser_deser("not-a-g00d-Exampl333", 18);
    }

    #[test]
    fn smaz5() {
        full_ser_deser("Smaz is a simple compression library", 13);
    }

    #[test]
    fn smaz6() {
        full_ser_deser("Nothing is more difficult, and therefore more precious, than to be able to decide", 32);
    }

    #[test]
    fn smaz7() {
        full_ser_deser("this is an example of what works very well with smaz", 23);
    }

    #[test]
    fn smaz8() {
        full_ser_deser("1000 numbers 2000 will 10 20 30 compress very little", 28);
    }

    #[test]
    fn smaz9() {
        full_ser_deser("Smaz is a simple compression library suitable for compressing very short
strings. General purpose compression libraries will build the state needed
for compressing data dynamically, in order to be able to compress every kind
of data. This is a very good idea, but not for a specific problem: compressing
small strings will not work.", 127);
    }

    #[test]
    fn smaz10() {
        full_ser_deser("small string shrinker", 8);
    }

    #[test]
    fn test_custom_space() {
        full_ser_deser_builder(" customstringspacetest", &Builder::default().set_custom_spaces(true).push_custom("customstringspacetest").engine(), 2)
    }

    #[test]
    fn test_custom_space2() {
        full_ser_deser_builder(" customstringspacetest", &Builder::default().set_custom_spaces(false).push_custom("customstringspacetest").engine(), 9)
    }

    #[test]
    fn test_custom_space3() {
        full_ser_deser_builder("customstringspacetest", &Builder::default().set_custom_spaces(false).push_custom("customstringspacetest").engine(), 2)
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
    fn test_lengths() {
        let mut count = 0;

        let str = std::fs::read_to_string(".\\.3m.txt").unwrap();

        for line in str.lines().take(10000) {
            let line = line.split_whitespace().nth(0).unwrap();

            let code_len = crate::engine::compress(line).len();

            if code_len > 3 {
                println!("{}: {:?}", line, CodeIterator::new(line, &Builder::default().engine()).collect::<Vec<_>>());
                count += 1;
            }


        }

        assert_eq!(count, 0)
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

            let code_len = crate::engine::compress(line).len();

            let smaz_len = compress(line.as_bytes()).len();

            if code_len > smaz_len {
                count += 1;
            }

            total += 1;
            if total >= max {
                println!("Scanned the first {} words out of .3m.txt, with {} times smaz was better ({}%)", total, count, count as f32 / total as f32 * 100f32);
                assert!(count as f32 / total as f32 * 100f32 < 1f32);
                break;
            }
        }

    }


}
