# konj

[![Crates.io](https://img.shields.io/crates/v/konj.svg)](https://crates.io/crates/konj)

_n. portmanteau of kana, conv(ert) and japanese_

Convert between various Japanese scripts (`hiragana`, `katakana`, `romaji`)

# current status

* `romaji` → `hiragana`, `katakana`
* `hiragana` → `katakana`, `romaji`
* `katakana` → `romaji`, `hiragana`

# next steps

* ~~handle capital letters in romaji input~~
* ~~handle whitespace in romaji input~~
* ~~handle punctuation: , and .~~
* ~~code cleanup:~~
  * ~~refactor generic transformation fns~~
  * ~~move out the static maps into another namepsace~~
  * ~~eager load the maps once during the program lifetime~~
* expose a library API instead of just printing out

# upcoming

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

romaji: kippu
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
