use indicatif::ProgressIterator;
use itertools::Itertools;
use std::{fmt::Display, ops::Range};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rock {
    Air,
    SettledRock,
    MovingRock,
}

const BLOCKS: [[[Rock; 7]; 7]; 5] = [
    [
        [Rock::Air; 7],
        [Rock::Air; 7],
        [Rock::Air; 7],
        [
            Rock::Air,
            Rock::Air,
            Rock::MovingRock,
            Rock::MovingRock,
            Rock::MovingRock,
            Rock::MovingRock,
            Rock::Air,
        ],
        [Rock::Air; 7],
        [Rock::Air; 7],
        [Rock::Air; 7],
    ],
    [
        [Rock::Air; 7],
        [Rock::Air; 7],
        [Rock::Air; 7],
        [
            Rock::Air,
            Rock::Air,
            Rock::Air,
            Rock::MovingRock,
            Rock::Air,
            Rock::Air,
            Rock::Air,
        ],
        [
            Rock::Air,
            Rock::Air,
            Rock::MovingRock,
            Rock::MovingRock,
            Rock::MovingRock,
            Rock::Air,
            Rock::Air,
        ],
        [
            Rock::Air,
            Rock::Air,
            Rock::Air,
            Rock::MovingRock,
            Rock::Air,
            Rock::Air,
            Rock::Air,
        ],
        [Rock::Air; 7],
    ],
    [
        [Rock::Air; 7],
        [Rock::Air; 7],
        [Rock::Air; 7],
        [
            Rock::Air,
            Rock::Air,
            Rock::MovingRock,
            Rock::MovingRock,
            Rock::MovingRock,
            Rock::Air,
            Rock::Air,
        ],
        [
            Rock::Air,
            Rock::Air,
            Rock::Air,
            Rock::Air,
            Rock::MovingRock,
            Rock::Air,
            Rock::Air,
        ],
        [
            Rock::Air,
            Rock::Air,
            Rock::Air,
            Rock::Air,
            Rock::MovingRock,
            Rock::Air,
            Rock::Air,
        ],
        [Rock::Air; 7],
    ],
    [
        [Rock::Air; 7],
        [Rock::Air; 7],
        [Rock::Air; 7],
        [
            Rock::Air,
            Rock::Air,
            Rock::MovingRock,
            Rock::Air,
            Rock::Air,
            Rock::Air,
            Rock::Air,
        ],
        [
            Rock::Air,
            Rock::Air,
            Rock::MovingRock,
            Rock::Air,
            Rock::Air,
            Rock::Air,
            Rock::Air,
        ],
        [
            Rock::Air,
            Rock::Air,
            Rock::MovingRock,
            Rock::Air,
            Rock::Air,
            Rock::Air,
            Rock::Air,
        ],
        [
            Rock::Air,
            Rock::Air,
            Rock::MovingRock,
            Rock::Air,
            Rock::Air,
            Rock::Air,
            Rock::Air,
        ],
    ],
    [
        [Rock::Air; 7],
        [Rock::Air; 7],
        [Rock::Air; 7],
        [
            Rock::Air,
            Rock::Air,
            Rock::MovingRock,
            Rock::MovingRock,
            Rock::Air,
            Rock::Air,
            Rock::Air,
        ],
        [
            Rock::Air,
            Rock::Air,
            Rock::MovingRock,
            Rock::MovingRock,
            Rock::Air,
            Rock::Air,
            Rock::Air,
        ],
        [Rock::Air; 7],
        [Rock::Air; 7],
    ],
];

#[derive(Debug, Clone)]
struct Board {
    board: Vec<[Rock; 7]>,
    truncated_height: usize,
    block_range: Range<usize>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.board
                .iter()
                .rev()
                .map(|arr| format!(
                    "|{}|",
                    arr.iter()
                        .map(|r| match r {
                            Rock::Air => '.',
                            Rock::SettledRock => '#',
                            Rock::MovingRock => '@',
                        })
                        .collect::<String>()
                ))
                .join("\n")
                + "\n+-------+\n"
        )
    }
}

impl Board {
    const fn new() -> Self {
        Self {
            board: vec![],
            truncated_height: 0,
            block_range: 0..0,
        }
    }

