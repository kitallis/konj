extern crate ansi_term;
#[macro_use]
extern crate lazy_static;
extern crate indexmap;

pub mod convert;
pub mod strings;

use std::io::stdin;
use std::string::String;
use std::collections::BTreeMap;
use indexmap::IndexMap;
use ansi_term::Colour::Green;

use convert::{ConversionData,
              HIRAGANA_TO_KATAKANA,
              ROMAJI_TO_KANA,
              KANA_TO_ROMAJI,
              GEMINATES_TO_KANA};

// hiragana + katakana char list
static KANA_BEG: char = '\u{3040}';
static KANA_END: char = '\u{30FF}';

// full-width roman char list
static FULL_WIDTH_ROMAN_BEG: char = '\u{0021}';
static FULL_WIDTH_ROMAN_END: char = '\u{007E}';

fn main() {
    println!("{}", Green.paint("🍱  Konj: convert from one japanese script to all 🍱\n"));

    // Eager-load all the conversion data
    let conversion_data = ConversionData {
        romaji_to_hiragana: &ROMAJI_TO_KANA,
        hiragana_to_romaji: &KANA_TO_ROMAJI,
        geminates_to_kana: &GEMINATES_TO_KANA,
        hiragana_to_katakana: &HIRAGANA_TO_KATAKANA,
    };

    // Accept user input from STDIN
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Could not read input");
    input = input.trim().to_lowercase();

    // Print output
    match (is_fully_kana(&input), is_fully_romaji(&input)) {
        (true, false) =>
            kana_to_romaji(&input, conversion_data),

        (false, true) =>
            romaji_to_kana(&input, conversion_data),

        (_, _) =>
            println!("Did not understand input character set.")
    }
}

fn romaji_to_kana(romaji: &str, conversion_data: ConversionData) {
    let geminates =
        strings::repeatedly_replace_str_with_map(&romaji,
                                                 &conversion_data.geminates_to_kana);

    let hiragana_output =
        transform_input(&geminates,
                        &conversion_data.romaji_to_hiragana);

    let katakana_output =
        strings::repeatedly_replace_str_with_map(&hiragana_output,
                                                 &conversion_data.hiragana_to_katakana);

    println!("romaji: {}\nhiragana: {}\nkatakana: {}", romaji, hiragana_output, katakana_output);
}

fn kana_to_romaji(kana: &str, conversion_data: ConversionData) {
    let katakana_output =
        strings::repeatedly_replace_str_with_map(&kana,
                                                 &conversion_data.hiragana_to_katakana);
    let romaji_output =
        transform_input(kana, &conversion_data.hiragana_to_romaji);

    println!("hiragana: {}\nkatakana: {}\nromaji: {}", kana, katakana_output, romaji_output);
}

fn transform_input(input: &str, map: &IndexMap<&str, &str>) -> String {
    // BTreeMap to allow for a sorted lookup by key length
    let mut group_by_key_size: BTreeMap<usize, IndexMap<&str, &str>> = BTreeMap::new();
    let mut result = String::from(input);

    // Build the BTreeMap keyed by romaji key length
    for (key, value) in map.iter() {
        group_by_key_size
            .entry(key.chars().count())
            .or_insert_with(IndexMap::new)
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

fn is_fully_kana(s: &str) -> bool {
    strings::is_str_between_char_range(s,
                                       KANA_BEG,
                                       KANA_END)
}


fn is_fully_romaji(s: &str) -> bool {
    strings::is_str_between_char_range(s,
                                       FULL_WIDTH_ROMAN_BEG,
                                       FULL_WIDTH_ROMAN_END)
}

// test data:
//
// きっう
// はは
// しんかんせん
// どきっ
// shinkansen
// kippu
