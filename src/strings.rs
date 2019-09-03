use std::collections::HashMap;

pub fn is_str_between_char_range(s: &str, range_beg: char, range_end: char) -> bool {
    for ch in s.trim().chars() {
        if !(ch >= range_beg && ch <= range_end || ch.is_whitespace()) { return false; }
    }

    true
}

pub fn repeatedly_replace_str_with_map(s: &str, map: &HashMap<&str, &str>) -> String {
    map.iter().fold(String::from(s), |mut acc, (k, v)| {
        acc = acc.replace(k, v);
        acc
    })
}
