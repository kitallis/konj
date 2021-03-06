use phf::Map;
use std::string::String;

pub fn is_str_between_char_range(s: &str, range_beg: char, range_end: char) -> bool {
    for ch in s.trim().chars() {
        if !is_char_between_char_range(ch, range_beg, range_end) {
            return false;
        }
    }

    true
}

pub fn is_char_between_char_range(ch: char, range_beg: char, range_end: char) -> bool {
    if !(ch >= range_beg && ch <= range_end || ch.is_whitespace()) {
        return false;
    }

    true
}

pub fn repeatedly_replace_str_with_map(s: &str, map: &Map<&str, &str>) -> String {
    map.into_iter().fold(String::from(s), |mut acc, (k, v)| {
        acc = acc.replace(k, v);
        acc
    })
}
