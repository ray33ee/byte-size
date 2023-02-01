# Byte Size

[![Crates.io](https://img.shields.io/crates/v/byte-size.svg)](https://crates.io/crates/byte-size)
[![Docs.rs](https://docs.rs/byte-size/badge.svg)](https://docs.rs/byte-size)
[![License](https://img.shields.io/crates/l/byte-size.svg)](https://opensource.org/licenses/MIT)

A short string compressor/decompressor that can store 20,000+ words in three bytes or less.

Similar to [smaz](https://github.com/antirez/smaz), byte-size is able to compress small strings, something that other conventional compression algorithms struggle with.

However, byte-size is typically better than smaz, certainly for very commonly used words (out of 10000 most common words, ony 1% had better compression with smaz)
byte-size can also represent numbers, repeated sequences and non-alphanumeric characters more efficiently than smaz. It can encode unicode characters, but not very efficiently. If your text includes a few unicode characters it should still compress better, but if your strings are mostly unicode characters, other schemes such as [Unishox](https://github.com/siara-cc/Unishox2) are better.

## Cost

byte-size uses several tables with over 18000 total entries. Obviously this will incur a large runtime memory and binary file size cost, but if you have the memory available, it is worth it to compress more effectively.

To match these, currently we use a poor algorithm that lops over EVERY entry in EVERY table to obtain the best map. Future versions will use a phf hash table approach.

## Examples

Using examples directly from smaz we have:

[Insert examples]

We can see how every example is compressed more with byte-size than smaz.

## Encoding

- the one byte wonder sequences are taken from [smaz](https://github.com/antirez/smaz/blob/master/smaz.c)
  - We filtered out certain sequences that don't work well
- the two byte common and three byte uncommon words are chosen from [here](https://norvig.com/ngrams/)
  - Words are only chosen from the list if their representation is more compact (for example the word 'the' does not appear in either 2 byte or 3 byte tables as it can be represented in 1 byte)

The Snaz encoding is as follows:

- The one byte wonders consist of 240 one byte sequences. These one byte wonders are the ascii values in their normal positions, with other common sequences filling the gaps to 
  - This leaves 16 values remaining for multi-byte codes.
    - One of these values is used to indicate a Unicode scalar value will follow
    - This leaves 15*256=3840 combinations of two byte sequences, which are divided as such:
      - 3586 are used to encode the 2 byte common words, 1793 with a space prefix and 1793 without
      - 32 are used to encode custom words
      - 32 are used to encode sequence repetitions of anywhere between 4 and 35 repeating units
      - 32 are used to encode numbers. 32 values means 5 bits in total, 3 for the number of bytes used, and 2 for the number itself
      - 29 are used to encode the non-printable control characters
      - 129 are used for the 3 byte codes. 129*256 = 33024 combinations
        - All 33024 combinations are used for the 3 byte uncommon words, 16512 with a space prefix, and 16512 without
