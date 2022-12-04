use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split(" | ")
                .nth(1)
                .unwrap()
                .split(' ')
                .map(str::len)
                .filter(|len| [2, 3, 4, 7].contains(len))
                .count()
        })
        .sum()
}

fn parse_input(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" | ");
            (
                split.next().unwrap().split(' ').collect(),
                split.next().unwrap().split(' ').collect(),
            )
        })
        .collect()
}

fn segments(n: u8) -> HashSet<u8> {
    HashSet::from_iter(match n {
        0 => vec![0, 1, 2, 4, 5, 6],
        1 => vec![2, 5],
        2 => vec![0, 2, 3, 4, 6],
        3 => vec![0, 2, 3, 5, 6],
        4 => vec![1, 2, 3, 5],
        5 => vec![0, 1, 3, 5, 6],
        6 => vec![0, 1, 3, 4, 5, 6],
        7 => vec![0, 2, 5],
        8 => vec![0, 1, 2, 3, 4, 5, 6],
        9 => vec![0, 1, 2, 3, 5, 6],
        _ => unreachable!(),
    })
}

fn get_possibilities(input: &[&str], output: &[&str]) -> HashMap<char, HashSet<u8>> {
    let all_nums = (0..=6).collect::<HashSet<_>>();
    let mut possibilities: HashMap<char, HashSet<u8>> =
        ('a'..='g').map(|c| (c, all_nums.clone())).collect();
    input
        .iter()
        .chain(output.iter())
        .for_each(|charset| match charset.len() {
            2 => charset.chars().for_each(|c| {
                possibilities
                    .get_mut(&c)
                    .unwrap()
                    .retain(|x| segments(1).contains(x));
            }),
            3 => charset.chars().for_each(|c| {
                possibilities
                    .get_mut(&c)
                    .unwrap()
                    .retain(|x| segments(7).contains(x));
            }),
            4 => charset.chars().for_each(|c| {
                possibilities
                    .get_mut(&c)
                    .unwrap()
                    .retain(|x| segments(4).contains(x));
            }),
            7 => charset.chars().for_each(|c| {
                possibilities
                    .get_mut(&c)
                    .unwrap()
                    .retain(|x| segments(8).contains(x));
            }),
            _ => {}
        });
    possibilities
}

fn charmap_to_num(candidate: &HashMap<char, u8>, num: &str) -> Option<u8> {
    let decoded = num
        .chars()
        .map(|c| candidate.get(&c).unwrap())
        .copied()
        .collect();
    dbg!((0..10).find(|&i| segments(i).eq(dbg!(&decoded))))
}

fn check(candidate: &HashMap<char, u8>, input: &[&str], output: &[&str]) -> bool {
    let all_nums: HashSet<u8> = (0..=9).collect::<HashSet<_>>();
    let Some(input_nums) = input.iter().map(|num| charmap_to_num(candidate, num)).collect() else {
        return false;
    };
    if all_nums.eq(&input_nums) {
        return false;
    }
    let Some(output_nums) = output.iter().map(|num| charmap_to_num(candidate, num)).collect() else {
        return false;
    };
    all_nums.is_superset(&output_nums)
}

fn quick_check(assumptions: &HashMap<char, HashSet<u8>>) -> bool {
    assumptions.iter().any(|(c, set)| {
        set.len() == 1
            && assumptions
                .iter()
                .any(|(c2, set2)| c != c2 && set2.is_superset(set))
    })
}

fn backtrack(
    assumptions: &HashMap<char, HashSet<u8>>,
    input: &[&str],
    output: &[&str],
) -> Result<HashMap<char, u8>, &'static str> {
    for (c, options) in assumptions.iter() {
        if options.len() == 1 {
            continue;
        }
        for opt in options {
            let mut new_assumptions = assumptions.clone();
            new_assumptions.insert(*c, HashSet::from_iter(vec![*opt]));
            if !quick_check(&new_assumptions) {
                continue;
            }
            match backtrack(&new_assumptions, input, output) {
                Ok(x) => return Ok(x),
                Err(_) => continue,
            }
        }
    }
    if !quick_check(assumptions){
        return Err("impossible");
    }
    if assumptions.iter().any(|(_, opts)| opts.len() > 1) {
        return Err("Inconclusive")
    }
    let candidate = assumptions
        .iter()
        .map(|(c, o)| (*c, *o.iter().next().unwrap()))
        .collect();
    if dbg!(check(dbg!(&candidate), input, output)) {
        Ok(candidate)
    } else {
        Err("Impossible")
    }
}

fn decode(input: &[&str], output: &[&str]) -> u32 {
    let possibilities = get_possibilities(input, output);
    let mapping = backtrack(&possibilities, input, output).unwrap();
    output
        .iter()
        .map(|digit| charmap_to_num(&mapping, digit).unwrap().to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}

pub fn part2(input: &str) -> u32 {
    println!("running");
    let data = parse_input(input);
    println!("{data:?}");
    data.iter()
        .map(|(input, output)| decode(input, output))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";
    const DATA2: &str =
        r"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    #[test]
    fn test_part1_small() {
        assert_eq!(part1(DATA2), 0);
    }

    #[test]
    fn test_part1_big() {
        assert_eq!(part1(DATA1), 26);
    }

    #[test]
    #[ignore]
    fn test_part2_small() {
        assert_eq!(part2(DATA2), 5353);
    }

    #[test]
    #[ignore]
    fn test_part2_big() {
        assert_eq!(part2(DATA1), 61229);
    }
}
