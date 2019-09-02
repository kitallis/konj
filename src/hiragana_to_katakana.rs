use std::collections::HashMap;

//
// Direct mappings from hiragana to katakana char-by-char
//
lazy_static! {
    pub static ref HIRAGANA_TO_KATAKANA: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();

        map.insert("ぁ", "ァ");
        map.insert("あ", "ア");
        map.insert("ぃ", "ィ");
        map.insert("い", "イ");
        map.insert("ぅ", "ゥ");
        map.insert("う", "ウ");
        map.insert("ぇ", "ェ");
        map.insert("え", "エ");
        map.insert("ぉ", "ォ");
        map.insert("お", "オ");
        map.insert("か", "カ");
        map.insert("が", "ガ");
        map.insert("き", "キ");
        map.insert("ぎ", "ギ");
        map.insert("く", "ク");
        map.insert("ぐ", "グ");
        map.insert("け", "ケ");
        map.insert("げ", "ゲ");
        map.insert("こ", "コ");
        map.insert("ご", "ゴ");
        map.insert("さ", "サ");
        map.insert("ざ", "ザ");
        map.insert("し", "シ");
        map.insert("じ", "ジ");
        map.insert("す", "ス");
        map.insert("ず", "ズ");
        map.insert("せ", "セ");
        map.insert("ぜ", "ゼ");
        map.insert("そ", "ソ");
        map.insert("ぞ", "ゾ");
        map.insert("た", "タ");
        map.insert("だ", "ダ");
        map.insert("ち", "チ");
        map.insert("ぢ", "ヂ");
        map.insert("っ", "ッ");
        map.insert("つ", "ツ");
        map.insert("づ", "ヅ");
        map.insert("て", "テ");
        map.insert("で", "デ");
        map.insert("と", "ト");
        map.insert("ど", "ド");
        map.insert("な", "ナ");
        map.insert("に", "ニ");
        map.insert("ぬ", "ヌ");
        map.insert("ね", "ネ");
        map.insert("の", "ノ");
        map.insert("は", "ハ");
        map.insert("ば", "バ");
        map.insert("ぱ", "パ");
        map.insert("ひ", "ヒ");
        map.insert("び", "ビ");
        map.insert("ぴ", "ピ");
        map.insert("ふ", "フ");
        map.insert("ぶ", "ブ");
        map.insert("ぷ", "プ");
        map.insert("へ", "ヘ");
        map.insert("べ", "ベ");
        map.insert("ぺ", "ペ");
        map.insert("ほ", "ホ");
        map.insert("ぼ", "ボ");
        map.insert("ぽ", "ポ");
        map.insert("ま", "マ");
        map.insert("み", "ミ");
        map.insert("む", "ム");
        map.insert("め", "メ");
        map.insert("も", "モ");
        map.insert("ゃ", "ャ");
        map.insert("や", "ヤ");
        map.insert("ゅ", "ュ");
        map.insert("ゆ", "ユ");
        map.insert("ょ", "ョ");
        map.insert("よ", "ヨ");
        map.insert("ら", "ラ");
        map.insert("り", "リ");
        map.insert("る", "ル");
        map.insert("れ", "レ");
        map.insert("ろ", "ロ");
        map.insert("ゎ", "ヮ");
        map.insert("わ", "ワ");
        map.insert("ゐ", "ヰ");
        map.insert("ゑ", "ヱ");
        map.insert("を", "ヲ");
        map.insert("ん", "ン");
        map.insert("ゔ", "ヴ");
        map.insert("ゕ", "ヵ");
        map.insert("ゖ", "ヶ");
        map.insert("わ゛", "ヷ");
        map.insert("ゐ゛", "ヸ");
        map.insert("ゑ゛", "ヹ");
        map.insert("を゛", "ヺ");

        map
    };
}

