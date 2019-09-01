use std::io::stdin;
use std::string::String;
use std::option::Option;
use std::option::Option::Some;
use std::option::Option::None;

//
// hiragana + katakana
//
static KANA_BEG: char = '\u{3040}';
static KANA_END: char = '\u{30FF}';

//
// full-width roman
//
static FULL_WIDTH_ROMAN_BEG: char = '\u{0021}';
static FULL_WIDTH_ROMAN_END: char = '\u{007E}';

fn main() {
    println!("Konj: convert from one japanese script to all");

    let mut input = String::new();
    stdin().read_line(&mut input);

    if is_fully_kana(&input.trim()) {
        println!("You entered kana. Converting to romaji...");
        //
        // do a mutable borrow
        //
        //        to_romaji(&input.trim());
    } else if is_fully_romaji(&input.trim()) {
        println!("You entered romaji. Converting to kana...");
        //
        // do a mutable borrow

        to_kana(&input.trim(), true);
    } else {
        println!("Did not understand input character set.")
    }
}

fn to_kana(s: &str, katakana: bool) -> str {
    // perform double-consonant mappings
    // lookup romaji -> kana mappings
    // kana -> katakana if necessary
}


fn to_romaji(s: &str) -> str {}

//
// borrow 's' as immutable
//
fn is_fully_kana(s: &str) -> bool {
    for ch in s.trim().chars() {
        //
        // KANA_BEG and KANA_END are borrowed
        //
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

fn to_double_constants(s: &str) -> str {
    let muz = s.chars().peekable();

    while z.peek().is_some() {
        double_consonants(z.take(2).collect())
    }
}

//
// using `match` over `lazy_static` or `phf` for cognitive simplicity
//
fn double_consonants(s: &str) -> Option<&str> {
    match s {
        "kk" => Some("っk"),
        "tt" => Some("っt"),
        "cc" => Some("っc"),
        "ss" => Some("っs"),
        "pp" => Some("っp"),
        "mm" => Some("んm"),
        "mt" => Some("んt"),
        "mb" => Some("んb"),
        "mp" => Some("んp"),
        "nt" => Some("んt"),
        "nb" => Some("んb"),
        "np" => Some("んp"),
        _ => None
    }
}

fn romaji_to_kana(s: &str) -> Option<&str> {
    match s {
        "n" => Some("ん"),
        "nnn" => Some("んn"),
        "nn" => Some("ん"),
        "xn" => Some("ん"),
        "n" => Some("ん"),
        "a" => Some("あ"),
        "i" => Some("い"),
        "u" => Some("う"),
        "e" => Some("え"),
        "o" => Some("お"),
        "yi" => Some("い"),
        "wu" => Some("う"),
        "whu" => Some("う"),
        "la" => Some("ぁ"),
        "li" => Some("ぃ"),
        "lu" => Some("ぅ"),
        "le" => Some("ぇ"),
        "lo" => Some("ぉ"),
        "xa" => Some("ぁ"),
        "xi" => Some("ぃ"),
        "xu" => Some("ぅ"),
        "xe" => Some("ぇ"),
        "xo" => Some("ぉ"),
        "lyi" => Some("ぃ"),
        "xyi" => Some("ぃ"),
        "lye" => Some("ぇ"),
        "xye" => Some("ぇ"),
        "ye" => Some("いぇ"),
        "wi" => Some("うぃ"),
        "we" => Some("うぇ"),
        "wha" => Some("うぁ"),
        "whi" => Some("うぃ"),
        "whe" => Some("うぇ"),
        "who" => Some("うぉ"),
        "vu" => Some("ヴ"),
        "va" => Some("ヴぁ"),
        "vi" => Some("ヴぃ"),
        "vyi" => Some("ヴぃ"),
        "ve" => Some("ヴぇ"),
        "vye" => Some("ヴぇ"),
        "vo" => Some("ヴぉ"),
        "vya" => Some("ヴゃ"),
        "vyu" => Some("ヴゅ"),
        "vyo" => Some("ヴょ"),
        "ka" => Some("か"),
        "ki" => Some("き"),
        "ku" => Some("く"),
        "ke" => Some("け"),
        "ko" => Some("こ"),
        "ca" => Some("か"),
        "cu" => Some("く"),
        "co" => Some("こ"),
        "qu" => Some("く"),
        "kya" => Some("きゃ"),
        "kyi" => Some("きぃ"),
        "kyu" => Some("きゅ"),
        "kye" => Some("きぇ"),
        "kyo" => Some("きょ"),
        "qya" => Some("くゃ"),
        "qyu" => Some("くゅ"),
        "qyo" => Some("くょ"),
        "lka" => Some("ヵ"),
        "xka" => Some("ヵ"),
        "lke" => Some("ヶ"),
        "xke" => Some("ヶ"),
        "qwa" => Some("くぁ"),
        "qwi" => Some("くぃ"),
        "qwu" => Some("くぅ"),
        "qwe" => Some("くぇ"),
        "qwo" => Some("くぉ"),
        "qa" => Some("くぁ"),
        "qi" => Some("くぃ"),
        "qe" => Some("くぇ"),
        "qo" => Some("くぉ"),
        "kwa" => Some("くぁ"),
        "qyi" => Some("くぃ"),
        "qye" => Some("くぇ"),
        "ga" => Some("が"),
        "gi" => Some("ぎ"),
        "gu" => Some("ぐ"),
        "ge" => Some("げ"),
        "go" => Some("ご"),
        "gya" => Some("ぎゃ"),
        "gyi" => Some("ぎぃ"),
        "gyu" => Some("ぎゅ"),
        "gye" => Some("ぎぇ"),
        "gyo" => Some("ぎょ"),
        "gwa" => Some("ぐぁ"),
        "gwi" => Some("ぐぃ"),
        "gwu" => Some("ぐぅ"),
        "gwe" => Some("ぐぇ"),
        "gwo" => Some("ぐぉ"),
        "shi" => Some("し"),
        "sa" => Some("さ"),
        "si" => Some("し"),
        "su" => Some("す"),
        "se" => Some("せ"),
        "so" => Some("そ"),
        "ci" => Some("し"),
        "ce" => Some("せ"),
        "sha" => Some("しゃ"),
        "shu" => Some("しゅ"),
        "she" => Some("しぇ"),
        "sho" => Some("しょ"),
        "sya" => Some("しゃ"),
        "syi" => Some("しぃ"),
        "syu" => Some("しゅ"),
        "sye" => Some("しぇ"),
        "syo" => Some("しょ"),
        "swa" => Some("すぁ"),
        "swi" => Some("すぃ"),
        "swu" => Some("すぅ"),
        "swe" => Some("すぇ"),
        "swo" => Some("すぉ"),
        "ji" => Some("じ"),
        "za" => Some("ざ"),
        "zi" => Some("じ"),
        "zu" => Some("ず"),
        "ze" => Some("ぜ"),
        "zo" => Some("ぞ"),
        "ja" => Some("じゃ"),
        "ju" => Some("じゅ"),
        "je" => Some("じぇ"),
        "jo" => Some("じょ"),
        "jya" => Some("じゃ"),
        "jyi" => Some("じぃ"),
        "jyu" => Some("じゅ"),
        "jye" => Some("じぇ"),
        "jyo" => Some("じょ"),
        "zya" => Some("じゃ"),
        "zyi" => Some("じぃ"),
        "zyu" => Some("じゅ"),
        "zye" => Some("じぇ"),
        "zyo" => Some("じょ"),
        "chi" => Some("ち"),
        "tsu" => Some("つ"),
        "ta" => Some("た"),
        "ti" => Some("ち"),
        "tu" => Some("つ"),
        "te" => Some("て"),
        "to" => Some("と"),
        "cha" => Some("ちゃ"),
        "chu" => Some("ちゅ"),
        "che" => Some("ちぇ"),
        "cho" => Some("ちょ"),
        "tya" => Some("ちゃ"),
        "tyi" => Some("ちぃ"),
        "tyu" => Some("ちゅ"),
        "tye" => Some("ちぇ"),
        "tyo" => Some("ちょ"),
        "cya" => Some("ちゃ"),
        "cyi" => Some("ちぃ"),
        "cyu" => Some("ちゅ"),
        "cye" => Some("ちぇ"),
        "cyo" => Some("ちょ"),
        "ltu" => Some("っ"),
        "xtu" => Some("っ"),
        "ltsu" => Some("っ"),
        "tsa" => Some("つぁ"),
        "tsi" => Some("つぃ"),
        "tse" => Some("つぇ"),
        "tso" => Some("つぉ"),
        "tha" => Some("てゃ"),
        "thi" => Some("てぃ"),
        "thu" => Some("てゅ"),
        "the" => Some("てぇ"),
        "tho" => Some("てょ"),
        "twa" => Some("とぁ"),
        "twi" => Some("とぃ"),
        "twu" => Some("とぅ"),
        "twe" => Some("とぇ"),
        "two" => Some("とぉ"),
        "dzu" => Some("づ"),
        "dzi" => Some("ぢ"),
        "da" => Some("だ"),
        "di" => Some("ぢ"),
        "du" => Some("づ"),
        "de" => Some("で"),
        "do" => Some("ど"),
        "dya" => Some("ぢゃ"),
        "dyi" => Some("ぢぃ"),
        "dyu" => Some("ぢゅ"),
        "dye" => Some("ぢぇ"),
        "dyo" => Some("ぢょ"),
        "dha" => Some("でゃ"),
        "dhi" => Some("でぃ"),
        "dhu" => Some("でゅ"),
        "dhe" => Some("でぇ"),
        "dho" => Some("でょ"),
        "dwa" => Some("どぁ"),
        "dwi" => Some("どぃ"),
        "dwu" => Some("どぅ"),
        "dwe" => Some("どぇ"),
        "dwo" => Some("どぉ"),
        "na" => Some("な"),
        "ni" => Some("に"),
        "nu" => Some("ぬ"),
        "ne" => Some("ね"),
        "no" => Some("の"),
        "nya" => Some("にゃ"),
        "nyi" => Some("にぃ"),
        "nyu" => Some("にゅ"),
        "nye" => Some("にぇ"),
        "nyo" => Some("にょ"),
        "fu" => Some("ふ"),
        "ha" => Some("は"),
        "hi" => Some("ひ"),
        "hu" => Some("ふ"),
        "he" => Some("へ"),
        "ho" => Some("ほ"),
        "hya" => Some("ひゃ"),
        "hyi" => Some("ひぃ"),
        "hyu" => Some("ひゅ"),
        "hye" => Some("ひぇ"),
        "hyo" => Some("ひょ"),
        "fya" => Some("ふゃ"),
        "fyi" => Some("ふぃ"),
        "fyu" => Some("ふゅ"),
        "fye" => Some("ふぇ"),
        "fyo" => Some("ふょ"),
        "fa" => Some("ふぁ"),
        "fi" => Some("ふぃ"),
        "fe" => Some("ふぇ"),
        "fo" => Some("ふぉ"),
        "ba" => Some("ば"),
        "bi" => Some("び"),
        "bu" => Some("ぶ"),
        "be" => Some("べ"),
        "bo" => Some("ぼ"),
        "bya" => Some("びゃ"),
        "byi" => Some("びぃ"),
        "byu" => Some("びゅ"),
        "bye" => Some("びぇ"),
        "byo" => Some("びょ"),
        "va" => Some("ヴぁ"),
        "vi" => Some("ヴぃ"),
        "vu" => Some("ヴ"),
        "ve" => Some("ヴぇ"),
        "vo" => Some("ヴぉ"),
        "vya" => Some("ヴゃ"),
        "vyi" => Some("ヴぃ"),
        "vyu" => Some("ヴゅ"),
        "vye" => Some("ヴぇ"),
        "vyo" => Some("ヴょ"),
        "pa" => Some("ぱ"),
        "pi" => Some("ぴ"),
        "pu" => Some("ぷ"),
        "pe" => Some("ぺ"),
        "po" => Some("ぽ"),
        "pya" => Some("ぴゃ"),
        "pyi" => Some("ぴぃ"),
        "pyu" => Some("ぴゅ"),
        "pye" => Some("ぴぇ"),
        "pyo" => Some("ぴょ"),
        "ma" => Some("ま"),
        "mi" => Some("み"),
        "mu" => Some("む"),
        "me" => Some("め"),
        "mo" => Some("も"),
        "mya" => Some("みゃ"),
        "myi" => Some("みぃ"),
        "myu" => Some("みゅ"),
        "mye" => Some("みぇ"),
        "myo" => Some("みょ"),
        "ya" => Some("や"),
        "yu" => Some("ゆ"),
        "yo" => Some("よ"),
        "lya" => Some("ゃ"),
        "lyu" => Some("ゅ"),
        "lyo" => Some("ょ"),
        "xya" => Some("ゃ"),
        "xyu" => Some("ゅ"),
        "xyo" => Some("ょ"),
        "ra" => Some("ら"),
        "ri" => Some("り"),
        "ru" => Some("る"),
        "re" => Some("れ"),
        "ro" => Some("ろ"),
        "rya" => Some("りゃ"),
        "ryi" => Some("りぃ"),
        "ryu" => Some("りゅ"),
        "rye" => Some("りぇ"),
        "ryo" => Some("りょ"),
        "wa" => Some("わ"),
        "wyi" => Some("ゐ"),
        "wye" => Some("ゑ"),
        "wo" => Some("を"),
        "lwa" => Some("ゎ"),
        "xwa" => Some("ゎ"),
        _ => None
    }
}

fn hiragana_to_katakana(s: &str) -> Option<&str> {
    match s {
        "ぁ" => Some("ァ"),
        "あ" => Some("ア"),
        "ぃ" => Some("ィ"),
        "い" => Some("イ"),
        "ぅ" => Some("ゥ"),
        "う" => Some("ウ"),
        "ぇ" => Some("ェ"),
        "え" => Some("エ"),
        "ぉ" => Some("ォ"),
        "お" => Some("オ"),
        "か" => Some("カ"),
        "が" => Some("ガ"),
        "き" => Some("キ"),
        "ぎ" => Some("ギ"),
        "く" => Some("ク"),
        "ぐ" => Some("グ"),
        "け" => Some("ケ"),
        "げ" => Some("ゲ"),
        "こ" => Some("コ"),
        "ご" => Some("ゴ"),
        "さ" => Some("サ"),
        "ざ" => Some("ザ"),
        "し" => Some("シ"),
        "じ" => Some("ジ"),
        "す" => Some("ス"),
        "ず" => Some("ズ"),
        "せ" => Some("セ"),
        "ぜ" => Some("ゼ"),
        "そ" => Some("ソ"),
        "ぞ" => Some("ゾ"),
        "た" => Some("タ"),
        "だ" => Some("ダ"),
        "ち" => Some("チ"),
        "ぢ" => Some("ヂ"),
        "っ" => Some("ッ"),
        "つ" => Some("ツ"),
        "づ" => Some("ヅ"),
        "て" => Some("テ"),
        "で" => Some("デ"),
        "と" => Some("ト"),
        "ど" => Some("ド"),
        "な" => Some("ナ"),
        "に" => Some("ニ"),
        "ぬ" => Some("ヌ"),
        "ね" => Some("ネ"),
        "の" => Some("ノ"),
        "は" => Some("ハ"),
        "ば" => Some("バ"),
        "ぱ" => Some("パ"),
        "ひ" => Some("ヒ"),
        "び" => Some("ビ"),
        "ぴ" => Some("ピ"),
        "ふ" => Some("フ"),
        "ぶ" => Some("ブ"),
        "ぷ" => Some("プ"),
        "へ" => Some("ヘ"),
        "べ" => Some("ベ"),
        "ぺ" => Some("ペ"),
        "ほ" => Some("ホ"),
        "ぼ" => Some("ボ"),
        "ぽ" => Some("ポ"),
        "ま" => Some("マ"),
        "み" => Some("ミ"),
        "む" => Some("ム"),
        "め" => Some("メ"),
        "も" => Some("モ"),
        "ゃ" => Some("ャ"),
        "や" => Some("ヤ"),
        "ゅ" => Some("ュ"),
        "ゆ" => Some("ユ"),
        "ょ" => Some("ョ"),
        "よ" => Some("ヨ"),
        "ら" => Some("ラ"),
        "り" => Some("リ"),
        "る" => Some("ル"),
        "れ" => Some("レ"),
        "ろ" => Some("ロ"),
        "ゎ" => Some("ヮ"),
        "わ" => Some("ワ"),
        "ゐ" => Some("ヰ"),
        "ゑ" => Some("ヱ"),
        "を" => Some("ヲ"),
        "ん" => Some("ン"),
        "ゔ" => Some("ヴ"),
        "ゕ" => Some("ヵ"),
        "ゖ" => Some("ヶ"),
        "わ゛" => Some("ヷ"),
        "ゐ゛" => Some("ヸ"),
        "ゑ゛" => Some("ヹ"),
        "を゛" => Some("ヺ"),
        _ => None()
    }
}
