# konj
Convert between various Japanese scripts (`hiragana`, `katakana`, `romaji`)

# current status

Basic conversion from `romaji` to `hiragana`, `katakana`

# upcoming

- Conversion from any input to all other inputs
    * hiragana → romaji, katakana
    * katakana → romaji, hiragana
- Conversion from Kana to Kanji 
    * this would most likely be something like [henkan](https://en.wikipedia.org/wiki/Language_input_keys#Conversion) in IMEs

# use

```
± cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/konj`
== Konj: convert from one japanese script to all ==
kippu
You entered in romaji. Converting to kana...
きっぷ
```
