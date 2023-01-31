# Snaz

- the one byte wonder sequences are taken from [here](https://github.com/antirez/smaz/blob/master/smaz.c)
  - We filtered out certain sequences that don't work well 
- the two byte common and three byte uncommon words are chosen from [here](https://github.com/first20hours/google-10000-english/blob/master/20k.txt) 
  - Words are only chosen from the list if their representation is more compact (for example the word 'the' does not appear in either 2 byte or 3 byte tables as it can be represented in 1 byte)

The Snaz code is as follows:

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
