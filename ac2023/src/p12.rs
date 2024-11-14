use indicatif::ProgressIterator;
use itertools::{EitherOrBoth, Itertools};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::{map, map_res, value},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Spring {
    #[default]
    Operational,
    Damaged,
    Unknown,
}

impl std::fmt::Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Operational => '.',
                Self::Damaged => '#',
                Self::Unknown => '?',
            }
        )
    }
}

fn spring(input: &str) -> IResult<&str, Spring> {
    alt((
        value(Spring::Operational, tag(".")),
        value(Spring::Damaged, tag("#")),
        value(Spring::Unknown, tag("?")),
    ))(input)
}

struct Row {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {:?}",
            self.springs
                .iter()
                .map(Spring::to_string)
                .collect::<String>(),
            self.groups
        )
    }
}

impl Row {
    fn calculate_possibilities(&self) -> usize {
        let mut gaps = Vec::with_capacity(self.springs.len());
        count_options(&self.springs, &self.groups, &mut gaps)
    }
}

fn count_options(springs: &[Spring], groups: &[usize], gaps: &mut Vec<usize>) -> usize {
    if gaps.len() == groups.len() {
        usize::from(is_valid(springs, groups, gaps))
    } else {
        get_gap_options(springs, groups, gaps)
            .into_iter()
            .map(|gap| {
                gaps.push(gap);
                let count = count_options(springs, groups, gaps);
                gaps.pop();
                count
            })
            .sum()
    }
}

fn get_gap_options(springs: &[Spring], groups: &[usize], gaps: &[usize]) -> Vec<usize> {
    let occupied = gaps
        .iter()
        .zip(groups.iter())
        .map(|(x, y)| x + y)
        .sum::<usize>();
    let remaining =
        Itertools::intersperse(groups.iter().skip(gaps.len()).copied(), 1).sum::<usize>();
    let current_group = groups[gaps.len()];
    springs
        .iter()
        .skip(occupied + usize::from(gaps.is_empty()))
        .enumerate()
        .take_while(|(i, _)| {
            *i + occupied + remaining <= springs.len()
                && !springs
                    .iter()
                    .skip(occupied)
                    .take(*i)
                    .any(|x| *x == Spring::Damaged)
        })
        .filter(|(i, _)| {
            springs
                .iter()
                .skip(occupied + *i)
                .take(current_group)
                .all(|x| *x == Spring::Damaged || *x == Spring::Unknown)
                && springs
                    .get(occupied + i + current_group)
                    .is_none_or(|x| *x == Spring::Operational || *x == Spring::Unknown)
        })
        .map(|(i, _)| i)
        .collect()
}

fn is_valid(springs: &[Spring], groups: &[usize], gaps: &[usize]) -> bool {
    if springs.len() < groups.iter().sum::<usize>() + gaps.iter().sum::<usize>() {
        return false;
    }
    if gaps.iter().skip(1).any(|i| *i == 0) {
        return false;
    }
    let res = gaps
        .iter()
        .zip(groups.iter())
        .flat_map(|(gap, group)| {
            itertools::repeat_n(Spring::Operational, *gap)
                .chain(itertools::repeat_n(Spring::Damaged, *group))
        })
        .zip_longest(springs)
        .all(|either| match either {
            EitherOrBoth::Both(Spring::Operational, Spring::Operational | Spring::Unknown)
            | EitherOrBoth::Both(Spring::Damaged, Spring::Damaged | Spring::Unknown)
            | EitherOrBoth::Right(Spring::Operational | Spring::Unknown) => true,
            EitherOrBoth::Both(_, _)
            | EitherOrBoth::Right(Spring::Damaged)
            | EitherOrBoth::Left(_) => false,
        });
    res
}

fn row(input: &str) -> IResult<&str, Row> {
    map(
        separated_pair(
            many1(spring),
            space1,
            separated_list1(tag(","), map_res(digit1, str::parse)),
        ),
        |(springs, groups)| Row { springs, groups },
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Row>> {
    separated_list1(newline, row)(input)
}

pub fn part1(input: &str) -> String {
    let (_, rows) = parse(input).expect("parsing failure");
    let result: usize = rows.iter().map(Row::calculate_possibilities).sum();
    result.to_string()
}

fn expand(row: &Row) -> Row {
    let mut springs: Vec<Spring> = row.springs.clone();
    for _ in 0..4 {
        springs.push(Spring::Unknown);
        springs.extend_from_slice(&row.springs);
    }
    let groups = row.groups.repeat(5);
    Row { springs, groups }
}

pub fn part2(input: &str) -> String {
    let (_, rows) = parse(input).expect("parsing failure");
    let expanded_rows: Vec<Row> = rows.iter().map(expand).collect();
    let result: usize = expanded_rows
        .iter()
        .progress()
        .map(Row::calculate_possibilities)
        .sum();
    result.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1
";

    const DATA2: &str = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(DATA1), 6.to_string());
    }

    #[test]
    // #[ignore]
    fn test_part1_2() {
        assert_eq!(part1(DATA2), 21.to_string());
    }

    #[test]
    // #[ignore]
    fn test_part2_1() {
        assert_eq!(part2(DATA2), 525_152.to_string());
    }
}
