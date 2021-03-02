use indexmap::IndexMap;

#[derive(Debug)]
pub struct ConversionData<'a> {
    pub romaji_to_hiragana: &'a IndexMap<&'static str, &'static str>,
    pub hiragana_to_romaji: &'a IndexMap<&'static str, &'static str>,
    pub geminates_to_kana: &'a IndexMap<&'static str, &'static str>,
    pub kana_to_geminates: &'a IndexMap<&'static str, &'static str>,
    pub hiragana_to_katakana: &'a IndexMap<&'static str, &'static str>,
    pub katakana_to_hiragana: &'a IndexMap<&'static str, &'static str>,
}

//
// In the form of: romaji → partial kana
//
lazy_static! {
    pub static ref GEMINATES_TO_KANA: IndexMap<&'static str, &'static str> = {
        let mut map = IndexMap::new();

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

lazy_static! {
    pub static ref KANA_TO_GEMINATES: IndexMap<&'static str, &'static str> = {
        let mut map: IndexMap<&str, &str> = IndexMap::new();

        for (k, v) in GEMINATES_TO_KANA.iter() {
            map.insert(v, k);
        }

        map
    };
}

//
// Direct mappings from hiragana to katakana char-by-char
//
lazy_static! {
    pub static ref HIRAGANA_TO_KATAKANA: IndexMap<&'static str, &'static str> = {
        let mut map = IndexMap::new();

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

lazy_static! {
    pub static ref KATAKANA_TO_HIRAGANA: IndexMap<&'static str, &'static str> = {
        let mut map: IndexMap<&str, &str> = IndexMap::new();

        for (k, v) in HIRAGANA_TO_KATAKANA.iter() {
            map.insert(v, k);
        }

        map
    };
}

//
// This does three things:
//
// 1. Inverts the insertion order of ROMAJI_TO_KANA
// 2. Inverts from Map<K,V> to Map<V,K>
// 3. TODO: Removes all instances of っ from the inverted mapping
//
// No. 2 ensures that the "dominant" mapping is preferred
// For eg, "ka" over "ca" for "か"
//
lazy_static! {
    pub static ref KANA_TO_ROMAJI: IndexMap<&'static str, &'static str> = {
        let mut map: IndexMap<&str, &str> = IndexMap::new();

        for (k, v) in GEMINATES_TO_KANA.iter() {
            map.insert(v, k);
        }

        // Iterate Map<K,V> in reverse, and invert Map<K,V> to Map<V,K>
        for (k, v) in ROMAJI_TO_KANA_SANS_SOKUON.iter().rev() {
            map.insert(v, k);
        }

        map
    };
}

//
// Hepburn / Kunrei-shiki romanization mappings to kana
//
lazy_static! {
    pub static ref ROMAJI_TO_KANA: IndexMap<&'static str, &'static str> = {
        let mut map = IndexMap::new();

        // The insertion order here matters for kana → romaji,
        map.insert("n", "ん");
        map.insert("nnn", "んn");
        map.insert("nn", "ん");
        map.insert("xn", "ん");
        map.insert("n", "ん");
        map.insert("a", "あ");
        map.insert("i", "い");
        map.insert("u", "う");
        map.insert("e", "え");
        map.insert("o", "お");
        map.insert("yi", "い");
        map.insert("wu", "う");
        map.insert("whu", "う");
        map.insert("la", "ぁ");
        map.insert("li", "ぃ");
        map.insert("lu", "ぅ");
        map.insert("le", "ぇ");
        map.insert("lo", "ぉ");
        map.insert("xa", "ぁ");
        map.insert("xi", "ぃ");
        map.insert("xu", "ぅ");
        map.insert("xe", "ぇ");
        map.insert("xo", "ぉ");
        map.insert("lyi", "ぃ");
        map.insert("xyi", "ぃ");
        map.insert("lye", "ぇ");
        map.insert("xye", "ぇ");
        map.insert("ye", "いぇ");
        map.insert("wi", "うぃ");
        map.insert("we", "うぇ");
        map.insert("wha", "うぁ");
        map.insert("whi", "うぃ");
        map.insert("whe", "うぇ");
        map.insert("who", "うぉ");
        map.insert("vu", "ヴ");
        map.insert("va", "ヴぁ");
        map.insert("vi", "ヴぃ");
        map.insert("vyi", "ヴぃ");
        map.insert("ve", "ヴぇ");
        map.insert("vye", "ヴぇ");
        map.insert("vo", "ヴぉ");
        map.insert("vya", "ヴゃ");
        map.insert("vyu", "ヴゅ");
        map.insert("vyo", "ヴょ");
        map.insert("ka", "か");
        map.insert("ki", "き");
        map.insert("ku", "く");
        map.insert("ke", "け");
        map.insert("ko", "こ");
        map.insert("ca", "か");
        map.insert("cu", "く");
        map.insert("co", "こ");
        map.insert("qu", "く");
        map.insert("kya", "きゃ");
        map.insert("kyi", "きぃ");
        map.insert("kyu", "きゅ");
        map.insert("kye", "きぇ");
        map.insert("kyo", "きょ");
        map.insert("qya", "くゃ");
        map.insert("qyu", "くゅ");
        map.insert("qyo", "くょ");
        map.insert("lka", "ヵ");
        map.insert("xka", "ヵ");
        map.insert("lke", "ヶ");
        map.insert("xke", "ヶ");
        map.insert("qwa", "くぁ");
        map.insert("qwi", "くぃ");
        map.insert("qwu", "くぅ");
        map.insert("qwe", "くぇ");
        map.insert("qwo", "くぉ");
        map.insert("qa", "くぁ");
        map.insert("qi", "くぃ");
        map.insert("qe", "くぇ");
        map.insert("qo", "くぉ");
        map.insert("kwa", "くぁ");
        map.insert("qyi", "くぃ");
        map.insert("qye", "くぇ");
        map.insert("ga", "が");
        map.insert("gi", "ぎ");
        map.insert("gu", "ぐ");
        map.insert("ge", "げ");
        map.insert("go", "ご");
        map.insert("gya", "ぎゃ");
        map.insert("gyi", "ぎぃ");
        map.insert("gyu", "ぎゅ");
        map.insert("gye", "ぎぇ");
        map.insert("gyo", "ぎょ");
        map.insert("gwa", "ぐぁ");
        map.insert("gwi", "ぐぃ");
        map.insert("gwu", "ぐぅ");
        map.insert("gwe", "ぐぇ");
        map.insert("gwo", "ぐぉ");
        map.insert("shi", "し");
        map.insert("sa", "さ");
        map.insert("si", "し");
        map.insert("su", "す");
        map.insert("se", "せ");
        map.insert("so", "そ");
        map.insert("ci", "し");
        map.insert("ce", "せ");
        map.insert("sha", "しゃ");
        map.insert("shu", "しゅ");
        map.insert("she", "しぇ");
        map.insert("sho", "しょ");
        map.insert("sya", "しゃ");
        map.insert("syi", "しぃ");
        map.insert("syu", "しゅ");
        map.insert("sye", "しぇ");
        map.insert("syo", "しょ");
        map.insert("swa", "すぁ");
        map.insert("swi", "すぃ");
        map.insert("swu", "すぅ");
        map.insert("swe", "すぇ");
        map.insert("swo", "すぉ");
        map.insert("ji", "じ");
        map.insert("za", "ざ");
        map.insert("zi", "じ");
        map.insert("zu", "ず");
        map.insert("ze", "ぜ");
        map.insert("zo", "ぞ");
        map.insert("ja", "じゃ");
        map.insert("ju", "じゅ");
        map.insert("je", "じぇ");
        map.insert("jo", "じょ");
        map.insert("jya", "じゃ");
        map.insert("jyi", "じぃ");
        map.insert("jyu", "じゅ");
        map.insert("jye", "じぇ");
        map.insert("jyo", "じょ");
        map.insert("zya", "じゃ");
        map.insert("zyi", "じぃ");
        map.insert("zyu", "じゅ");
        map.insert("zye", "じぇ");
        map.insert("zyo", "じょ");
        map.insert("chi", "ち");
        map.insert("tsu", "つ");
        map.insert("ta", "た");
        map.insert("ti", "ち");
        map.insert("tu", "つ");
        map.insert("te", "て");
        map.insert("to", "と");
        map.insert("cha", "ちゃ");
        map.insert("chu", "ちゅ");
        map.insert("che", "ちぇ");
        map.insert("cho", "ちょ");
        map.insert("tya", "ちゃ");
        map.insert("tyi", "ちぃ");
        map.insert("tyu", "ちゅ");
        map.insert("tye", "ちぇ");
        map.insert("tyo", "ちょ");
        map.insert("cya", "ちゃ");
        map.insert("cyi", "ちぃ");
        map.insert("cyu", "ちゅ");
        map.insert("cye", "ちぇ");
        map.insert("cyo", "ちょ");
        map.insert("ltu", "っ");
        map.insert("xtu", "っ");
        map.insert("ltsu", "っ");
        map.insert("tsa", "つぁ");
        map.insert("tsi", "つぃ");
        map.insert("tse", "つぇ");
        map.insert("tso", "つぉ");
        map.insert("tha", "てゃ");
        map.insert("thi", "てぃ");
        map.insert("thu", "てゅ");
        map.insert("the", "てぇ");
        map.insert("tho", "てょ");
        map.insert("twa", "とぁ");
        map.insert("twi", "とぃ");
        map.insert("twu", "とぅ");
        map.insert("twe", "とぇ");
        map.insert("two", "とぉ");
        map.insert("dzu", "づ");
        map.insert("dzi", "ぢ");
        map.insert("da", "だ");
        map.insert("di", "ぢ");
        map.insert("du", "づ");
        map.insert("de", "で");
        map.insert("do", "ど");
        map.insert("dya", "ぢゃ");
        map.insert("dyi", "ぢぃ");
        map.insert("dyu", "ぢゅ");
        map.insert("dye", "ぢぇ");
        map.insert("dyo", "ぢょ");
        map.insert("dha", "でゃ");
        map.insert("dhi", "でぃ");
        map.insert("dhu", "でゅ");
        map.insert("dhe", "でぇ");
        map.insert("dho", "でょ");
        map.insert("dwa", "どぁ");
        map.insert("dwi", "どぃ");
        map.insert("dwu", "どぅ");
        map.insert("dwe", "どぇ");
        map.insert("dwo", "どぉ");
        map.insert("na", "な");
        map.insert("ni", "に");
        map.insert("nu", "ぬ");
        map.insert("ne", "ね");
        map.insert("no", "の");
        map.insert("nya", "にゃ");
        map.insert("nyi", "にぃ");
        map.insert("nyu", "にゅ");
        map.insert("nye", "にぇ");
        map.insert("nyo", "にょ");
        map.insert("fu", "ふ");
        map.insert("ha", "は");
        map.insert("hi", "ひ");
        map.insert("hu", "ふ");
        map.insert("he", "へ");
        map.insert("ho", "ほ");
        map.insert("hya", "ひゃ");
        map.insert("hyi", "ひぃ");
        map.insert("hyu", "ひゅ");
        map.insert("hye", "ひぇ");
        map.insert("hyo", "ひょ");
        map.insert("fya", "ふゃ");
        map.insert("fyi", "ふぃ");
        map.insert("fyu", "ふゅ");
        map.insert("fye", "ふぇ");
        map.insert("fyo", "ふょ");
        map.insert("fa", "ふぁ");
        map.insert("fi", "ふぃ");
        map.insert("fe", "ふぇ");
        map.insert("fo", "ふぉ");
        map.insert("ba", "ば");
        map.insert("bi", "び");
        map.insert("bu", "ぶ");
        map.insert("be", "べ");
        map.insert("bo", "ぼ");
        map.insert("bya", "びゃ");
        map.insert("byi", "びぃ");
        map.insert("byu", "びゅ");
        map.insert("bye", "びぇ");
        map.insert("byo", "びょ");
        map.insert("va", "ヴぁ");
        map.insert("vi", "ヴぃ");
        map.insert("vu", "ヴ");
        map.insert("ve", "ヴぇ");
        map.insert("vo", "ヴぉ");
        map.insert("vya", "ヴゃ");
        map.insert("vyi", "ヴぃ");
        map.insert("vyu", "ヴゅ");
        map.insert("vye", "ヴぇ");
        map.insert("vyo", "ヴょ");
        map.insert("pa", "ぱ");
        map.insert("pi", "ぴ");
        map.insert("pu", "ぷ");
        map.insert("pe", "ぺ");
        map.insert("po", "ぽ");
        map.insert("pya", "ぴゃ");
        map.insert("pyi", "ぴぃ");
        map.insert("pyu", "ぴゅ");
        map.insert("pye", "ぴぇ");
        map.insert("pyo", "ぴょ");
        map.insert("ma", "ま");
        map.insert("mi", "み");
        map.insert("mu", "む");
        map.insert("me", "め");
        map.insert("mo", "も");
        map.insert("mya", "みゃ");
        map.insert("myi", "みぃ");
        map.insert("myu", "みゅ");
        map.insert("mye", "みぇ");
        map.insert("myo", "みょ");
        map.insert("ya", "や");
        map.insert("yu", "ゆ");
        map.insert("yo", "よ");
        map.insert("lya", "ゃ");
        map.insert("lyu", "ゅ");
        map.insert("lyo", "ょ");
        map.insert("xya", "ゃ");
        map.insert("xyu", "ゅ");
        map.insert("xyo", "ょ");
        map.insert("ra", "ら");
        map.insert("ri", "り");
        map.insert("ru", "る");
        map.insert("re", "れ");
        map.insert("ro", "ろ");
        map.insert("rya", "りゃ");
        map.insert("ryi", "りぃ");
        map.insert("ryu", "りゅ");
        map.insert("rye", "りぇ");
        map.insert("ryo", "りょ");
        map.insert("wa", "わ");
        map.insert("wyi", "ゐ");
        map.insert("wye", "ゑ");
        map.insert("wo", "を");
        map.insert("lwa", "ゎ");
        map.insert("xwa", "ゎ");

        // punctuation
        map.insert("[", "「");
        map.insert("]", "」");
        map.insert("*", "＊");
        map.insert("?", "？");
        map.insert(".", "。");
        map.insert(",", "、");

        map
    };
}

lazy_static! {
    pub static ref ROMAJI_TO_KANA_SANS_SOKUON: IndexMap<&'static str, &'static str> = {
        let mut map: IndexMap<&str, &str> = IndexMap::new();

        for (k, v) in ROMAJI_TO_KANA.iter() {
            if !v.contains('っ') {
                map.insert(k, v);
            }
        }

        map
    };
}
