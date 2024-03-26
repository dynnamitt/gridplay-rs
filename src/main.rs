use itertools::Itertools;
use std::str;

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

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub i16, pub i16);

pub struct Board {
    pub width: u8,
    pub height: u8,
    pub data: Vec<Vec<u8>>,
    pub allow_diagonal: bool,
}

impl Board {
    pub fn new(level: LevelSketch) -> Self {
        let height: u8 = level.len() as u8;
        let width: u8 = level[0].len() as u8;
        let d = level
            .into_iter()
            .map(|s| s.as_bytes().to_vec())
            .collect_vec();
        Board {
            data: d,
            height,
            width,
            allow_diagonal: false,
        }
    }
    pub fn print(&self, _mover: Option<u8>) {
        for line in &self.data {
            println!("{}", str::from_utf8(line).unwrap());
        }
    }

    pub fn successors(&self, pos: &Pos) -> Vec<Successor> {
        let grid = (-1i16..=1).cartesian_product(-1i16..=1);

        let frame: Vec<(i16, i16)> = grid
            .filter(|pair| *pair != (0, 0)) // start-pos is not useful, discard
            .map(|(x, y)| (pos.0 + x, pos.1 + y))
            .collect();

        println!("----> {:?}", frame);

        frame
            .into_iter()
            .filter(|(col, row)| self.ok_col(col) && self.ok_row(row))
            .map(|(col, row)| Successor {
                pos: Pos(col, row),
                cost: self.data[row as usize][col as usize] as u32,
            })
            .collect()
    }

    fn ok_col(&self, x: &i16) -> bool {
        *x >= 0 && *x < self.width.into()
    }
    fn ok_row(&self, y: &i16) -> bool {
        *y >= 0 && *y < self.height.into()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct Successor {
    pub pos: Pos,
    pub cost: u32,
}

// ---------------------------
//
//     m  a  i  n
//
// ---------------------------
fn main() {
    let b1 = Board::new(LEVEL_1);
    b1.print(None);
}

// ---------------------------
//
//     t e s t
//
// ---------------------------

#[cfg(test)]
mod tests {

    use crate::{Board, Pos, LEVEL_1};

    #[test]
    fn board_size() {
        let b = Board::new(LEVEL_1);
        assert_eq!(b.width, 10);
        assert_eq!(b.height, 4);
    }
    #[test]
    fn succ_corners() {
        let b = Board::new(LEVEL_1);
        let max_row: i16 = b.height as i16 - 1;
        let max_col: i16 = b.width as i16 - 1;
        let lt = b.successors(&Pos(0, 0));
        let rt = b.successors(&Pos(max_col, 0));
        let lb = b.successors(&Pos(0, max_row));
        let rb = b.successors(&Pos(max_col, max_row));
        assert_eq!(lt.len(), 3, "RightTop");
        assert_eq!(rt.len(), 3, "RightTop");
        assert_eq!(lb.len(), 3, "LeftBottom");
        assert_eq!(rb.len(), 3, "RightBottom");
    }
    #[test]
    fn succ_1_1() {
        let b = Board::new(LEVEL_1);
        let p1_1 = b.successors(&Pos(1, 1));
        assert_eq!(p1_1.len(), 8, "frame col cnt");
        assert_eq!(p1_1[0].pos, Pos(0, 0));
        assert_eq!(p1_1[1].pos, Pos(0, 1));
        assert_eq!(p1_1[3].pos, Pos(0, 2));
    }
}
