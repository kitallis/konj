extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

struct KanaGeminates<'a> {
    preferred: [(&'a str, &'a str); 9],
    rest: [(&'a str, &'a str); 3],
}

static GEMINATES_AND_KANA: KanaGeminates<'static> = KanaGeminates {
    preferred: [
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

    // -------------------------
    // --- geminates to kana ---
    // -------------------------
    let mut geminates_to_kana = phf_codegen::Map::new();
    for &(key, value) in GEMINATES_AND_KANA.preferred.iter() {
        geminates_to_kana.entry(key, &format!("\"{}\"", value));
    }
    for &(key, value) in GEMINATES_AND_KANA.rest.iter() {
        geminates_to_kana.entry(key, &format!("\"{}\"", value));
    }

    // -------------------------
    // --- kana to geminates ---
    // -------------------------
    let mut kana_to_geminates = phf_codegen::Map::new();
    for &(key, value) in GEMINATES_AND_KANA.preferred.iter() {
        kana_to_geminates.entry(value, &format!("\"{}\"", key));
    }

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
