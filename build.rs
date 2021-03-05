extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

trait Map<'a> {
    fn gen(&self, from: Vec<ScriptTup<'a>>, invert: bool) -> phf_codegen::Map<&'a str> {
        let mut map = phf_codegen::Map::new();

        for &ScriptTup(key, value) in from.iter() {
            if invert {
                // NOTE:
                // How is &format allowed here?
                // The string will destruct after the function body is finished
                // Wouldn't that mean the map then points to something that doesn't exist since we lend a string reference?
                map.entry(value, &format!("\"{}\"", key));
            } else {
                map.entry(key, &format!("\"{}\"", value));
            }
        }

        // NOTE:
        // Not a 100% clear on why this works.
        // How can a mutable map be returned from this trait fn, does the caller basically get the ownership for it?
        map
    }
}

trait Gen<'a> {
    fn write(&self, name: &str, buf: &mut BufWriter<File>, map: &phf_codegen::Map<&'a str>) {
        writeln!(
            buf,
            "pub static {}: phf::Map<&str, &str> = \n{};\n",
            name,
            map.build()
        )
        .unwrap()
    }
}

#[derive(Clone)]
struct ScriptTup<'a>(&'a str, &'a str);

struct Geminates<'a> {
    dominant: [ScriptTup<'a>; 9],
    rest: [ScriptTup<'a>; 3],
}

impl Map<'_> for Geminates<'_> {}

impl Gen<'_> for Geminates<'_> {}

impl<'a> Geminates<'a> {
    // combine the dominant + rest and generate a Map
    fn romaji_to_partial_hiragana(self, buf: &mut BufWriter<File>) {
        let data = self.gen(
            // NOTE:
            // Is there a better way to concatenate these arrays?
            self.dominant
                .to_vec()
                .iter()
                .cloned()
                .chain(self.rest.to_vec().iter().cloned())
                .collect(),
            false,
        );

        self.write("GEMINATES_TO_HIRAGANA", buf, &data)
    }

    // generate a Map from only the dominant Geminates
    fn partial_hiragana_to_romaji(self, buf: &mut BufWriter<File>) {
        let data = self.gen(self.dominant.to_vec().iter().cloned().collect(), true);

        self.write("HIRAGANA_TO_GEMINATES", buf, &data)
    }
}

// In the form of: romaji → partial kana
const GEMINATES: Geminates = Geminates {
    dominant: [
        ScriptTup("kk", "っk"),
        ScriptTup("tt", "っt"),
        ScriptTup("cc", "っc"),
        ScriptTup("ss", "っs"),
        ScriptTup("pp", "っp"),
        ScriptTup("mm", "んm"),
        ScriptTup("mt", "んt"),
        ScriptTup("mb", "んb"),
        ScriptTup("mp", "んp"),
    ],

    rest: [
        ScriptTup("nt", "んt"),
        ScriptTup("np", "んp"),
        ScriptTup("nb", "んb"),
    ],
};

struct Romaji<'a> {
    dominant: [ScriptTup<'a>; 205],
    rest: [ScriptTup<'a>; 70],
    with_sokuon: [ScriptTup<'a>; 3],
    punctuation: [ScriptTup<'a>; 6],
}

impl Map<'_> for Romaji<'_> {}

impl Gen<'_> for Romaji<'_> {}


// This does three things:
//
// 1. Inverts the insertion order of ROMAJI_TO_KANA
// 2. Inverts from Map<K,V> to Map<V,K>
// 3. Removes all instances of っ from the inverted mapping
//
// No. 2 ensures that the "dominant" mapping is preferred
// For eg, "ka" over "ca" for "か"
impl<'a> Romaji<'a> {
    // NOTE:
    // How am I allowed to take self as the first parameter here?
    // Shouldn't this be &self?
    // This would mean that I'm passing ownership to this method?
    // But how does that work when my struct is actually a const (see down below)?

    // combine dominant + rest + with_sukuon + punction
    fn romaji_to_hiragana(self, buf: &mut BufWriter<File>) {
        let data = self.gen(
            self.dominant
                .to_vec()
                .iter()
                .cloned()
                .chain(self.rest.to_vec().iter().cloned())
                .chain(self.with_sokuon.to_vec().iter().cloned())
                .chain(self.punctuation.to_vec().iter().cloned())
                .collect(),
            false,
        );

        self.write("ROMAJI_TO_HIRAGANA", buf, &data)
    }

    // combine dominant + puncation and elide rest + with_sukuon
    fn hiragana_to_romaji(self, buf: &mut BufWriter<File>) {
        let data = self.gen(
            self.dominant
                .to_vec()
                .iter()
                .cloned()
                .chain(self.punctuation.to_vec().iter().cloned())
                .collect(),
            true,
        );

        self.write("HIRAGANA_TO_ROMAJI", buf, &data)
    }
}

const ROMAJI: Romaji = Romaji {
    dominant: [
        ScriptTup("n", "ん"),
        ScriptTup("nnn", "んn"),
        ScriptTup("a", "あ"),
        ScriptTup("i", "い"),
        ScriptTup("u", "う"),
        ScriptTup("e", "え"),
        ScriptTup("o", "お"),
        ScriptTup("la", "ぁ"),
        ScriptTup("li", "ぃ"),
        ScriptTup("lu", "ぅ"),
        ScriptTup("le", "ぇ"),
        ScriptTup("lo", "ぉ"),
        ScriptTup("ye", "いぇ"),
        ScriptTup("wi", "うぃ"),
        ScriptTup("we", "うぇ"),
        ScriptTup("wha", "うぁ"),
        ScriptTup("ka", "か"),
        ScriptTup("ki", "き"),
        ScriptTup("ku", "く"),
        ScriptTup("ke", "け"),
        ScriptTup("ko", "こ"),
        ScriptTup("kya", "きゃ"),
        ScriptTup("kyi", "きぃ"),
        ScriptTup("kyu", "きゅ"),
        ScriptTup("kye", "きぇ"),
        ScriptTup("kyo", "きょ"),
        ScriptTup("qya", "くゃ"),
        ScriptTup("qyu", "くゅ"),
        ScriptTup("qyo", "くょ"),
        ScriptTup("lka", "ヵ"),
        ScriptTup("qwa", "くぁ"),
        ScriptTup("qwi", "くぃ"),
        ScriptTup("qwu", "くぅ"),
        ScriptTup("qwe", "くぇ"),
        ScriptTup("qwo", "くぉ"),
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
        ScriptTup("su", "す"),
        ScriptTup("se", "せ"),
        ScriptTup("so", "そ"),
        ScriptTup("sha", "しゃ"),
        ScriptTup("shu", "しゅ"),
        ScriptTup("she", "しぇ"),
        ScriptTup("sho", "しょ"),
        ScriptTup("syi", "しぃ"),
        ScriptTup("swa", "すぁ"),
        ScriptTup("swi", "すぃ"),
        ScriptTup("swu", "すぅ"),
        ScriptTup("swe", "すぇ"),
        ScriptTup("swo", "すぉ"),
        ScriptTup("ji", "じ"),
        ScriptTup("za", "ざ"),
        ScriptTup("zu", "ず"),
        ScriptTup("ze", "ぜ"),
        ScriptTup("zo", "ぞ"),
        ScriptTup("ja", "じゃ"),
        ScriptTup("ju", "じゅ"),
        ScriptTup("je", "じぇ"),
        ScriptTup("jo", "じょ"),
        ScriptTup("jyi", "じぃ"),
        ScriptTup("chi", "ち"),
        ScriptTup("tsu", "つ"),
        ScriptTup("ta", "た"),
        ScriptTup("te", "て"),
        ScriptTup("to", "と"),
        ScriptTup("cha", "ちゃ"),
        ScriptTup("chu", "ちゅ"),
        ScriptTup("che", "ちぇ"),
        ScriptTup("cho", "ちょ"),
        ScriptTup("tyi", "ちぃ"),
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
        ScriptTup("vu", "ヴ"),
        ScriptTup("vo", "ヴぉ"),
        ScriptTup("vya", "ヴゃ"),
        ScriptTup("vi", "ヴぃ"),
        ScriptTup("vyu", "ヴゅ"),
        ScriptTup("ve", "ヴぇ"),
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
        ScriptTup("vye", "ヴぇ"),
        ScriptTup("vyi", "ヴぃ"),
        ScriptTup("fi", "ふぃ"),
        ScriptTup("fe", "ふぇ"),
        ScriptTup("hu", "ふ"),
        ScriptTup("di", "ぢ"),
        ScriptTup("du", "づ"),
        ScriptTup("cye", "ちぇ"),
        ScriptTup("cyo", "ちょ"),
        ScriptTup("tyu", "ちゅ"),
        ScriptTup("cyu", "ちゅ"),
        ScriptTup("tye", "ちぇ"),
        ScriptTup("tyo", "ちょ"),
        ScriptTup("cyi", "ちぃ"),
        ScriptTup("tya", "ちゃ"),
        ScriptTup("cya", "ちゃ"),
        ScriptTup("ti", "ち"),
        ScriptTup("tu", "つ"),
        ScriptTup("zi", "じ"),
        ScriptTup("sya", "しゃ"),
        ScriptTup("syu", "しゅ"),
        ScriptTup("sye", "しぇ"),
        ScriptTup("syo", "しょ"),
        ScriptTup("jya", "じゃ"),
        ScriptTup("jyu", "じゅ"),
        ScriptTup("jye", "じぇ"),
        ScriptTup("jyo", "じょ"),
        ScriptTup("zya", "じゃ"),
        ScriptTup("zyi", "じぃ"),
        ScriptTup("zyu", "じゅ"),
        ScriptTup("zye", "じぇ"),
        ScriptTup("zyo", "じょ"),
        ScriptTup("si", "し"),
        ScriptTup("ci", "し"),
        ScriptTup("ce", "せ"),
        ScriptTup("xka", "ヵ"),
        ScriptTup("qyi", "くぃ"),
        ScriptTup("qye", "くぇ"),
        ScriptTup("kwa", "くぁ"),
        ScriptTup("qa", "くぁ"),
        ScriptTup("qi", "くぃ"),
        ScriptTup("qe", "くぇ"),
        ScriptTup("qo", "くぉ"),
        ScriptTup("xke", "ヶ"),
        ScriptTup("lke", "ヶ"),
        ScriptTup("ca", "か"),
        ScriptTup("cu", "く"),
        ScriptTup("co", "こ"),
        ScriptTup("qu", "く"),
        ScriptTup("whi", "うぃ"),
        ScriptTup("whe", "うぇ"),
        ScriptTup("who", "うぉ"),
        ScriptTup("yi", "い"),
        ScriptTup("lyi", "ぃ"),
        ScriptTup("xyi", "ぃ"),
        ScriptTup("lye", "ぇ"),
        ScriptTup("xye", "ぇ"),
        ScriptTup("xa", "ぁ"),
        ScriptTup("xi", "ぃ"),
        ScriptTup("xu", "ぅ"),
        ScriptTup("xe", "ぇ"),
        ScriptTup("xo", "ぉ"),
        ScriptTup("whu", "う"),
        ScriptTup("nn", "ん"),
        ScriptTup("xn", "ん"),
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

// NOTE:
// This could really just be a constant containing the vector of tuples
// But making this a struct allows us to "inherit" the Map and Gen traits
struct Kana<'a> {
    data: [ScriptTup<'a>; 90],
}

// Direct mappings from hiragana to katakana char-by-char
const KANA: Kana = Kana {
    data: [
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
    ],
};

impl Map<'_> for Kana<'_> {}

impl Gen<'_> for Kana<'_> {}

impl<'a> Kana<'a> {
    fn hiragana_to_katakana(self, buf: &mut BufWriter<File>) {
        let data = self.gen(self.data.to_vec(), false);

        self.write("HIRAGANA_TO_KATAKANA", buf, &data)
    }

    fn katakana_to_hiragana(self, buf: &mut BufWriter<File>) {
        let data = self.gen(self.data.to_vec(), true);

        self.write("KATAKANA_TO_HIRAGANA", buf, &data)
    }
}

// Generate all permutations of necessary data mappings at compile-time.
// This avoids the use of lazy_static! and hand-writing repetitive data in a file.
// The disadvantage is that the actual data is not easily debuggable/visible to humans.
// However, tests in the source code can solve the problem of correctness (you'd need those anyway).
fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("data.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    GEMINATES.romaji_to_partial_hiragana(&mut file);
    GEMINATES.partial_hiragana_to_romaji(&mut file);
    ROMAJI.romaji_to_hiragana(&mut file);
    ROMAJI.hiragana_to_romaji(&mut file);
    KANA.katakana_to_hiragana(&mut file);
    KANA.hiragana_to_katakana(&mut file);
}
