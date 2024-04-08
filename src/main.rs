use pathfinding::prelude::*;

type LevelSketch = [&'static str; 7];

#[rustfmt::skip]
const LEVEL_1: LevelSketch = [
                                ":...:.......:...",
                                ".........:....:.",
                                ".::.......::....",
                                "..:.....i...:...",
                                "...:..:....:..:.",
                                "...::.:.......:.",
                                "...:.......i..:.",
                             ];

mod board;
use board::*;

// ---------------------------
//
//     m  a  i  n
//
// ---------------------------
fn main() {
    let l1 = LEVEL_1.to_vec();
    let b1 = Board::new(l1, true);

    println!("{:?}", b1.moveable_directions); // not smart enuff !
    b1.print(None);

    let start = Pos(2, 0); // print cannot handle negative direction !
    let goal = Pos(13, 6);

    let result = bfs(
        &start,
        |p| {
            b1.successors(p)
                .iter()
                .map(|successor| successor.pos)
                .collect::<Vec<_>>()
        },
        |p| *p == goal,
    );
    println!("{:?}", result);
    b1.print(result);
}
