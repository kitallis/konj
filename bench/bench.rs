use konj::*;

fn main() {
    let start = std::time::Instant::now();

    for _ in 0..10_000 {
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

// |------------------------------+------------------+--------------------+---|
// | bench                        | iterations       | -                  |   |
// |------------------------------+------------------+--------------------+---|
// |                              | 1000             | 10_000             |   |
// |------------------------------+------------------+--------------------+---|
// | lazy initialize of seed data | 6750.33, 6725.70 | 66846.28, 66637.71 |   |
// | compile-time static data     | 6584.27, 6533.34 | 65253.52, 65363.71 |   |
