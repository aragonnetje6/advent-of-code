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
    (0..10).find(|&i| segments(i).eq(&decoded))
}

fn counting_reduce(possibilities: &mut HashMap<char, HashSet<u8>>, input: &[&str]) {
    for c in 'a'..='g' {
        let count: usize = input.iter().map(|x| x.chars().filter(|y| *y == c).count()).sum();
        possibilities.get_mut(&c).unwrap().retain(|x| match count {
            4 => vec![4],
            6 => vec![1],
            7 => vec![3, 6],
            8 => vec![0, 2],
            9 => vec![5],
            _ => unreachable!()
        }.contains(x));
    }
}

fn dedup(possibilities: &mut HashMap<char, HashSet<u8>>) {
    let mut changed = true;
    while changed {
        changed = false;
        for c in 'a'..='g' {
            if possibilities.get(&c).unwrap().len() > 1 {
                continue
            }
            let options = possibilities.get(&c).unwrap().clone();
            for c2 in ('a'..='g').filter(|x| x != &c) {
                if possibilities.get_mut(&c2).unwrap().remove(options.iter().next().unwrap()) {
                    changed = true;
                }
            }
        }
    }
}

fn decode(input: &[&str], output: &[&str]) -> u32 {
    let mut possibilities = get_possibilities(input, output);
    counting_reduce(&mut possibilities, input);
    dedup(&mut possibilities);
    let mapping = possibilities.iter().map(|(c, set)| (*c, *set.iter().next().unwrap())).collect();
    // let mapping = backtrack(&dbg!(possibilities), input, output).unwrap();
    output
        .iter()
        .map(|digit| charmap_to_num(&mapping, digit).unwrap().to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}

pub fn part2(input: &str) -> u32 {
    let data = parse_input(input);
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
    fn test_part2_small() {
        assert_eq!(part2(DATA2), 5353);
    }

    #[test]
    fn test_part2_big() {
        assert_eq!(part2(DATA1), 61229);
    }
}
