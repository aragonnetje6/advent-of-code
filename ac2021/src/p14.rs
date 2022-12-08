use std::collections::HashMap;
use std::hash::Hash;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline};
use nom::multi::{count, separated_list1};
use nom::IResult;

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
        .flat_map(|chars| match rules.get(chars) {
            None => vec![chars[0]],
            Some(x) => x.clone(),
        })
        .chain([*polymer.last().unwrap()])
        .collect()
}

fn frequencies(polymer: &[char]) -> HashMap<char, usize> {
    let mut freq = HashMap::new();
    for c in polymer {
        match freq.get_mut(c) {
            None => {
                freq.insert(*c, 0);
            }
            Some(x) => *x += 1,
        }
    }
    freq
}

fn minmax<T: Hash>(freq: &HashMap<T, usize>) -> usize {
    freq.iter().max_by_key(|(_, f)| **f).unwrap().1
        - freq.iter().min_by_key(|(_, f)| **f).unwrap().1
}

fn transform_rules(rules: &HashMap<[char; 2], char>) -> HashMap<[char; 2], Vec<char>> {
    rules
        .iter()
        .map(|(name, res)| (*name, vec![name[0], *res]))
        .collect()
}

fn transform_rules2(rules: &HashMap<[char; 2], char>) -> HashMap<[char; 2], Vec<char>> {
    rules
        .iter()
        .map(|(name, res)| (*name, vec![name[0], *res, name[1]]))
        .collect()
}

pub fn part1(input: &str) -> usize {
    let (_, (polymer, rules)) = parse_input(input).unwrap();
    let rules = transform_rules(&rules);
    let mut polymer: Vec<char> = polymer.chars().collect();
    for _ in 0..10 {
        polymer = apply_rules(&polymer, &rules);
    }
    minmax(&frequencies(&polymer))
}

fn transform_rules3(rules: &HashMap<[char; 2], Vec<char>>) -> HashMap<[char; 2], Vec<[char; 2]>> {
    rules
        .iter()
        .map(|(chars, res)| (*chars, res.windows(2).map(|iter| [iter[0], iter[1]]).collect()))
        .collect()
}

fn statistical_solve(
    polymer: &[char],
    rules: &HashMap<[char; 2], Vec<[char; 2]>>,
    count: u8,
) -> HashMap<char, usize> {
    let mut set_counts: HashMap<[char; 2], usize> = HashMap::new();
    for window in polymer.windows(2).map(|iter| [iter[0], iter[1]]) {
        add_to_map(&mut set_counts, window, 1);
    }
    for _ in 0..count {
        let mut new_set_counts: HashMap<[char; 2], usize> =
            rules.values().flatten().map(|x| (*x, 0)).collect();
        for (set, set_count) in &set_counts {
            match rules.get(set) {
                None => add_to_map(&mut new_set_counts, *set, *set_count),
                Some(sets) => {
                    for set in sets {
                        add_to_map(&mut new_set_counts, *set, *set_count);
                    }
                }
            }
        }
        set_counts = new_set_counts;
    }
    let mut frequencies = HashMap::new();
    set_counts
        .iter()
        .flat_map(|(set, count)| [(set[0], *count), (set[1], *count)])
        .for_each(|(c, count)| add_to_map(&mut frequencies, c, count));
    frequencies
}

fn add_to_map<T: Hash + Eq>(map: &mut HashMap<T, usize>, index: T, val: usize) {
    match map.get_mut(&index) {
        None => {
            map.insert(index, val);
        }
        Some(x) => {
            *x += val;
        }
    }
}

pub fn part2(input: &str) -> usize {
    let (_, (polymer, rules)) = parse_input(input).unwrap();
    let rules = transform_rules3(&transform_rules2(&rules));
    let polymer: Vec<char> = polymer.chars().collect();
    let frequencies = statistical_solve(&polymer, &rules, 40);
    minmax(&frequencies) / 2 + 1
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
