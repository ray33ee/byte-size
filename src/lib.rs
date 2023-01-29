mod ir;
mod iterator;
mod tables;

extern crate core;


#[cfg(test)]
mod tests {

    use crate::iterator::CodeIterator;



    #[test]
    fn it_works() {
        use smaz::compress;

        let message =  "Nel mezzo del cammin di nostra vita, mi ritrovai in una selva oscura";

        let code_len = CodeIterator::new(message).fold(0usize, |sum, x| sum + x.len());

        println!("Codes: {:?}", CodeIterator::new(message).collect::<Vec<_>>());
        println!("Code length: {:?}", code_len);
        println!("Original length: {}", message.len());
        println!("COmpresion ratio: {}", 1f32 - (code_len as f32 as f32 / message.len() as f32 ));

        println!("Smaz: {}", compress(message.as_bytes()).len());
        //println!("Shoco: {}", shoco_rs::compress(message).len());

        /*
        let str = std::fs::read_to_string(".\\20k.txt").unwrap();

        let mut count = 0;

        for line in str.lines() {
            let code_len = CodeIterator::new(line).fold(0usize, |sum, x| sum + x.len());

            if code_len > 3 {
                print!("{:?}, ", line);


                count += 1;

                if count == 15872 {
                    break;
                }

            }

        }*/


    }
}
