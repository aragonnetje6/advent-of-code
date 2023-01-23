use std::str::FromStr;

#[derive(Clone, Debug)]
struct Bingo {
    board: [[u8; 5]; 5],
    checked: [[bool; 5]; 5],
}

impl FromStr for Bingo {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = s.lines().filter(|line| line.len() > 2).flat_map(|line| {
            line.split(' ')
                .filter_map(|item| item.trim().parse::<u8>().ok())
                .collect::<Vec<u8>>()
        });
        let mut board: [[u8; 5]; 5] = [[0; 5]; 5];
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                board[i][j] = data.next().unwrap();
            }
        }
        Ok(Self {
            board,
            checked: [[false; 5]; 5],
        })
    }
}

impl Bingo {
    fn check(&mut self, x: u8) {
        for i in 0..self.board.len() {
            for j in 0..self.board[0].len() {
                if self.board[i][j] == x {
                    self.checked[i][j] = true;
                }
            }
        }
    }

    fn has_won(&self) -> bool {
        for i in 0..self.board.len() {
            if self.checked[i].iter().all(|x| *x) {
                return true;
            }
            if self.checked.iter().all(|line| line[i]) {
                return true;
            }
        }
        false
    }

    fn score(&self) -> u32 {
        let mut total = 0;
        for i in 0..self.board.len() {
            for j in 0..self.board[0].len() {
                if !self.checked[i][j] {
                    total += u32::from(self.board[i][j]);
                }
            }
        }
        total
    }
}

fn parse_input(input: &str) -> (Vec<u8>, Vec<Bingo>) {
    let mut blocks = input.split("\n\n");
    let nums: Vec<u8> = blocks
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect();
    let bingos: Vec<Bingo> = blocks.map(|block| block.parse().unwrap()).collect();
    (nums, bingos)
}

pub fn part1(input: &str) -> String {
    let (nums, mut bingos) = parse_input(input);
    for num in nums {
        bingos.iter_mut().for_each(|bingo| bingo.check(num));
        if let Some(winner) = bingos.iter().find(|x| x.has_won()) {
            return (winner.score() * u32::from(num)).to_string();
        }
    }
    unreachable!()
}

pub fn part2(input: &str) -> String {
    let (nums, mut bingos) = parse_input(input);
    let mut nums_iter = nums.iter();
    while bingos.iter().filter(|x| !x.has_won()).count() > 1 {
        let num = *nums_iter.next().unwrap();
        bingos.iter_mut().for_each(|bingo| bingo.check(num));
    }
    let mut last = bingos
        .iter()
        .find(|bingo| !bingo.has_won())
        .unwrap()
        .clone();
    for num in nums_iter {
        last.check(*num);
        if last.has_won() {
            return (last.score() * u32::from(*num)).to_string();
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

 2 22 18 10 14
 0 11  8 16 21
12 13 23 15 17
 3  6 26  9 24
 7  5 20 19  4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), "4512");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), "1924");
    }
}
