use crate::constants::*;
use crate::strings::*;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexItem {
    Rom,
    Num,
    Kanji,
    Hiragana,
    Katakana,
    Space,
    Other,
}

fn parse(input: char) -> LexItem {
    match input {
        input if is_char_between_char_range(input, KATAKANA_BEG, KATAKANA_END) => LexItem::Katakana,
        input if is_char_between_char_range(input, HIRAGANA_BEG, HIRAGANA_END) => LexItem::Hiragana,
        input if is_char_between_char_range(input, KANJI_BEG, KANJI_END) => LexItem::Kanji,
        input if is_char_between_char_range(input, LATIN_NUM_BEG, LATIN_NUM_END) => LexItem::Num,
        input if is_char_between_char_range(input, ROMAN_BEG, ROMAN_END) => LexItem::Rom,
        JAPANESE_SPACE => LexItem::Space,
        SPACE => LexItem::Space,
        _ => LexItem::Other,
    }
}

fn lex(input: &str) -> Vec<String> {
    let mut result = Vec::new();

    for (_, group) in &input.chars().group_by(|c| parse(*c)) {
        result.push(group.collect());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        assert_eq!(lex("けしゴム"), vec!["けし", "ゴム"]);
    }
}
