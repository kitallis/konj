extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Clone)]
struct ScriptTup<'a> {
    romaji: &'a str,
    kana: &'a str,
}

struct KanaGeminates<'a> {
    favored: [ScriptTup<'a>; 9],
    rest: [ScriptTup<'a>; 3],
}

// In the form of: romaji → partial kana
const GEMINATES_AND_KANA: KanaGeminates<'static> = KanaGeminates {
    favored: [
        ScriptTup {
            romaji: "kk",
            kana: "っk",
        },
        ScriptTup {
            romaji: "tt",
            kana: "っt",
        },
        ScriptTup {
            romaji: "cc",
            kana: "っc",
        },
        ScriptTup {
            romaji: "ss",
            kana: "っs",
        },
        ScriptTup {
            romaji: "pp",
            kana: "っp",
        },
        ScriptTup {
            romaji: "mm",
            kana: "んm",
        },
        ScriptTup {
            romaji: "mt",
            kana: "んt",
        },
        ScriptTup {
            romaji: "mb",
            kana: "んb",
        },
        ScriptTup {
            romaji: "mp",
            kana: "んp",
        },
    ],

    rest: [
        ScriptTup {
            romaji: "nt",
            kana: "んt",
        },
        ScriptTup {
            romaji: "np",
            kana: "んp",
        },
        ScriptTup {
            romaji: "nb",
            kana: "んb",
        },
    ],
};