    fn fall(&mut self) -> bool {
        if self.block_range.start == 0 {
            return false;
        }
        if !self
            .block_range
            .clone()
            .all(|i| (0..7).all(|j| check_move(self.board[i][j], self.board[i - 1][j]).is_some()))
        {
            return false;
        }
        for i in self.block_range.clone() {
            for j in 0..7 {
                let (new_upper, new_lower) =
                    check_move(self.board[i][j], self.board[i - 1][j]).expect("just checked");
                self.board[i][j] = new_upper;
                self.board[i - 1][j] = new_lower;
            }
        }
        self.block_range = self.block_range.start - 1..self.block_range.end - 1;
        true
    }

    fn move_left(&mut self) {
        if self
            .block_range
            .clone()
            .any(|i| self.board[i][0] == Rock::MovingRock)
        {
            return;
        }
        if !self
            .block_range
            .clone()
            .all(|i| (0..6).all(|j| check_move(self.board[i][j + 1], self.board[i][j]).is_some()))
        {
            return;
        }
        for i in self.block_range.clone() {
            for j in 0..6 {
                let (new_src, new_dest) =
                    check_move(self.board[i][j + 1], self.board[i][j]).expect("just checked");

                self.board[i][j + 1] = new_src;
                self.board[i][j] = new_dest;
            }
        }
    }

    fn move_right(&mut self) {
        if self
            .block_range
            .clone()
            .any(|i| self.board[i][6] == Rock::MovingRock)
        {
            return;
        }
        if !self
            .block_range
            .clone()
            .all(|i| (0..6).all(|j| check_move(self.board[i][j], self.board[i][j + 1]).is_some()))
        {
            return;
        }
        for i in self.block_range.clone() {
            for j in (0..6).rev() {
                let (new_src, new_dest) =
                    check_move(self.board[i][j], self.board[i][j + 1]).expect("just checked");
                self.board[i][j] = new_src;
                self.board[i][j + 1] = new_dest;
            }
        }
    }

    fn simulate(&mut self, moves: &[Move], count: usize) {
        let mut moves_loop = std::iter::repeat(moves.iter()).flatten();
        let blocks_loop = std::iter::repeat(BLOCKS.iter()).flatten();
        for block in blocks_loop.take(count).progress_count(count as u64) {
            self.board.extend(block);
            self.block_range = self.board.len() - 4
                ..self.board.len()
                    - (4 - block
                        .iter()
                        .filter(|x| x.contains(&Rock::MovingRock))
                        .count());
            loop {
                let next_move = moves_loop.next().expect("infinite iterator");
                // println!("{self}");
                match next_move {
                    Move::Left => self.move_left(),
                    Move::Right => self.move_right(),
                }
                // println!("{self}");
                if !self.fall() {
                    break;
                }
            }
            self.solidify();
            self.trim();
        }
    }

    fn trim(&mut self) {
        let count = self
            .board
            .iter()
            .rev()
            .take_while(|x| !x.contains(&Rock::SettledRock) && !x.contains(&Rock::MovingRock))
            .count();
        self.board.truncate(self.board.len() - count);
    }

    fn height(&mut self) -> usize {
        self.trim();
        self.board.len() + self.truncated_height
    }

    fn solidify(&mut self) {
        for i in self.block_range.clone() {
            for j in 0..7 {
                let x = &mut self.board[i][j];
                if matches!(x, Rock::MovingRock) {
                    *x = Rock::SettledRock;
                }
            }
        }
    }
}

const fn check_move(src: Rock, dest: Rock) -> Option<(Rock, Rock)> {
    match (src, dest) {
        (Rock::MovingRock, Rock::Air | Rock::MovingRock) => Some((Rock::Air, Rock::MovingRock)),
        (Rock::MovingRock, Rock::SettledRock) => None,
        (Rock::Air | Rock::SettledRock, _) => Some((src, dest)),
    }
}

pub fn part1(input: &str) -> String {
    let data: Vec<Move> = input
        .chars()
        .filter(|x| *x != '\n')
        .map(|c| match c {
            '<' => Move::Left,
            '>' => Move::Right,
            _ => panic!("wrong input"),
        })
        .collect();
    let mut board = Board::new();
    board.simulate(&data, 2022);
    board.trim();
    board.height().to_string()
}

pub fn part2(input: &str) -> String {
    let data: Vec<Move> = input
        .chars()
        .filter(|x| *x != '\n')
        .map(|c| match c {
            '<' => Move::Left,
            '>' => Move::Right,
            _ => panic!("wrong input"),
        })
        .collect();
    let mut board = Board::new();
    board.simulate(&data, 1_000_000_000_000);
    board.trim();
    board.height().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), "3068");
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(DATA1), "1_514_285_714_288");
    }
}
