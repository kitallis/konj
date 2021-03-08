use konj::*;

fn main() {
    let start = std::time::Instant::now();

    for _ in 0..1000 {
        // romaji to kana
        romaji_to_kana("shinkansen");
        romaji_to_kana("kore wa nan desu ka");
        romaji_to_kana("anata no tokei ha doko no tokei desu ka");

        // hiragana to romaji
        hiragana_to_romaji("あなた の とけい は どこ の とけい です か");
        hiragana_to_romaji("きっう");
        hiragana_to_romaji("しんかんせん");
    }

    println!("{:.2}", start.elapsed().as_nanos() as f32 / 1_000_000_f32);
}
