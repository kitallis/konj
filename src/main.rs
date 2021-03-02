extern crate ansi_term;

include!(concat!(env!("OUT_DIR"), "/data.rs"));
pub mod strings;

use ansi_term::Colour::Green;
use std::collections::{BTreeMap, HashMap};
use std::io::stdin;
use std::string::String;

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
        (false, true, false) => hiragana_to_romaji(&input),
        (true, false, false) => katakana_to_romaji(&input),
        (false, false, true) => romaji_to_kana(&input),
        (_, _, _) => println!("Did not understand input character set."),
    }
}

fn romaji_to_kana(romaji: &str) {
    let geminates = strings::repeatedly_replace_str_with_map(&romaji, &GEMINATES_TO_KANA);

    for (k, v) in GEMINATES_TO_KANA.into_iter() {
        println!("{} {}", k, v)
    }

    let hiragana_output = transform_input(&geminates, &ROMAJI_TO_KANA);

    let katakana_output =
        strings::repeatedly_replace_str_with_map(&hiragana_output, &HIRAGANA_TO_KATAKANA);

    println!(
        "romaji: {}\nhiragana: {}\nkatakana: {}",
        romaji, hiragana_output, katakana_output
    );
}

fn katakana_to_romaji(kana: &str) {
    let hiragana_output = strings::repeatedly_replace_str_with_map(&kana, &KATAKANA_TO_HIRAGANA);

    let romaji_output = strings::repeatedly_replace_str_with_map(
        &transform_input(&hiragana_output, &KANA_TO_ROMAJI),
        &KANA_TO_GEMINATES,
    );

    println!(
        "hiragana: {}\nkatakana: {}\nromaji: {}",
        hiragana_output, kana, romaji_output
    );
}

fn hiragana_to_romaji(kana: &str) {
    let katakana_output = strings::repeatedly_replace_str_with_map(&kana, &HIRAGANA_TO_KATAKANA);

    let romaji_output = strings::repeatedly_replace_str_with_map(
        &transform_input(kana, &KANA_TO_ROMAJI),
        &KANA_TO_GEMINATES,
    );

    println!(
        "hiragana: {}\nkatakana: {}\nromaji: {}",
        kana, katakana_output, romaji_output
    );
}

fn transform_input(input: &str, map: &phf::Map<&str, &str>) -> String {
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