struct KanaRomaji<'a> {
    favored: [ScriptTup<'a>; 277],
    rest: [ScriptTup<'a>; 9],
    with_sokuon: [ScriptTup<'a>; 3],
    punctuation: [ScriptTup<'a>; 6],
}

const HIRAGANA_AND_ROMAJI: KanaRomaji<'static> = KanaRomaji {
    favored: [
        ScriptTup("n", "ん"),
        ScriptTup("nnn", "んn"),
        ScriptTup("a", "あ"),
        ScriptTup("i", "い"),
        ScriptTup("u", "う"),
        ScriptTup("e", "え"),
        ScriptTup("o", "お"),
        ScriptTup("yi", "い"),
        ScriptTup("la", "ぁ"),
        ScriptTup("li", "ぃ"),
        ScriptTup("lu", "ぅ"),
        ScriptTup("le", "ぇ"),
        ScriptTup("lo", "ぉ"),
        ScriptTup("xa", "ぁ"),
        ScriptTup("xi", "ぃ"),
        ScriptTup("xu", "ぅ"),
        ScriptTup("xe", "ぇ"),
        ScriptTup("xo", "ぉ"),
        ScriptTup("lyi", "ぃ"),
        ScriptTup("xyi", "ぃ"),
        ScriptTup("lye", "ぇ"),
        ScriptTup("xye", "ぇ"),
        ScriptTup("ye", "いぇ"),
        ScriptTup("wi", "うぃ"),
        ScriptTup("we", "うぇ"),
        ScriptTup("wha", "うぁ"),
        ScriptTup("whi", "うぃ"),
        ScriptTup("whe", "うぇ"),
        ScriptTup("who", "うぉ"),
        ScriptTup("vu", "ヴ"),
        ScriptTup("va", "ヴぁ"),
        ScriptTup("vi", "ヴぃ"),
        ScriptTup("vyi", "ヴぃ"),
        ScriptTup("ve", "ヴぇ"),
        ScriptTup("vye", "ヴぇ"),
        ScriptTup("vo", "ヴぉ"),
        ScriptTup("vya", "ヴゃ"),
        ScriptTup("vyu", "ヴゅ"),
        ScriptTup("vyo", "ヴょ"),
        ScriptTup("ka", "か"),
        ScriptTup("ki", "き"),
        ScriptTup("ku", "く"),
        ScriptTup("ke", "け"),
        ScriptTup("ko", "こ"),
        ScriptTup("ca", "か"),
        ScriptTup("cu", "く"),
        ScriptTup("co", "こ"),
        ScriptTup("qu", "く"),
        ScriptTup("kya", "きゃ"),
        ScriptTup("kyi", "きぃ"),
        ScriptTup("kyu", "きゅ"),
        ScriptTup("kye", "きぇ"),
        ScriptTup("kyo", "きょ"),
        ScriptTup("qya", "くゃ"),
        ScriptTup("qyu", "くゅ"),
        ScriptTup("qyo", "くょ"),
        ScriptTup("lka", "ヵ"),
        ScriptTup("xka", "ヵ"),
        ScriptTup("lke", "ヶ"),
        ScriptTup("xke", "ヶ"),
        ScriptTup("qwa", "くぁ"),
        ScriptTup("qwi", "くぃ"),
        ScriptTup("qwu", "くぅ"),
        ScriptTup("qwe", "くぇ"),
        ScriptTup("qwo", "くぉ"),
        ScriptTup("qa", "くぁ"),
        ScriptTup("qi", "くぃ"),
        ScriptTup("qe", "くぇ"),
        ScriptTup("qo", "くぉ"),
        ScriptTup("kwa", "くぁ"),
        ScriptTup("qyi", "くぃ"),
        ScriptTup("qye", "くぇ"),
        ScriptTup("ga", "が"),
        ScriptTup("gi", "ぎ"),
        ScriptTup("gu", "ぐ"),
        ScriptTup("ge", "げ"),
        ScriptTup("go", "ご"),
        ScriptTup("gya", "ぎゃ"),
        ScriptTup("gyi", "ぎぃ"),
        ScriptTup("gyu", "ぎゅ"),
        ScriptTup("gye", "ぎぇ"),
        ScriptTup("gyo", "ぎょ"),
        ScriptTup("gwa", "ぐぁ"),
        ScriptTup("gwi", "ぐぃ"),
        ScriptTup("gwu", "ぐぅ"),
        ScriptTup("gwe", "ぐぇ"),
        ScriptTup("gwo", "ぐぉ"),
        ScriptTup("shi", "し"),
        ScriptTup("sa", "さ"),
        ScriptTup("si", "し"),
        ScriptTup("su", "す"),
        ScriptTup("se", "せ"),
        ScriptTup("so", "そ"),
        ScriptTup("ci", "し"),
        ScriptTup("ce", "せ"),
        ScriptTup("sha", "しゃ"),
        ScriptTup("shu", "しゅ"),
        ScriptTup("she", "しぇ"),
        ScriptTup("sho", "しょ"),
        ScriptTup("sya", "しゃ"),
        ScriptTup("syi", "しぃ"),
        ScriptTup("syu", "しゅ"),
        ScriptTup("sye", "しぇ"),
        ScriptTup("syo", "しょ"),
        ScriptTup("swa", "すぁ"),
        ScriptTup("swi", "すぃ"),
        ScriptTup("swu", "すぅ"),
        ScriptTup("swe", "すぇ"),
        ScriptTup("swo", "すぉ"),
        ScriptTup("ji", "じ"),
        ScriptTup("za", "ざ"),
        ScriptTup("zi", "じ"),
        ScriptTup("zu", "ず"),
        ScriptTup("ze", "ぜ"),
        ScriptTup("zo", "ぞ"),
        ScriptTup("ja", "じゃ"),
        ScriptTup("ju", "じゅ"),
        ScriptTup("je", "じぇ"),
        ScriptTup("jo", "じょ"),
        ScriptTup("jya", "じゃ"),
        ScriptTup("jyi", "じぃ"),
        ScriptTup("jyu", "じゅ"),
        ScriptTup("jye", "じぇ"),
        ScriptTup("jyo", "じょ"),
        ScriptTup("zya", "じゃ"),
        ScriptTup("zyi", "じぃ"),
        ScriptTup("zyu", "じゅ"),
        ScriptTup("zye", "じぇ"),
        ScriptTup("zyo", "じょ"),
        ScriptTup("chi", "ち"),
        ScriptTup("tsu", "つ"),
        ScriptTup("ta", "た"),
        ScriptTup("ti", "ち"),
        ScriptTup("tu", "つ"),
        ScriptTup("te", "て"),
        ScriptTup("to", "と"),
        ScriptTup("cha", "ちゃ"),
        ScriptTup("chu", "ちゅ"),
        ScriptTup("che", "ちぇ"),
        ScriptTup("cho", "ちょ"),
        ScriptTup("tya", "ちゃ"),
        ScriptTup("tyi", "ちぃ"),
        ScriptTup("tyu", "ちゅ"),
        ScriptTup("tye", "ちぇ"),
        ScriptTup("tyo", "ちょ"),
        ScriptTup("cya", "ちゃ"),
        ScriptTup("cyi", "ちぃ"),
        ScriptTup("cyu", "ちゅ"),
        ScriptTup("cye", "ちぇ"),
        ScriptTup("cyo", "ちょ"),
        ScriptTup("tsa", "つぁ"),
        ScriptTup("tsi", "つぃ"),
        ScriptTup("tse", "つぇ"),
        ScriptTup("tso", "つぉ"),
        ScriptTup("tha", "てゃ"),
        ScriptTup("thi", "てぃ"),
        ScriptTup("thu", "てゅ"),
        ScriptTup("the", "てぇ"),
        ScriptTup("tho", "てょ"),
        ScriptTup("twa", "とぁ"),
        ScriptTup("twi", "とぃ"),
        ScriptTup("twu", "とぅ"),
        ScriptTup("twe", "とぇ"),
        ScriptTup("two", "とぉ"),
        ScriptTup("dzu", "づ"),
        ScriptTup("dzi", "ぢ"),
        ScriptTup("da", "だ"),
        ScriptTup("di", "ぢ"),
        ScriptTup("du", "づ"),
        ScriptTup("de", "で"),
        ScriptTup("do", "ど"),
        ScriptTup("dya", "ぢゃ"),
        ScriptTup("dyi", "ぢぃ"),
        ScriptTup("dyu", "ぢゅ"),
        ScriptTup("dye", "ぢぇ"),
        ScriptTup("dyo", "ぢょ"),
        ScriptTup("dha", "でゃ"),
        ScriptTup("dhi", "でぃ"),
        ScriptTup("dhu", "でゅ"),
        ScriptTup("dhe", "でぇ"),
        ScriptTup("dho", "でょ"),
        ScriptTup("dwa", "どぁ"),
        ScriptTup("dwi", "どぃ"),
        ScriptTup("dwu", "どぅ"),
        ScriptTup("dwe", "どぇ"),
        ScriptTup("dwo", "どぉ"),
        ScriptTup("na", "な"),
        ScriptTup("ni", "に"),
        ScriptTup("nu", "ぬ"),
        ScriptTup("ne", "ね"),
        ScriptTup("no", "の"),
        ScriptTup("nya", "にゃ"),
        ScriptTup("nyi", "にぃ"),
        ScriptTup("nyu", "にゅ"),
        ScriptTup("nye", "にぇ"),
        ScriptTup("nyo", "にょ"),
        ScriptTup("fu", "ふ"),
        ScriptTup("ha", "は"),
        ScriptTup("hi", "ひ"),
        ScriptTup("hu", "ふ"),
        ScriptTup("he", "へ"),
        ScriptTup("ho", "ほ"),
        ScriptTup("hya", "ひゃ"),
        ScriptTup("hyi", "ひぃ"),
        ScriptTup("hyu", "ひゅ"),
        ScriptTup("hye", "ひぇ"),
        ScriptTup("hyo", "ひょ"),
        ScriptTup("fya", "ふゃ"),
        ScriptTup("fyi", "ふぃ"),
        ScriptTup("fyu", "ふゅ"),
        ScriptTup("fye", "ふぇ"),
        ScriptTup("fyo", "ふょ"),
        ScriptTup("fa", "ふぁ"),
        ScriptTup("fi", "ふぃ"),
        ScriptTup("fe", "ふぇ"),
        ScriptTup("fo", "ふぉ"),
        ScriptTup("ba", "ば"),
        ScriptTup("bi", "び"),
        ScriptTup("bu", "ぶ"),
        ScriptTup("be", "べ"),
        ScriptTup("bo", "ぼ"),
        ScriptTup("bya", "びゃ"),
        ScriptTup("byi", "びぃ"),
        ScriptTup("byu", "びゅ"),
        ScriptTup("bye", "びぇ"),
        ScriptTup("byo", "びょ"),
        ScriptTup("va", "ヴぁ"),
        ScriptTup("vi", "ヴぃ"),
        ScriptTup("vu", "ヴ"),
        ScriptTup("ve", "ヴぇ"),
        ScriptTup("vo", "ヴぉ"),
        ScriptTup("vya", "ヴゃ"),
        ScriptTup("vyi", "ヴぃ"),
        ScriptTup("vyu", "ヴゅ"),
        ScriptTup("vye", "ヴぇ"),
        ScriptTup("vyo", "ヴょ"),
        ScriptTup("pa", "ぱ"),
        ScriptTup("pi", "ぴ"),
        ScriptTup("pu", "ぷ"),
        ScriptTup("pe", "ぺ"),
        ScriptTup("po", "ぽ"),
        ScriptTup("pya", "ぴゃ"),
        ScriptTup("pyi", "ぴぃ"),
        ScriptTup("pyu", "ぴゅ"),
        ScriptTup("pye", "ぴぇ"),
        ScriptTup("pyo", "ぴょ"),
        ScriptTup("ma", "ま"),
        ScriptTup("mi", "み"),
        ScriptTup("mu", "む"),
        ScriptTup("me", "め"),
        ScriptTup("mo", "も"),
        ScriptTup("mya", "みゃ"),
        ScriptTup("myi", "みぃ"),
        ScriptTup("myu", "みゅ"),
        ScriptTup("mye", "みぇ"),
        ScriptTup("myo", "みょ"),
        ScriptTup("ya", "や"),
        ScriptTup("yu", "ゆ"),
        ScriptTup("yo", "よ"),
        ScriptTup("lya", "ゃ"),
        ScriptTup("lyu", "ゅ"),
        ScriptTup("lyo", "ょ"),
        ScriptTup("ra", "ら"),
        ScriptTup("ri", "り"),
        ScriptTup("ru", "る"),
        ScriptTup("re", "れ"),
        ScriptTup("ro", "ろ"),
        ScriptTup("rya", "りゃ"),
        ScriptTup("ryi", "りぃ"),
        ScriptTup("ryu", "りゅ"),
        ScriptTup("rye", "りぇ"),
        ScriptTup("ryo", "りょ"),
        ScriptTup("wa", "わ"),
        ScriptTup("wyi", "ゐ"),
        ScriptTup("wye", "ゑ"),
        ScriptTup("wo", "を"),
        ScriptTup("lwa", "ゎ"),
    ],

    rest: [
        ScriptTup("whu", "う"),
        ScriptTup("nn", "ん"),
        ScriptTup("xn", "ん"),
        ScriptTup("n", "ん"),
        ScriptTup("wu", "う"),
        ScriptTup("xya", "ゃ"),
        ScriptTup("xyu", "ゅ"),
        ScriptTup("xyo", "ょ"),
        ScriptTup("xwa", "ゎ"),
    ],

    with_sokuon: [
        ScriptTup("ltu", "っ"),
        ScriptTup("xtu", "っ"),
        ScriptTup("ltsu", "っ"),
    ],

    punctuation: [
        ScriptTup("[", "「"),
        ScriptTup("]", "」"),
        ScriptTup("*", "＊"),
        ScriptTup("?", "？"),
        ScriptTup(".", "。"),
        ScriptTup(",", "、"),
    ],
};

// Direct mappings from hiragana to katakana char-by-char
const HIRAGANA_AND_KATAKANA: [ScriptTup; 90] = [
    ScriptTup("ぁ", "ァ"),
    ScriptTup("あ", "ア"),
    ScriptTup("ぃ", "ィ"),
    ScriptTup("い", "イ"),
    ScriptTup("ぅ", "ゥ"),
    ScriptTup("う", "ウ"),
    ScriptTup("ぇ", "ェ"),
    ScriptTup("え", "エ"),
    ScriptTup("ぉ", "ォ"),
    ScriptTup("お", "オ"),
    ScriptTup("か", "カ"),
    ScriptTup("が", "ガ"),
    ScriptTup("き", "キ"),
    ScriptTup("ぎ", "ギ"),
    ScriptTup("く", "ク"),
    ScriptTup("ぐ", "グ"),
    ScriptTup("け", "ケ"),
    ScriptTup("げ", "ゲ"),
    ScriptTup("こ", "コ"),
    ScriptTup("ご", "ゴ"),
    ScriptTup("さ", "サ"),
    ScriptTup("ざ", "ザ"),
    ScriptTup("し", "シ"),
    ScriptTup("じ", "ジ"),
    ScriptTup("す", "ス"),
    ScriptTup("ず", "ズ"),
    ScriptTup("せ", "セ"),
    ScriptTup("ぜ", "ゼ"),
    ScriptTup("そ", "ソ"),
    ScriptTup("ぞ", "ゾ"),
    ScriptTup("た", "タ"),
    ScriptTup("だ", "ダ"),
    ScriptTup("ち", "チ"),
    ScriptTup("ぢ", "ヂ"),
    ScriptTup("っ", "ッ"),
    ScriptTup("つ", "ツ"),
    ScriptTup("づ", "ヅ"),
    ScriptTup("て", "テ"),
    ScriptTup("で", "デ"),
    ScriptTup("と", "ト"),
    ScriptTup("ど", "ド"),
    ScriptTup("な", "ナ"),
    ScriptTup("に", "ニ"),
    ScriptTup("ぬ", "ヌ"),
    ScriptTup("ね", "ネ"),
    ScriptTup("の", "ノ"),
    ScriptTup("は", "ハ"),
    ScriptTup("ば", "バ"),
    ScriptTup("ぱ", "パ"),
    ScriptTup("ひ", "ヒ"),
    ScriptTup("び", "ビ"),
    ScriptTup("ぴ", "ピ"),
    ScriptTup("ふ", "フ"),
    ScriptTup("ぶ", "ブ"),
    ScriptTup("ぷ", "プ"),
    ScriptTup("へ", "ヘ"),
    ScriptTup("べ", "ベ"),
    ScriptTup("ぺ", "ペ"),
    ScriptTup("ほ", "ホ"),
    ScriptTup("ぼ", "ボ"),
    ScriptTup("ぽ", "ポ"),
    ScriptTup("ま", "マ"),
    ScriptTup("み", "ミ"),
    ScriptTup("む", "ム"),
    ScriptTup("め", "メ"),
    ScriptTup("も", "モ"),
    ScriptTup("ゃ", "ャ"),
    ScriptTup("や", "ヤ"),
    ScriptTup("ゅ", "ュ"),
    ScriptTup("ゆ", "ユ"),
    ScriptTup("ょ", "ョ"),
    ScriptTup("よ", "ヨ"),
    ScriptTup("ら", "ラ"),
    ScriptTup("り", "リ"),
    ScriptTup("る", "ル"),
    ScriptTup("れ", "レ"),
    ScriptTup("ろ", "ロ"),
    ScriptTup("ゎ", "ヮ"),
    ScriptTup("わ", "ワ"),
    ScriptTup("ゐ", "ヰ"),
    ScriptTup("ゑ", "ヱ"),
    ScriptTup("を", "ヲ"),
    ScriptTup("ん", "ン"),
    ScriptTup("ゔ", "ヴ"),
    ScriptTup("ゕ", "ヵ"),
    ScriptTup("ゖ", "ヶ"),
    ScriptTup("わ゛", "ヷ"),
    ScriptTup("ゐ゛", "ヸ"),
    ScriptTup("ゑ゛", "ヹ"),
    ScriptTup("を゛", "ヺ"),
];

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("data.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    gen_geminates_to_kana(&mut file);
    gen_kana_to_geminates(&mut file);
    gen_hiragana_to_katakana(&mut file);
    gen_katakana_to_hiragana(&mut file);
}

fn gen_geminates_to_kana(file: &mut BufWriter<File>) {
    let mut romaji_geminates_to_partial_kana = phf_codegen::Map::new();

    gen_map(
        &mut romaji_geminates_to_partial_kana,
        GEMINATES_AND_KANA.favored.to_vec(),
        false,
    );

    gen_map(
        &mut romaji_geminates_to_partial_kana,
        GEMINATES_AND_KANA.rest.to_vec(),
        false,
    );

    writeln!(
        file,
        "pub static GEMINATES_TO_KANA: phf::Map<&'static str, &'static str> = \n{};\n",
        romaji_geminates_to_partial_kana.build()
    )
    .unwrap();
}

fn gen_kana_to_geminates(file: &mut BufWriter<File>) {
    let mut partial_kana_to_romaji_geminates = phf_codegen::Map::new();

    gen_map(
        &mut partial_kana_to_romaji_geminates,
        GEMINATES_AND_KANA.rest.to_vec(),
        true,
    );

    writeln!(
        file,
        "pub static KANA_TO_GEMINATES: phf::Map<&'static str, &'static str> = \n{};\n",
        partial_kana_to_romaji_geminates.build()
    )
    .unwrap();
}

fn gen_hiragana_to_katakana(file: &mut BufWriter<File>) {
    let mut hiragana_to_katakana = phf_codegen::Map::new();

    gen_map(
        &mut hiragana_to_katakana,
        HIRAGANA_AND_KATAKANA.to_vec(),
        false,
    );

    writeln!(
        file,
        "pub static HIRAGANA_TO_KATAKANA: phf::Map<&'static str, &'static str> = \n{};\n",
        hiragana_to_katakana.build()
    )
    .unwrap();
}

fn gen_katakana_to_hiragana(file: &mut BufWriter<File>) {
    let mut katakana_to_hiragana = phf_codegen::Map::new();

    gen_map(
        &mut katakana_to_hiragana,
        HIRAGANA_AND_KATAKANA.to_vec(),
        true,
    );

    writeln!(
        file,
        "pub static KATAKANA_TO_HIRAGANA: phf::Map<&'static str, &'static str> = \n{};\n",
        katakana_to_hiragana.build()
    )
    .unwrap();
}

// Populate the phf_codegen::Map with the vector of tuples in `from`.
// Vec<(a,b)> -> Map<a,b>
//
// We can also generate this map in an inverted way, such that,
// Vec<(a,b)> -> Map<b,a>
fn gen_map<'a>(of: &mut phf_codegen::Map<&'a str>, from: Vec<ScriptTup<'a>>, invert: bool) {
    for &ScriptTup(key, value) in from.iter() {
        if invert {
            of.entry(value, &format!("\"{}\"", key));
        } else {
            of.entry(key, &format!("\"{}\"", value));
        }
    }
}
