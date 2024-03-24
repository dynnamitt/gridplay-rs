type LevelSketch = [&'static str; 4];

#[rustfmt::skip]

const LEVEL_1: LevelSketch = [
                                "s.........",
                                ".e..bT....",
                                "..b...e...",
                                "...T....x."];

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub i16, pub i16);

pub struct Board {
    pub width: u8,
    pub height: u8,
    pub data: Vec<Vec<Option<u8>>>,
    pub allow_diagonal: bool,
}

impl Board {
    fn init(level: LevelSketch) -> Self {
        let height: u8 = level.len() as u8;
        let width: u8 = level[0].len() as u8;
        let d = level
            .iter()
            .map(|s| s.as_bytes().iter().map(|x| Some(*x)).collect())
            .collect();

        Board {
            data: d,
            height,
            width,
            allow_diagonal: false,
        }
    }
}

fn main() {
    let b1 = Board::init(LEVEL_1);

    println!("data:\n{:?}", b1.data);
    // The `vec!` macro can be used to initialize a vector

    // A `Vector` can also be iterated over while the iteration
    // count is enumerated in a separate variable (`i`)
    // for (i, x) in xs.iter().enumerate() {
    //     println!("In position {} we have value {:?}", i, x);
    // }

    // Thanks to `iter_mut`, mutable `Vector`s can also be iterated
    // over in a way that allows modifying each value
    // for x in xs.iter_mut() {
    //     *x *= 3;
    // }
    // println!("Updated vector: {:?}", xs);
}
