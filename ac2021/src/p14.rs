use std::collections::HashMap;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline};
use nom::multi::{count, separated_list1};
use nom::IResult;
use rayon::prelude::*;

fn rule(input: &str) -> IResult<&str, ([char; 2], char)> {
    let (input, pair) = alpha1(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, outcome) = alpha1(input)?;
    Ok((
        input,
        (
            [pair.chars().next().unwrap(), pair.chars().nth(1).unwrap()],
            outcome.chars().next().unwrap(),
        ),
    ))
}

fn rules(input: &str) -> IResult<&str, Vec<([char; 2], char)>> {
    separated_list1(newline, rule)(input)
}

fn parse_input(input: &str) -> IResult<&str, (&str, HashMap<[char; 2], char>)> {
    let (input, polymer) = alpha1(input)?;
    let (input, _) = count(newline, 2)(input)?;
    let (input, mut rules) = rules(input)?;
    Ok((input, (polymer, rules.drain(..).collect())))
}

fn apply_rules(polymer: &[char], rules: &HashMap<[char; 2], Vec<char>>) -> Vec<char> {
    polymer
        .windows(2)
        .par_bridge()
        .flat_map(|chars| match rules.get(chars) {
            None => vec![chars[0]],
            Some(x) => x.clone(),
        })
        .chain([*polymer.last().unwrap()])
        .collect()
}

fn frequencies(polymer: &[char]) -> HashMap<&char, usize> {
    let mut freq = HashMap::new();
    for c in polymer {
        match freq.get_mut(&c) {
            None => {
                freq.insert(c, 0);
            }
            Some(x) => *x += 1,
        }
    }
    freq
}

fn minmax(polymer: &[char]) -> usize {
    let freq = frequencies(polymer);
    freq.iter().max_by_key(|(_, f)| **f).unwrap().1
        - freq.iter().min_by_key(|(_, f)| **f).unwrap().1
}

fn transform_rules(rules: &HashMap<[char; 2], char>) -> HashMap<[char; 2], Vec<char>> {
    rules
        .iter()
        .map(|(name, res)| (*name, vec![name[0], *res]))
        .collect()
}

pub fn part1(input: &str) -> usize {
    let (_, (polymer, rules)) = parse_input(input).unwrap();
    let rules = transform_rules(&rules);
    let mut polymer: Vec<char> = polymer.chars().collect();
    for _ in 0..10 {
        polymer = apply_rules(&polymer, &rules);
    }
    minmax(&polymer)
}

pub fn part2(input: &str) -> usize {
    let (_, (polymer, rules)) = parse_input(input).unwrap();
    let rules = transform_rules(&rules);
    let mut polymer: Vec<char> = polymer.chars().collect();
    for i in 0..40 {
        println!("{i}");
        polymer = apply_rules(&polymer, &rules);
    }
    minmax(&polymer)
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";
    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 1588);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 2_188_189_693_529);
    }
}
