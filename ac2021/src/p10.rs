const fn closing(bracket: char) -> char {
    match bracket {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

#[derive(Debug)]
enum Status {
    Ok,
    Incomplete(Vec<char>),
    Corrupted(char),
}

fn check_line(line: &str) -> Status {
    let mut stack = vec![];
    for c in line.chars() {
        if "([{<".contains(c) {
            stack.push(c);
        } else if let Some(opening) = stack.pop() {
            if closing(opening) != c {
                return Status::Corrupted(c);
            }
        }
    }
    if stack.is_empty() {
        Status::Ok
    } else {
        stack.reverse();
        Status::Incomplete(stack)
    }
}

const fn corrupt_score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

pub fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            if let Status::Corrupted(c) = check_line(line) {
                corrupt_score(c)
            } else {
                0
            }
        })
        .sum::<u32>()
        .to_string()
}

const fn incomplete_score(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!(),
    }
}

pub fn part2(input: &str) -> String {
    let mut scored = input
        .lines()
        .filter_map(|line| {
            if let Status::Incomplete(rest) = check_line(line) {
                Some(
                    rest.iter()
                        .copied()
                        .map(incomplete_score)
                        .reduce(|acc, x| acc * 5 + x)
                        .unwrap(),
                )
            } else {
                None
            }
        })
        .collect::<Vec<u64>>();
    scored.sort_unstable();
    scored[scored.len() / 2].to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), "26397");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), "288957");
    }
}
