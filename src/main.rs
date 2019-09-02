extern crate ansi_term;
#[macro_use]
extern crate lazy_static;

pub mod maps;

use std::io::stdin;
use std::string::String;
use std::collections::{HashMap, BTreeMap};
use ansi_term::Colour::Green;

use maps::{HIRAGANA_TO_KATAKANA, ROMAJI_TO_KANA, DOUBLE_CONSONANTS_TO_KANA};

// hiragana + katakana char list
static KANA_BEG: char = '\u{3040}';
static KANA_END: char = '\u{30FF}';

// full-width roman char list
static FULL_WIDTH_ROMAN_BEG: char = '\u{0021}';
static FULL_WIDTH_ROMAN_END: char = '\u{007E}';

fn main() {
    println!("{}", Green.paint("🍱  Konj: convert from one japanese script to all 🍱\n"));

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Could not read input");

    if is_fully_kana(&input.trim()) {
        println!("You entered in kana. Cannot do anything.");
    } else if is_fully_romaji(&input.trim()) {
        println!("You entered in romaji. Converting to kana...");
        to_kana(&input.trim(), true)
    } else {
        println!("Did not understand input character set.")
    }

    println!("{}", "🍙")
}

fn to_kana(s: &str, enable_katakana: bool) {
    let hiragana_output = romaji_to_kana(&to_double_consonants(&s));
    let mut katakana_output = String::new();

    if enable_katakana { katakana_output = to_katakana(&hiragana_output); }

    println!("hiragana: {}\nkatakana: {}", hiragana_output, katakana_output);
}

fn is_fully_kana(s: &str) -> bool {
    is_str_between_char_range(s, KANA_BEG, KANA_END)
}

fn is_fully_romaji(s: &str) -> bool {
    is_str_between_char_range(s, FULL_WIDTH_ROMAN_BEG, FULL_WIDTH_ROMAN_END)
}

fn is_str_between_char_range(s: &str, range_beg: char, range_end: char) -> bool {
    for ch in s.trim().chars() {
        if !(ch >= range_beg && ch <= range_end) { return false; }
    }

    true
}

fn to_double_consonants(s: &str) -> String {
    repeatedly_replace_str_with_map(&s, &DOUBLE_CONSONANTS_TO_KANA)
}

fn to_katakana(s: &str) -> String {
    repeatedly_replace_str_with_map(&s, &HIRAGANA_TO_KATAKANA)
}

fn repeatedly_replace_str_with_map(s: &str, map: &HashMap<&str, &str>) -> String {
    map.iter().fold(String::from(s), |mut acc, (k, v)| {
        acc = acc.replace(k, v);
        acc
    })
}

fn romaji_to_kana(s: &str) -> String {
    // BTreeMap to allow for a sorted lookup by key length
    let mut romaji_to_kana_by_key_size: BTreeMap<usize, HashMap<&str, &str>> = BTreeMap::new();
    let mut result = String::from(s);

    // Build the BTreeMap keyed by romaji key length
    for (romaji, kana) in ROMAJI_TO_KANA.iter() {
        romaji_to_kana_by_key_size
            .entry(romaji.chars().count())
            .or_insert_with(HashMap::new)
            .insert(romaji, kana);
    }

    // Reverse the BTreeMap iterator to effectively start from the largest romaji key
    for (_k, v) in romaji_to_kana_by_key_size.iter().rev() {
        for (romaji, kana) in v {
            result = result.replace(romaji, kana);
        }
    }

    result
}
