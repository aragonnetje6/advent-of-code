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

// fn recursive_apply(
//     chunk: [char; 2],
//     rules: &HashMap<[char; 2], Vec<char>>,
//     count: u8,
// ) -> HashMap<char, usize> {
//     if let Some(expanded) = rules.get(&chunk).cloned() {
//         if count == 1 {
//             expanded.iter().map(|x| (*x, 1)).collect()
//         } else {
//             let freqs = expanded
//                 .array_windows::<2>()
//                 .map(|x| recursive_apply(dbg!(*x), rules, dbg!(count - 1)))
//                 .collect::<Vec<_>>();
//             merge_maps(&freqs)
//         }
//     } else {
//         chunk.iter().map(|x| (*x, 1)).collect()
//     }
// }

// fn merge_maps<T: Eq + Hash + Copy, N: AddAssign + Copy>(
//     frequencies: &[HashMap<T, N>],
// ) -> HashMap<T, N> {
//     let mut result = HashMap::new();
//     for map in frequencies.iter() {
//         map.iter().for_each(|(c, i)| match result.get_mut(c) {
//             None => {
//                 result.insert(*c, *i);
//             }
//             Some(x) => *x += *i,
//         });
//     }
//     result
// }

fn merge_apply(
    polymer: &[char],
    rules: &HashMap<[char; 2], Vec<char>>,
    count: u8,
    out: &mut HashMap<char, usize>,
) {
    if count > 1 {
        if count > 20 {
            println!("{count}");
        }
        polymer
            .array_windows::<2>()
            .for_each(|chunk| match rules.get(chunk) {
                None => match out.get_mut(&chunk[0]) {
                    None => {
                        out.insert(chunk[0], 1);
                    }
                    Some(x) => *x += 1,
                },
                Some(expanded) => merge_apply(expanded, rules, count - 1, out),
            });
    } else {
        polymer
            .array_windows::<2>()
            .map(|chunk| match rules.get(chunk) {
                None => chunk.to_vec(),
                Some(expanded) => expanded.clone(),
            })
            .for_each(|chunk| {
                for c in chunk.iter().take(2) {
                    match out.get_mut(c) {
                        None => {
                            out.insert(*c, 1);
                        }
                        Some(x) => *x += 1,
                    }
                }
            });
    }
}

pub fn part2(input: &str) -> usize {
    let (_, (polymer, rules)) = parse_input(input).unwrap();
    let rules = transform_rules2(&rules);
    let polymer: Vec<char> = polymer.chars().collect();
    let mut frequencies = HashMap::new();
    merge_apply(&polymer, &rules, 40, &mut frequencies);
    *frequencies.get_mut(polymer.last().unwrap()).unwrap() += 1;
    minmax(&frequencies)
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
