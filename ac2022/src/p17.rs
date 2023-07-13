use itertools::Itertools;
use std::fmt::Display;

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

const PADDING: [[Rock; 7]; 3] = [[Rock::Air; 7]; 3];

const BLOCK1: [[Rock; 7]; 1] = [[
    Rock::Air,
    Rock::Air,
    Rock::MovingRock,
    Rock::MovingRock,
    Rock::MovingRock,
    Rock::MovingRock,
    Rock::Air,
]];

const BLOCK2: [[Rock; 7]; 3] = [
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
];

const BLOCK3: [[Rock; 7]; 3] = [
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
    [
        Rock::Air,
        Rock::Air,
        Rock::MovingRock,
        Rock::MovingRock,
        Rock::MovingRock,
        Rock::Air,
        Rock::Air,
    ],
];

const BLOCK4: [[Rock; 7]; 4] = [
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
];

const BLOCK5: [[Rock; 7]; 2] = [
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
];

#[derive(Debug, Clone)]
struct Board {
    board: Vec<[Rock; 7]>,
    truncated_height: usize,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.board
                .iter()
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
                + "\n---------\n"
        )
    }
}

impl Board {
    const fn new() -> Self {
        Self {
            board: vec![],
            truncated_height: 0,
        }
    }

    fn fall(&mut self) -> bool {
        let depths = self.depths();
        if self.board.len() < depths.end + 1 {
            return false;
        }
        let old_board = self.board.clone();
        for i in depths.rev() {
            for j in 0..7 {
                if let Some((new_upper, new_lower)) =
                    check_move(self.board[i][j], self.board[i + 1][j])
                {
                    self.board[i][j] = new_upper;
                    self.board[i + 1][j] = new_lower;
                } else {
                    self.board = old_board;
                    return false;
                }
            }
        }
        self.trim();
        true
    }

    fn depths(&self) -> std::ops::Range<usize> {
        let lower_bound = self
            .board
            .iter()
            .take_while(|x| !x.contains(&Rock::MovingRock))
            .count();
        let upper_bound = self
            .board
            .iter()
            .skip(lower_bound)
            .take_while(|x| x.contains(&Rock::MovingRock))
            .count();
        lower_bound..lower_bound + upper_bound
    }

    fn move_left(&mut self) {
        let old_board = self.board.clone();
        for i in self.depths() {
            for j in 0..6 {
                if let Some((new_src, new_dest)) =
                    check_move(self.board[i][j + 1], self.board[i][j])
                {
                    self.board[i][j + 1] = new_src;
                    self.board[i][j] = new_dest;
                } else {
                    self.board = old_board;
                    return;
                }
            }
        }
    }

    fn move_right(&mut self) {
        let old_board = self.board.clone();
        for i in self.depths() {
            for j in (0..6).rev() {
                if let Some((new_src, new_dest)) =
                    check_move(self.board[i][j], self.board[i][j + 1])
                {
                    self.board[i][j] = new_src;
                    self.board[i][j + 1] = new_dest;
                } else {
                    self.board = old_board;
                    return;
                }
            }
        }
    }

    fn simulate(&mut self, moves: &[Move], count: usize) {
        let mut moves_loop = std::iter::repeat(moves.iter()).flatten();
        let mut blocks: Vec<Vec<[Rock; 7]>> = vec![
            BLOCK1.to_vec(),
            BLOCK2.to_vec(),
            BLOCK3.to_vec(),
            BLOCK4.to_vec(),
            BLOCK5.to_vec(),
        ];
        blocks.iter_mut().for_each(|x| x.extend(PADDING));
        let blocks_loop = std::iter::repeat(blocks.iter()).flatten();
        for block in blocks_loop.take(count) {
            let mut new_board = block.clone();
            new_board.extend(&self.board);
            self.board = new_board;
            loop {
                // println!("{self}");
                let next_move = moves_loop.next().expect("infinite iterator");
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
        }
    }

    fn trim(&mut self) {
        let mut temp = vec![];
        std::mem::swap(&mut self.board, &mut temp);
        self.board = temp
            .into_iter()
            .skip_while(|r| !r.contains(&Rock::SettledRock) && !r.contains(&Rock::MovingRock))
            .collect();
    }

    fn height(&self) -> usize {
        self.board.len() + self.truncated_height
    }

    fn solidify(&mut self) {
        for i in self.depths() {
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
        (Rock::MovingRock, Rock::Air) => Some((Rock::Air, Rock::MovingRock)),
        (Rock::MovingRock, Rock::SettledRock) | (_, Rock::MovingRock) => None,
        (Rock::Air | Rock::SettledRock, Rock::Air | Rock::SettledRock) => Some((src, dest)),
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
    todo!("{}", input)
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
        assert_eq!(part2(DATA1), "1707");
    }
}
