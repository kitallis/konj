include!(concat!(env!("OUT_DIR"), "/data.rs"));
pub mod strings;

use std::collections::{BTreeMap, HashMap};
use std::io::stdin;
use std::string::String;

static HIRAGANA_BEG: char = '\u{3040}';
static HIRAGANA_END: char = '\u{309F}';
static KATAKANA_BEG: char = '\u{30A0}';
static KATAKANA_END: char = '\u{30FF}';
static FULL_WIDTH_ROMAN_BEG: char = '\u{0021}';
static FULL_WIDTH_ROMAN_END: char = '\u{007E}';

pub fn romaji_to_kana(romaji: &str) {
    let geminates = strings::repeatedly_replace_str_with_map(&romaji, &GEMINATES_TO_HIRAGANA);

    let hiragana_output = transform_input(&geminates, &ROMAJI_TO_HIRAGANA);

    let katakana_output =
        strings::repeatedly_replace_str_with_map(&hiragana_output, &HIRAGANA_TO_KATAKANA);

    println!(
        "romaji: {}\nhiragana: {}\nkatakana: {}",
        romaji, hiragana_output, katakana_output
    );
}

pub fn katakana_to_romaji(kana: &str) {
    let hiragana_output = strings::repeatedly_replace_str_with_map(&kana, &KATAKANA_TO_HIRAGANA);

    let romaji_output = strings::repeatedly_replace_str_with_map(
        &transform_input(&hiragana_output, &HIRAGANA_TO_ROMAJI),
        &HIRAGANA_TO_GEMINATES,
    );

    println!(
        "hiragana: {}\nkatakana: {}\nromaji: {}",
        hiragana_output, kana, romaji_output
    );
}

pub fn hiragana_to_romaji(kana: &str) {
    let katakana_output = strings::repeatedly_replace_str_with_map(&kana, &HIRAGANA_TO_KATAKANA);

    let romaji_output = strings::repeatedly_replace_str_with_map(
        &transform_input(kana, &HIRAGANA_TO_ROMAJI),
        &HIRAGANA_TO_GEMINATES,
    );

    println!(
        "hiragana: {}\nkatakana: {}\nromaji: {}",
        kana, katakana_output, romaji_output
    );
}

pub fn transform_input(input: &str, map: &phf::Map<&str, &str>) -> String {
    // BTreeMap to allow for a sorted lookup by key length
    let mut group_by_key_size: BTreeMap<usize, HashMap<&str, &str>> = BTreeMap::new();
    let mut result = String::from(input);

    // Build the BTreeMap keyed by romaji key length
    for (key, value) in map.into_iter() {
        group_by_key_size
            .entry(key.chars().count())
            .or_insert_with(HashMap::new)
            .insert(key, value);
    }

    // Reverse the BTreeMap iterator to effectively start from the largest romaji key
    for (_k, v) in group_by_key_size.iter().rev() {
        for (key, value) in v {
            result = result.replace(key, value);
        }
    }

    result
}

pub fn is_fully_katakana(s: &str) -> bool {
    strings::is_str_between_char_range(s, KATAKANA_BEG, KATAKANA_END)
}

pub fn is_fully_hiragana(s: &str) -> bool {
    strings::is_str_between_char_range(s, HIRAGANA_BEG, HIRAGANA_END)
}

pub fn is_fully_romaji(s: &str) -> bool {
    strings::is_str_between_char_range(s, FULL_WIDTH_ROMAN_BEG, FULL_WIDTH_ROMAN_END)
}

// test data:
//
// きっう
// はは
// しんかんせん
// どきっ
// shinkansen
// kippu
