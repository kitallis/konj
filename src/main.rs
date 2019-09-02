extern crate ansi_term;
#[macro_use]
extern crate lazy_static;

pub mod maps;

use std::io::stdin;
use std::string::String;
use std::collections::{HashMap, BTreeMap};
use ansi_term::Colour::Green;

use maps::{HIRAGANA_TO_KATAKANA, ROMAJI_TO_KANA, DOUBLE_CONSONANTS_TO_KANA};

//
// hiragana + katakana char list
//
static KANA_BEG: char = '\u{3040}';
static KANA_END: char = '\u{30FF}';

//
// full-width roman char list
//
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

fn to_kana(s: &str, katakana: bool) {
    let hiragana_output = replace_by_size(&to_double_consonants(&s));
    let mut katakana_output = String::new();

    if katakana {
        katakana_output = to_katakana(&hiragana_output);
    }

    println!("hiragana: {}\nkatakana: {}", hiragana_output, katakana_output);
}

fn is_fully_kana(s: &str) -> bool {
    for ch in s.trim().chars() {
        if !(ch >= KANA_BEG && ch <= KANA_END) { return false; }
    }

    true
}

fn is_fully_romaji(s: &str) -> bool {
    for ch in s.trim().chars() {
        if !(ch >= FULL_WIDTH_ROMAN_BEG && ch <= FULL_WIDTH_ROMAN_END) { return false; }
    }

    true
}

fn to_double_consonants(s: &str) -> String {
    let mut result = String::from(s);

    for (k, v) in DOUBLE_CONSONANTS_TO_KANA.iter() {
        result = result.replace(k, v);
    }

    result
}

fn to_katakana(s: &str) -> String {
    let mut result = String::from(s);

    for (k, v) in HIRAGANA_TO_KATAKANA.iter() {
        result = result.replace(k, v);
    }

    result
}

fn replace_by_size(s: &str) -> String {
    let mut result = String::from(s);

    // reversing the BTreeMap iterator to effectively start from the largest romaji key
    for (_k, v) in romaji_to_kana().iter().rev() {
        for (romaji, kana) in v {
            result = result.replace(romaji, kana);
        }
    }

    result
}

fn romaji_to_kana() -> BTreeMap<usize, HashMap<&'static str, &'static str>> {
    // BTreeMap to allow for a sorted lookup by key length
    let mut map: BTreeMap<usize, HashMap<&str, &str>> = BTreeMap::new();

    for (k, v) in ROMAJI_TO_KANA.iter() {
        group_by_length(&mut map, k, v);
    }

    map
}

fn group_by_length<'a>(map: &mut BTreeMap<usize, HashMap<&'a str, &'a str>>, romaji: &'a str, kana: &'a str) {
    map.entry(romaji.chars().count()).or_insert_with(HashMap::new).insert(romaji, kana);
}

