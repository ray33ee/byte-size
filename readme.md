# Snaz

- Ascii characters can be stored as is (single byte with value < 128)
- Single byte with value > 128 and less than 245 store 118 common codes
- Extra 27 common codes are stored in the non-printable ascii values
- Dual byte where the first valuye is greater than 245 gives 10*256 = 2560 combinations.
  - 2304 of these include 1152 common lemmas in the english language (1152 prefixed with a space, 1152 without)
  - 100 are custom codes (MUST be more than 2 byte strings to be worth it)
  - 30 are used for repetitions
  - 32 are used for numbers
    - these 32 are represented using 5 bits. 3 bits are used to encode the number of bytes used, and the remaining 2 used to store the number
  - 124 are used for the 3 byte uncommon (124 * 256 gives us a meaty 15872 words with a prefix space, and 15872 without)
    - Finally we have a triple byte that is used for another 2500 less common strings 
      - NOTE: these strings are chosen such that their representation via just 1 and 2 bytes is shorter than 3 bytes

- the 128 chosen words are picked from [here](https://github.com/antirez/smaz/blob/master/smaz.c)
  - they are formed of common prefixes and postfixes, and also comtain common fragments
- the two byte common and three byte uncommon words are chosen from [here](https://github.com/first20hours/google-10000-english/blob/master/20k.txt) 
  - Words are only chosen from the list if their representation is more compact (for example the word 'the' does not appear in either 2 byte or 3 byte tables as it can be represented in 1 byte)
