extern crate ansi_term;
#[macro_use]
extern crate lazy_static;
extern crate indexmap;

pub mod convert;
pub mod strings;

use ansi_term::Colour::Green;
use indexmap::IndexMap;
use std::collections::BTreeMap;
use std::io::stdin;
use std::string::String;

use convert::{
    ConversionData, GEMINATES_TO_KANA, HIRAGANA_TO_KATAKANA, KANA_TO_GEMINATES, KANA_TO_ROMAJI,
    KATAKANA_TO_HIRAGANA, ROMAJI_TO_KANA,
};

static HIRAGANA_BEG: char = '\u{3040}';
static HIRAGANA_END: char = '\u{309F}';
static KATAKANA_BEG: char = '\u{30A0}';
static KATAKANA_END: char = '\u{30FF}';
static FULL_WIDTH_ROMAN_BEG: char = '\u{0021}';
static FULL_WIDTH_ROMAN_END: char = '\u{007E}';

// katakana: hiragana + romaji
// romaji: katakana + hiragana
// hiragana: katakana + romaji

fn main() {
    println!(
        "{}",
        Green.paint("🍱 Konj: convert from one japanese script to all 🍱\n")
    );

    // Eager-load all the conversion data
    let conversion_data = ConversionData {
        romaji_to_hiragana: &ROMAJI_TO_KANA,
        hiragana_to_romaji: &KANA_TO_ROMAJI,
        geminates_to_kana: &GEMINATES_TO_KANA,
        kana_to_geminates: &KANA_TO_GEMINATES,
        hiragana_to_katakana: &HIRAGANA_TO_KATAKANA,
        katakana_to_hiragana: &KATAKANA_TO_HIRAGANA,
    };

    // Accept user input from STDIN
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Could not read input");
    input = input.trim().to_lowercase();

    // Print output
    match (
        is_fully_katakana(&input),
        is_fully_hiragana(&input),
        is_fully_romaji(&input),
    ) {
        (false, true, false) => hiragana_to_romaji(&input, conversion_data),
        (true, false, false) => katakana_to_romaji(&input, conversion_data),
        (false, false, true) => romaji_to_kana(&input, conversion_data),
        (_, _, _) => println!("Did not understand input character set."),
    }
}

fn romaji_to_kana(romaji: &str, conversion_data: ConversionData) {
    let geminates =
        strings::repeatedly_replace_str_with_map_2(&romaji, &conversion_data.geminates_to_kana);

    for (k, v) in conversion_data.geminates_to_kana.into_iter() {
        println!("{} {}", k, v)
    }

    let hiragana_output = transform_input(&geminates, &conversion_data.romaji_to_hiragana);

    let katakana_output = strings::repeatedly_replace_str_with_map(
        &hiragana_output,
        &conversion_data.hiragana_to_katakana,
    );

    println!(
        "romaji: {}\nhiragana: {}\nkatakana: {}",
        romaji, hiragana_output, katakana_output
    );
}

fn katakana_to_romaji(kana: &str, conversion_data: ConversionData) {
    let hiragana_output =
        strings::repeatedly_replace_str_with_map(&kana, &conversion_data.katakana_to_hiragana);

    let romaji_output = strings::repeatedly_replace_str_with_map_2(
        &transform_input(&hiragana_output, &conversion_data.hiragana_to_romaji),
        &conversion_data.kana_to_geminates,
    );

    println!(
        "hiragana: {}\nkatakana: {}\nromaji: {}",
        hiragana_output, kana, romaji_output
    );
}

fn hiragana_to_romaji(kana: &str, conversion_data: ConversionData) {
    let katakana_output =
        strings::repeatedly_replace_str_with_map(&kana, &conversion_data.hiragana_to_katakana);

    let romaji_output = strings::repeatedly_replace_str_with_map_2(
        &transform_input(kana, &conversion_data.hiragana_to_romaji),
        &conversion_data.kana_to_geminates,
    );

    println!(
        "hiragana: {}\nkatakana: {}\nromaji: {}",
        kana, katakana_output, romaji_output
    );
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

fn is_fully_katakana(s: &str) -> bool {
    strings::is_str_between_char_range(s, KATAKANA_BEG, KATAKANA_END)
}

fn is_fully_hiragana(s: &str) -> bool {
    strings::is_str_between_char_range(s, HIRAGANA_BEG, HIRAGANA_END)
}

fn is_fully_romaji(s: &str) -> bool {
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
