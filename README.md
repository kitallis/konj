# konj

![crates.io](https://img.shields.io/crates/v/konj.svg)

_n. portmanteau of kana, conv(ert) and japanese_

Convert between various Japanese scripts (`hiragana`, `katakana`, `romaji`)

# current status

* `romaji` → `hiragana`, `katakana`
* `hiragana` → `katakana`, `romaji` (incomplete support)

# next steps

* ~~handle capital letters in romaji input~~
* ~~handle whitespace in romaji input~~
* ~~handle punctuation: , and .~~
* output json (flag)
* make the program unix-pipe-able only
* code cleanup:
  * ~~refactor generic transformation fns~~
  * ~~move out the static maps into another namepsace~~
  * ~~eager load the maps once during the program lifetime~~

# upcoming

- Katakana → romaji, hiragana
- Handle mixed-input (hiragana + romaji etc.)
- Conversion from Kana to Kanji 
    * this would most likely be something like [henkan](https://en.wikipedia.org/wiki/Language_input_keys#Conversion) in IMEs

# use

```
± cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s

#
# romaji to kana
#
    
± echo "kippu" | ./target/debug/konj
🍱  Konj: convert from one japanese script to all 🍱

You entered in romaji. Converting to kana...
hiragana: きっぷ
katakana: キップ

#
# hiragana to romaji
#

± echo "しんかんせん" | ./target/debug/konj
🍱  Konj: convert from one japanese script to all 🍱

hiragana: しんかんせん
katakana: シンカンセン
romaji: shinkansen
```
