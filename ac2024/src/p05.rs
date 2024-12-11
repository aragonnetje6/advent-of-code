use nom::{
    character::complete,
    combinator::map,
    multi::{count, separated_list1},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone, Copy)]
struct Rule {
    before: u64,
    after: u64,
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    map(
        separated_pair(complete::u64, complete::char('|'), complete::u64),
        |(before, after)| Rule { before, after },
    )(input)
}

fn parse_update(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(complete::char(','), complete::u64)(input)
}

fn parse_file(input: &str) -> IResult<&str, (Vec<Rule>, Vec<Vec<u64>>)> {
    separated_pair(
        separated_list1(complete::newline, parse_rule),
        count(complete::newline, 2),
        separated_list1(complete::newline, parse_update),
    )(input)
}

pub fn part1(input: &str) -> String {
    let (_, (rules, updates)) = parse_file(input).expect("parsing error");
    updates
        .into_iter()
        .filter(|update| is_valid(&rules, update))
        .map(|update| update[update.len() / 2])
        .sum::<u64>()
        .to_string()
}

fn is_valid(rules: &[Rule], update: &[u64]) -> bool {
    rules.iter().all(|rule| {
        update
            .iter()
            .position(|x| *x == rule.before)
            .and_then(|first| {
                update
                    .iter()
                    .position(|x| *x == rule.after)
                    .map(|last| first < last)
            })
            .unwrap_or(true)
    })
}

pub fn part2(input: &str) -> String {
    let (_, (rules, updates)) = parse_file(input).expect("parsing error");
    updates
        .into_iter()
        .filter(|update| !is_valid(&rules, update))
        .map(|mut update| {
            fix_update(&rules, &mut update);
            update[update.len() / 2]
        })
        .sum::<u64>()
        .to_string()
}

fn fix_update(rules: &[Rule], update: &mut [u64]) {
    while !is_valid(rules, update) {
        for rule in rules {
            if let Some(lower) = update.iter().position(|x| *x == rule.before) {
                if let Some(upper) = update.iter().position(|x| *x == rule.after) {
                    if upper < lower {
                        update.swap(lower, upper);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 143.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 123.to_string());
    }
}
