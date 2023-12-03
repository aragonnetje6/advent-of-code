use std::collections::{BTreeMap, BTreeSet};

const fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

const fn is_gear(c: char) -> bool {
    c == '*'
}

pub fn part1(input: &str) -> String {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut parts: Vec<u32> = vec![];
    for (i, &line) in lines.iter().enumerate() {
        let mut current_number = vec![];
        let mut is_valid = false;
        for (j, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                current_number.push(c);
                if !is_valid
                    && (is_touching_symbol(j, lines[i])
                        || (i >= 1 && is_touching_symbol(j, lines[i - 1]))
                        || (i <= lines.len() - 2 && is_touching_symbol(j, lines[i + 1])))
                {
                    is_valid = true;
                }
            } else if is_valid {
                parts.push(
                    current_number
                        .into_iter()
                        .collect::<String>()
                        .parse::<u32>()
                        .expect("wrong digit"),
                );
                current_number = vec![];
                is_valid = false;
            } else {
                current_number.clear();
            }
        }
        if is_valid {
            parts.push(
                current_number
                    .into_iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .expect("wrong digit"),
            );
        }
    }
    parts.into_iter().sum::<u32>().to_string()
}

fn is_touching_symbol(j: usize, line: &str) -> bool {
    (j >= 1 && line.chars().nth(j - 1).is_some_and(is_symbol))
        || (j <= line.len() - 2 && line.chars().nth(j + 1).is_some_and(is_symbol))
        || line.chars().nth(j).is_some_and(is_symbol)
}

fn is_touching_gear_on_line(j: usize, line: &str) -> Vec<usize> {
    let mut result = vec![];
    if j >= 1 && line.chars().nth(j - 1).is_some_and(is_gear) {
        result.push(j - 1);
    }
    if line.chars().nth(j).is_some_and(is_gear) {
        result.push(j);
    }
    if j < line.len() - 1 && line.chars().nth(j + 1).is_some_and(is_gear) {
        result.push(j + 1);
    }
    result
}

fn is_touching_gear(i: usize, j: usize, lines: &[&str]) -> Vec<(usize, usize)> {
    let mut result = vec![];
    if i >= 1 {
        result.extend(
            is_touching_gear_on_line(j, lines[i - 1])
                .into_iter()
                .map(|x| (i - 1, x)),
        );
    }
    result.extend(
        is_touching_gear_on_line(j, lines[i])
            .into_iter()
            .map(|x| (i, x)),
    );
    if i < lines.len() - 1 {
        result.extend(
            is_touching_gear_on_line(j, lines[i + 1])
                .into_iter()
                .map(|x| (i + 1, x)),
        );
    }
    result
}

struct Number {
    num: u32,
    gears: BTreeSet<(usize, usize)>,
}

pub fn part2(input: &str) -> String {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut parts: Vec<Number> = vec![];
    for (i, &line) in lines.iter().enumerate() {
        let mut current_number = vec![];
        let mut adjacent_gears = vec![];
        for (j, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                current_number.push(c);
                adjacent_gears.extend(is_touching_gear(i, j, &lines));
            } else if !adjacent_gears.is_empty() {
                parts.push(Number {
                    num: current_number
                        .into_iter()
                        .collect::<String>()
                        .parse::<u32>()
                        .expect("wrong digit"),
                    gears: adjacent_gears.into_iter().collect(),
                });
                current_number = vec![];
                adjacent_gears = vec![];
            } else {
                current_number.clear();
                adjacent_gears.clear();
            }
        }
        if !adjacent_gears.is_empty() {
            parts.push(Number {
                num: current_number
                    .into_iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .expect("wrong digit"),
                gears: adjacent_gears.into_iter().collect(),
            });
        }
    }
    let mut map: BTreeMap<(usize, usize), Vec<u32>> = BTreeMap::new();
    for Number { num, gears } in parts {
        for gear in gears {
            if let Some(entry) = map.get_mut(&gear) {
                entry.push(num);
            } else {
                map.insert(gear, vec![num]);
            }
        }
    }
    map.values()
        .filter(|x| x.len() == 2)
        .map(|x| x[0] * x[1])
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 4361.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 467_835.to_string());
    }
}
