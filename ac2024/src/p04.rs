fn horizontal(input: &[&str]) -> usize {
    input
        .iter()
        .map(|line| line.matches("XMAS").count() + line.matches("SAMX").count())
        .sum()
}

fn vertical(input: &[Vec<char>]) -> usize {
    (0..input[0].len())
        .map(|i| {
            let col = input.iter().map(|x| x[i]).collect::<String>();
            col.matches("XMAS").count() + col.matches("SAMX").count()
        })
        .sum()
}

fn diagonal(input: &[Vec<char>]) -> usize {
    input
        .windows(4)
        .map(|rows| {
            itertools::multizip((
                rows[0].iter(),
                rows[1].iter().skip(1),
                rows[2].iter().skip(2),
                rows[3].iter().skip(3),
            ))
            .filter(|(&c1, &c2, &c3, &c4)| {
                let word = [c1, c2, c3, c4].into_iter().collect::<String>();
                word == "XMAS" || word == "SAMX"
            })
            .count()
        })
        .sum::<usize>()
        + input
            .windows(4)
            .map(|rows| {
                itertools::multizip((
                    rows[0].iter().skip(3),
                    rows[1].iter().skip(2),
                    rows[2].iter().skip(1),
                    rows[3].iter(),
                ))
                .filter(|(&c1, &c2, &c3, &c4)| {
                    let word = [c1, c2, c3, c4].into_iter().collect::<String>();
                    word == "XMAS" || word == "SAMX"
                })
                .count()
            })
            .sum::<usize>()
}

pub fn part1(input: &str) -> String {
    let data: Vec<&str> = input.lines().filter(|x| !x.is_empty()).collect();
    let as_vecs: Vec<Vec<char>> = data.iter().map(|x| x.chars().collect()).collect();
    (horizontal(&data) + vertical(&as_vecs) + diagonal(&as_vecs)).to_string()
}

fn count_x_mas(input: &[Vec<char>]) -> usize {
    input
        .windows(3)
        .map(|rows| {
            itertools::multizip((rows[0].windows(3), rows[1].windows(3), rows[2].windows(3)))
                .filter(|(l1, l2, l3)| {
                    l2[1] == 'A'
                        && ((l1[0] == 'M' && l3[2] == 'S') || (l1[0] == 'S' && l3[2] == 'M'))
                        && ((l3[0] == 'M' && l1[2] == 'S') || (l3[0] == 'S' && l1[2] == 'M'))
                })
                .count()
        })
        .sum::<usize>()
}

pub fn part2(input: &str) -> String {
    let data: Vec<&str> = input.lines().filter(|x| !x.is_empty()).collect();
    let as_vecs: Vec<Vec<char>> = data.iter().map(|x| x.chars().collect()).collect();
    count_x_mas(&as_vecs).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 18.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 9.to_string());
    }
}
