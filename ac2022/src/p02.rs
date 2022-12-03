type Recommendation = (Move, Move);
type Recommendation2 = (Move, Outcome);

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

pub fn part1(input: &str) -> u32 {
    let data = process_input(input);
    data.iter()
        .map(|recommend| get_move_score(recommend) + get_outcome_score(&get_outcome(recommend)))
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let data = process_input_2(input);
    data.iter()
        .map(translate)
        .map(|recommend| get_move_score(&recommend) + get_outcome_score(&get_outcome(&recommend)))
        .sum()
}

fn translate(rec: &Recommendation2) -> Recommendation {
    match rec.1 {
        Outcome::Win => (rec.0, defeats(&rec.0)),
        Outcome::Draw => (rec.0, rec.0),
        Outcome::Loss => (rec.0, defeated_by(&rec.0)),
    }
}

const fn get_move_score(rec: &Recommendation) -> u32 {
    match rec.1 {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

const fn defeats(x: &Move) -> Move {
    match x {
        Move::Rock => Move::Paper,
        Move::Paper => Move::Scissors,
        Move::Scissors => Move::Rock,
    }
}

const fn defeated_by(x: &Move) -> Move {
    match x {
        Move::Rock => Move::Scissors,
        Move::Paper => Move::Rock,
        Move::Scissors => Move::Paper,
    }
}

fn get_outcome(rec: &Recommendation) -> Outcome {
    if rec.0 == rec.1 {
        Outcome::Draw
    } else if rec.1 == defeats(&rec.0) {
        Outcome::Win
    } else {
        Outcome::Loss
    }
}

const fn get_outcome_score(outcome: &Outcome) -> u32 {
    match outcome {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Loss => 0,
    }
}

fn process_input(input: &str) -> Vec<Recommendation> {
    input
        .split('\n')
        .map(|line| {
            line.split(' ')
                .filter_map(|char| match char {
                    "A" | "X" => Some(Move::Rock),
                    "B" | "Y" => Some(Move::Paper),
                    "C" | "Z" => Some(Move::Scissors),
                    _ => None,
                })
                .collect::<Vec<Move>>()
        })
        .filter(|x| !x.is_empty())
        .map(|v| (v[0], v[1]))
        .collect::<Vec<Recommendation>>()
}

fn process_input_2(input: &str) -> Vec<Recommendation2> {
    process_input(input)
        .iter()
        .map(|(mov1, mov2)| {
            (
                *mov1,
                match mov2 {
                    Move::Rock => Outcome::Loss,
                    Move::Paper => Outcome::Draw,
                    Move::Scissors => Outcome::Win,
                },
            )
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            get_outcome_score(&get_outcome(&(Move::Rock, Move::Paper))),
            6
        );
        assert_eq!(
            get_outcome_score(&get_outcome(&(Move::Paper, Move::Rock))),
            0
        );
        assert_eq!(
            get_outcome_score(&get_outcome(&(Move::Scissors, Move::Scissors))),
            3
        );
        assert_eq!(get_move_score(&(Move::Rock, Move::Paper)), 2);
        assert_eq!(get_move_score(&(Move::Paper, Move::Rock)), 1);
        assert_eq!(get_move_score(&(Move::Scissors, Move::Scissors)), 3);
        assert_eq!(part1("A Y\nB X\nC Z\n"), 15);
        dbg!(process_input("A X\n"));
        assert_eq!(part1("A X\n"), 3 + 1);
        assert_eq!(part1("A Y\n"), 6 + 2);
        assert_eq!(part1("A Z\n"), 0 + 3);
        assert_eq!(part1("B X\n"), 0 + 1);
        assert_eq!(part1("B Y\n"), 3 + 2);
        assert_eq!(part1("B Z\n"), 6 + 3);
        assert_eq!(part1("C X\n"), 6 + 1);
        assert_eq!(part1("C Y\n"), 0 + 2);
        assert_eq!(part1("C Z\n"), 3 + 3);
        assert_eq!(
            part1("A X\nA Y\nA Z\nB X\nB Y\nB Z\nC X\nC Y\nC Z\n"),
            4 + 8 + 3 + 1 + 5 + 9 + 7 + 2 + 6
        );
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("A Y\nB X\nC Z\n"), 12);
    }
}
