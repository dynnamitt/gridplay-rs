use itertools::Itertools;
use std::str;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub i16, pub i16);

#[derive(Debug)]
pub struct Board {
    pub width: u8,
    pub height: u8,
    pub data: Vec<Vec<u8>>,
    pub moveable_directions: Vec<(i16, i16)>,
}

impl Board {
    pub fn new(level: Vec<&str>, move_diagonal: bool) -> Self {
        let height: u8 = level.len() as u8;
        assert!(height > 0);
        let width: u8 = level[0].len() as u8;
        let d = level
            .into_iter()
            .map(|s| s.as_bytes().to_vec())
            .collect_vec();
        Board {
            data: d,
            height,
            width,
            moveable_directions: surroundings(move_diagonal),
        }
    }
    pub fn print(&self, mover: Option<Vec<Pos>>) {
        for (y, board_line) in self.data.iter().enumerate() {
            let board_line = str::from_utf8(board_line).unwrap();

            if let Some(ref ps) = mover {
                let ps = ps.iter().filter(|Pos(_mx, my)| *my == y as i16);
                let mut prev_mx: usize = 0;
                for Pos(mx, _my) in ps {
                    // lets hope it is sorted..
                    let mx = *mx as usize;
                    print!("{}", &board_line[prev_mx..mx]);
                    print!("X");
                    prev_mx = mx + 1;
                }
                if prev_mx < self.width as usize {
                    println!("{}", &board_line[prev_mx..self.width as usize]);
                } else {
                    println!();
                }
            } else {
                println!("{}", &board_line);
            }
        }
    }

    pub fn successors(&self, pos: &Pos) -> Vec<Successor> {
        let frame: Vec<_> = self
            .moveable_directions
            .iter()
            .map(|(x, y)| (pos.0 + x, pos.1 + y))
            .collect();

        // println!("----> {:?}", frame);

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

fn surroundings(move_diagonal: bool) -> Vec<(i16, i16)> {
    match move_diagonal {
        true =>
        // 9 pairs
        // minus s tart-pos, discard
        {
            (-1i16..=1)
                .cartesian_product(-1i16..=1)
                .filter(|pair| *pair != (0, 0))
                .collect()
        }
        false => [(0, -1), (-1, 0), (1, 0), (1, 1)].to_vec(),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct Successor {
    pub pos: Pos,
    pub cost: u32,
}

// ---------------------------
//
//     t e s t
//
// ---------------------------

#[cfg(test)]
mod tests {
    // crate::board::*;
    use crate::board::Board;
    use crate::board::Pos;

    #[rustfmt::skip]
    const LEVEL_0:  [&'static str; 4] = [
                                "....",
                                "...:",
                                ".::.",
                                "..:.",
                             ];
    #[test]
    fn board_size() {
        let b = Board::new(LEVEL_0.to_vec(), true);
        assert_eq!(b.width, 4);
        assert_eq!(b.height, 4);
    }
    #[test]
    fn succ_no_diag() {
        let b = Board::new(LEVEL_0.to_vec(), false);

        assert_eq!(4, b.moveable_directions.len());

        let s00 = b.successors(&Pos(0, 0));
        assert_eq!(2, s00.len());
    }

    #[test]
    fn succ_corners() {
        let b = Board::new(LEVEL_0.to_vec(), true);
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
        let b = Board::new(LEVEL_0.to_vec(), true);
        let p1_1 = b.successors(&Pos(1, 1));
        // println!("{:?}", p1_1);
        assert_eq!(p1_1.len(), 8, "frame col cnt");
        assert_eq!(p1_1[0].pos, Pos(0, 0));
        assert_eq!(p1_1[1].pos, Pos(0, 1));
        assert_eq!(p1_1[2].pos, Pos(0, 2));
    }
}
