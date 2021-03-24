include!(concat!(env!("OUT_DIR"), "/data.rs"));
pub mod strings;

use std::collections::{BTreeMap, HashMap};
use std::string::String;

static HIRAGANA_BEG: char = '\u{3040}';
static HIRAGANA_END: char = '\u{309F}';
static KATAKANA_BEG: char = '\u{30A0}';
static KATAKANA_END: char = '\u{30FF}';
static FULL_WIDTH_ROMAN_BEG: char = '\u{0021}';
static FULL_WIDTH_ROMAN_END: char = '\u{007E}';

fn to_hiragana(input: &str) -> String {
    match (is_katakana(&input), is_hiragana(&input), is_romaji(&input)) {
        (true, false, false) => {
            strings::repeatedly_replace_str_with_map(&input, &KATAKANA_TO_HIRAGANA)
        }

        (false, true, false) => String::from(input),

        (false, false, true) => {
            let geminates =
                strings::repeatedly_replace_str_with_map(&input, &GEMINATES_TO_HIRAGANA);
            transform_input(&geminates, &ROMAJI_TO_HIRAGANA)
        }

        (_, _, _) => format!("Did not understand input character set."),
    }
}

fn to_katakana(input: &str) -> String {
    match (is_katakana(&input), is_hiragana(&input), is_romaji(&input)) {
        (true, false, false) => String::from(input),

        (false, true, false) => {
            strings::repeatedly_replace_str_with_map(&input, &HIRAGANA_TO_KATAKANA)
        }

        (false, false, true) => {
            let geminates =
                strings::repeatedly_replace_str_with_map(&input, &GEMINATES_TO_HIRAGANA);
            let hiragana_output = transform_input(&geminates, &ROMAJI_TO_HIRAGANA);

            strings::repeatedly_replace_str_with_map(&hiragana_output, &HIRAGANA_TO_KATAKANA)
        }

        (_, _, _) => format!("Did not understand input character set."),
    }
}

fn to_romaji(input: &str) -> String {
    match (is_katakana(&input), is_hiragana(&input), is_romaji(&input)) {
        (true, false, false) => {
            let hiragana_output =
                strings::repeatedly_replace_str_with_map(&input, &KATAKANA_TO_HIRAGANA);

            strings::repeatedly_replace_str_with_map(
                &transform_input(&hiragana_output, &HIRAGANA_TO_ROMAJI),
                &HIRAGANA_TO_GEMINATES,
            )
        }

        (false, true, false) => strings::repeatedly_replace_str_with_map(
            &transform_input(&input, &HIRAGANA_TO_ROMAJI),
            &GEMINATES_TO_HIRAGANA,
        ),

        (false, false, true) => String::from(input),

        (_, _, _) => format!("Did not understand input character set."),
    }
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

fn is_katakana(s: &str) -> bool {
    strings::is_str_between_char_range(s, KATAKANA_BEG, KATAKANA_END)
}

fn is_hiragana(s: &str) -> bool {
    strings::is_str_between_char_range(s, HIRAGANA_BEG, HIRAGANA_END)
}

fn is_romaji(s: &str) -> bool {
    strings::is_str_between_char_range(s, FULL_WIDTH_ROMAN_BEG, FULL_WIDTH_ROMAN_END)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_hiragana() {
        assert_eq!(to_hiragana("shinkansen"), "しんかんせん");
        assert_eq!(to_hiragana("はは"), "はは");
        assert_eq!(to_hiragana("doki"), "どき");
        assert_eq!(to_hiragana("kippu"), "きっぷ");
        assert_eq!(to_hiragana("きっう"), "きっう");
    }

    #[test]
    fn test_to_katakana() {
        assert_eq!(to_katakana("shinkansen"), "シンカンセン");
        assert_eq!(to_katakana("はは"), "ハハ");
        assert_eq!(to_katakana("doki"), "ドキ");
        assert_eq!(to_katakana("kippu"), "キップ");
        assert_eq!(to_katakana("きっう"), "キッウ");
    }

    #[test]
    fn test_to_romaji() {
        assert_eq!(to_romaji("shinkansen"), "shinkansen");
        assert_eq!(to_romaji("はは"), "haha");
        assert_eq!(to_romaji("ドキ"), "doki");
        assert_eq!(to_romaji("きっぷ"), "kiっpu");
        assert_eq!(to_romaji("きっう"), "kiっu");
    }
}
