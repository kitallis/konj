extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

struct KanaGeminates<'a> {
    favored: [(&'a str, &'a str); 9],
    rest: [(&'a str, &'a str); 3],
}

const GEMINATES_AND_KANA: KanaGeminates<'static> = KanaGeminates {
    favored: [
        ("kk", "っk"),
        ("tt", "っt"),
        ("cc", "っc"),
        ("ss", "っs"),
        ("pp", "っp"),
        ("mm", "んm"),
        ("mt", "んt"),
        ("mb", "んb"),
        ("mp", "んp"),
    ],

    rest: [("nt", "んt"), ("np", "んp"), ("nb", "んb")],
};

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("data.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());
    let mut geminates_to_kana = phf_codegen::Map::new();
    let mut kana_to_geminates = phf_codegen::Map::new();

    // -------------------------
    // --- geminates to kana ---
    // -------------------------
    gen_map(
        &mut geminates_to_kana,
        GEMINATES_AND_KANA.favored.to_vec(),
        false,
    );
    gen_map(
        &mut geminates_to_kana,
        GEMINATES_AND_KANA.rest.to_vec(),
        false,
    );

    // -------------------------
    // --- kana to geminates ---
    // -------------------------
    gen_map(
        &mut kana_to_geminates,
        GEMINATES_AND_KANA.rest.to_vec(),
        true,
    );

    writeln!(
        &mut file,
        "pub static GEMINATES_TO_KANA: phf::Map<&'static str, &'static str> = \n{};\n",
        geminates_to_kana.build()
    )
    .unwrap();

    writeln!(
        &mut file,
        "pub static KANA_TO_GEMINATES: phf::Map<&'static str, &'static str> = \n{};\n",
        kana_to_geminates.build()
    )
    .unwrap();
}

// Populate the phf_codegen::Map with the vector of tuples in `from`.
// Vec<(a,b)> -> Map<a,b>
//
// We can also generate this map in an inverted way, such that,
// Vec<(a,b)> -> Map<b,a>
fn gen_map<'a>(of: &mut phf_codegen::Map<&'a str>, from: Vec<(&'a str, &'a str)>, invert: bool) {
    for &(key, value) in from.iter() {
        if invert {
            of.entry(value, &format!("\"{}\"", key));
        } else {
            of.entry(key, &format!("\"{}\"", value));
        }
    }
}
