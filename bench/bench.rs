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

    println!("{:?}", start.elapsed());
}

// |----------------------------------------------------+------------------+--------------------|
// | bench                                              | time in seconds  | -                  |
// |----------------------------------------------------+------------------+--------------------|
// |                                                    | for iters: 1000  | for iters: 10_000  |
// |----------------------------------------------------+------------------+--------------------|
// | compile-time statically generated datga            | 6750.33, 6725.70 | 66846.28, 66637.71 |
// | lazy-initialize of seed data (but once at startup) | 6584.27, 6533.34 | 65253.52, 65363.71 |
// |----------------------------------------------------+------------------+--------------------|
