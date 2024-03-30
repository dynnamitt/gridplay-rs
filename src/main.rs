type LevelSketch = [&'static str; 7];

#[rustfmt::skip]
const LEVEL_1: LevelSketch = [
                                ":...:.:.:...",
                                "...:.:....:.",
                                ".::..:::....",
                                "..:.i...:...",
                                "...:..::..:.",
                                "...::.:...:.",
                                "...:...i..:.",
                             ];

mod board;

// ---------------------------
//
//     m  a  i  n
//
// ---------------------------
fn main() {
    let l1 = LEVEL_1.to_vec();
    let b1 = board::Board::new(l1);
    b1.print(None);
}
