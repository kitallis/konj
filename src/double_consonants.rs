use std::collections::HashMap;

//
// In the form of: romaji → partial kana
//
lazy_static! {
    pub static ref DOUBLE_CONSONANTS_TO_KANA: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();

        map.insert("kk", "っk");
        map.insert("tt", "っt");
        map.insert("cc", "っc");
        map.insert("ss", "っs");
        map.insert("pp", "っp");
        map.insert("mm", "んm");
        map.insert("mt", "んt");
        map.insert("mb", "んb");
        map.insert("mp", "んp");
        map.insert("nt", "んt");
        map.insert("nb", "んb");
        map.insert("np", "んp");

        map
    };
}
