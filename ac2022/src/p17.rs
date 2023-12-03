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
struct SaveState {
    board: Vec<[Rock; 7]>,
    move_index: usize,
    block_index: usize,
    total_blocks: usize,
    height: usize,
}

impl PartialEq for SaveState {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
            && self.move_index == other.move_index
            && self.block_index == other.block_index
    }
}

type EnumeratedEndless<'a, T> =
    std::iter::Flatten<std::iter::Repeat<std::iter::Enumerate<std::slice::Iter<'a, T>>>>;

#[derive(Debug, Clone)]
struct Board<'a> {
    board: Vec<[Rock; 7]>,
    truncated_height: usize,
    block_range: Range<usize>,
    savestates: Vec<SaveState>,
    moves_loop: EnumeratedEndless<'a, Move>,
    blocks_loop: EnumeratedEndless<'static, [[Rock; 7]; 7]>,
}

impl<'a> Display for Board<'a> {
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

impl<'a> Board<'a> {
    fn new(moves: &'a [Move]) -> Self {
        Self {
            board: vec![],
            truncated_height: 0,
            block_range: 0..0,
            savestates: vec![],
            moves_loop: std::iter::repeat(moves.iter().enumerate()).flatten(),
            blocks_loop: std::iter::repeat(BLOCKS.iter().enumerate()).flatten(),
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

    fn simulate(&mut self, count: usize) {
        for _ in 0..count {
            let (_, block) = self.blocks_loop.next().expect("infinite iterator");
            self.board.extend(block);
            self.block_range = self.board.len() - 4
                ..self.board.len()
                    - (4 - block
                        .iter()
                        .filter(|x| x.contains(&Rock::MovingRock))
                        .count());
            loop {
                let next_move = self.moves_loop.next().expect("infinite iterator").1;
                match next_move {
                    Move::Left => self.move_left(),
                    Move::Right => self.move_right(),
                }
                if !self.fall() {
                    break;
                }
            }
            self.solidify();
            self.trim();
        }
    }

    fn simulate_until_cycle(&mut self, count: usize) {
        for i in 1..=count {
            let (block_index, block) = self.blocks_loop.next().expect("infinite iterator");
            self.board.extend(block);
            self.block_range = self.board.len() - 4
                ..self.board.len()
                    - (4 - block
                        .iter()
                        .filter(|x| x.contains(&Rock::MovingRock))
                        .count());
            let move_index = loop {
                let (move_i, next_move) = self.moves_loop.next().expect("infinite iterator");
                match next_move {
                    Move::Left => self.move_left(),
                    Move::Right => self.move_right(),
                }
                if !self.fall() {
                    break move_i;
                }
            };
            self.solidify();
            if let Some((height_diff, block_diff)) = self.trim_and_save(move_index, block_index, i)
            {
                self.truncated_height += (count - i) / block_diff * height_diff;
                self.simulate((count - i) % block_diff);
                break;
            }
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

    fn trim_and_save(
        &mut self,
        move_index: usize,
        block_index: usize,
        total_blocks: usize,
    ) -> Option<(usize, usize)> {
        self.trim();
        if let Some(i) = self
            .block_range
            .clone()
            .find(|ri| !self.board[*ri].contains(&Rock::Air))
        {
            self.truncated_height += i + 1;
            self.board = self.board[i + 1..].to_vec();
            let savestate = SaveState {
                board: self.board.clone(),
                move_index,
                block_index,
                total_blocks,
                height: self.height(),
            };
            if let Some(other) = self.savestates.iter().rev().find(|x| x == &&savestate) {
                println!(
                    "Cycle found, current height: {}, cycle height: {}, cycle blocks: {}",
                    savestate.height,
                    savestate.height - other.height,
                    savestate.total_blocks - other.total_blocks
                );
                return Some((
                    savestate.height - other.height,
                    savestate.total_blocks - other.total_blocks,
                ));
            }
            self.savestates.push(savestate);
        }
        None
    }

    fn height(&mut self) -> usize {
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
    let mut board = Board::new(&data);
    board.simulate_until_cycle(2022);
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
    let mut board = Board::new(&data);
    board.simulate_until_cycle(1_000_000_000_000);
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
