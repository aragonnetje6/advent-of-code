type Recommendation = (Move, Move);
type Recommendation2 = (Move, Outcome);

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

pub fn part1(input: &str) -> String {
    let data = process_input(input);
    data.iter()
        .map(|recommend| get_move_score(*recommend) + get_outcome(*recommend) as u32)
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let data = process_input_2(input);
    data.iter()
        .map(|x| translate(*x))
        .map(|recommend| -> u32 { get_move_score(recommend) + get_outcome(recommend) as u32 })
        .sum::<u32>()
        .to_string()
}

const fn translate(rec: Recommendation2) -> Recommendation {
    match rec.1 {
        Outcome::Win => (rec.0, defeats(rec.0)),
        Outcome::Draw => (rec.0, rec.0),
        Outcome::Loss => (rec.0, defeated_by(rec.0)),
    }
}

const fn get_move_score(rec: Recommendation) -> u32 {
    rec.1 as u32
}

const fn defeats(x: Move) -> Move {
    match x {
        Move::Rock => Move::Paper,
        Move::Paper => Move::Scissors,
        Move::Scissors => Move::Rock,
    }
}

const fn defeated_by(x: Move) -> Move {
    match x {
        Move::Rock => Move::Scissors,
        Move::Paper => Move::Rock,
        Move::Scissors => Move::Paper,
    }
}

fn get_outcome(rec: Recommendation) -> Outcome {
    if rec.0 == rec.1 {
        Outcome::Draw
    } else if rec.1 == defeats(rec.0) {
        Outcome::Win
    } else {
        Outcome::Loss
    }
}

fn process_input(input: &str) -> Vec<Recommendation> {
    input
        .lines()
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
        assert_eq!(part1("A Y\nB X\nC Z\n"), "15");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("A Y\nB X\nC Z\n"), "12");
    }
}
