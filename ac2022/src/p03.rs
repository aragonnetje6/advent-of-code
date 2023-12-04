use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Item {
    letter: char,
    score: usize,
}

impl Item {
    fn new(c: char) -> Self {
        Self {
            letter: c,
            score: ('a'..='z')
                .chain('A'..='Z')
                .collect::<String>()
                .find(c)
                .expect("parsing error")
                + 1,
        }
    }
}

struct Rucksack {
    compartment_1: Vec<Item>,
    compartment_2: Vec<Item>,
}

impl FromStr for Rucksack {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(&s.chars().map(Item::new).collect::<Vec<Item>>()))
    }
}

impl Rucksack {
    fn new(contents: &[Item]) -> Self {
        let (compartment_1, compartment_2) = contents.split_at(contents.len() / 2);
        Self {
            compartment_1: compartment_1.to_vec(),
            compartment_2: compartment_2.to_vec(),
        }
    }

    fn get_duplicate(&self) -> Item {
        let set1: HashSet<Item, RandomState> = HashSet::from_iter(self.compartment_1.clone());
        let set2 = HashSet::from_iter(self.compartment_2.clone());
        *set1.intersection(&set2).next().unwrap()
    }

    fn total(&self) -> Vec<Item> {
        [self.compartment_1.clone(), self.compartment_2.clone()].concat()
    }
}

fn process_input(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|line| str::parse::<Rucksack>(line).unwrap())
        .collect()
}

pub fn part1(input: &str) -> String {
    let data = process_input(input);
    data.iter()
        .map(|rucksack| rucksack.get_duplicate().score)
        .sum::<usize>()
        .to_string()
}

fn find_common_item(sack1: &Rucksack, sack2: &Rucksack, sack3: &Rucksack) -> Item {
    let set1: HashSet<Item, RandomState> = HashSet::from_iter(sack1.total());
    let intersect12: HashSet<Item> = set1
        .intersection(&HashSet::from_iter(sack2.total()))
        .copied()
        .collect();
    intersect12
        .intersection(&HashSet::from_iter(sack3.total()))
        .copied()
        .next()
        .unwrap()
}

pub fn part2(input: &str) -> String {
    let data = process_input(input);
    data.iter()
        .enumerate()
        .step_by(3)
        .map(|(i, rucksack)| find_common_item(rucksack, &data[i + 1], &data[i + 2]).score)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), "157");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), "70");
    }
}
