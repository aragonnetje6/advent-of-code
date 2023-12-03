fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn step(octopuses: &mut [Vec<u8>]) -> usize {
    octopuses
        .iter_mut()
        .for_each(|row| row.iter_mut().for_each(|x| *x += 1));

    let mut has_flashed = vec![vec![false; octopuses[0].len()]; octopuses.len()];
    let mut changed = true;
    while changed {
        changed = false;
        for x in 0..octopuses.len() {
            for y in 0..octopuses[0].len() {
                if octopuses[x][y] > 9 && !has_flashed[x][y] {
                    changed = true;
                    has_flashed[x][y] = true;
                    if x > 0 {
                        octopuses[x - 1][y] += 1;
                        if y > 0 {
                            octopuses[x - 1][y - 1] += 1;
                        }
                        if y < octopuses[0].len() - 1 {
                            octopuses[x - 1][y + 1] += 1;
                        }
                    }
                    if x < octopuses.len() - 1 {
                        octopuses[x + 1][y] += 1;
                        if y > 0 {
                            octopuses[x + 1][y - 1] += 1;
                        }
                        if y < octopuses[0].len() - 1 {
                            octopuses[x + 1][y + 1] += 1;
                        }
                    }
                    if y > 0 {
                        octopuses[x][y - 1] += 1;
                    }
                    if y < octopuses[0].len() - 1 {
                        octopuses[x][y + 1] += 1;
                    }
                }
            }
        }
    }
    has_flashed.iter().enumerate().for_each(|(x, row)| {
        row.iter().enumerate().for_each(|(y, elem)| {
            if *elem {
                octopuses[x][y] = 0;
            }
        });
    });

    has_flashed
        .iter()
        .map(|row| row.iter().filter(|x| **x).count())
        .sum()
}

pub fn part1(input: &str) -> String {
    let mut data = parse(input);
    (0..100).map(|_| step(&mut data)).sum::<usize>().to_string()
}

pub fn part2(input: &str) -> String {
    let mut data = parse(input);
    ((0..1000)
        .find(|_| step(&mut data) == data.len() * data[0].len())
        .unwrap()
        + 1)
    .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), "1656");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), "195");
    }
}
