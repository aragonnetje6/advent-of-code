fn parse(input: &str) -> Vec<(u128, u128)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c == '#').then_some((x as u128, y as u128)))
        })
        .collect()
}

fn expand(galaxies: &mut [(u128, u128)], factor: u128) {
    let max_x = *galaxies.iter().map(|(x, _)| x).max().expect("no galaxies");
    let max_y = *galaxies.iter().map(|(_, y)| y).max().expect("no galaxies");
    let mut shifts_x = vec![];
    for x in 0..=max_x {
        if galaxies.iter().any(|(gx, _)| *gx == x) {
            shifts_x.push(shifts_x.last().copied().unwrap_or(0));
        } else {
            shifts_x.push(shifts_x.last().copied().unwrap_or(0) + (factor - 1));
        }
    }
    for (i, shift) in shifts_x.into_iter().enumerate().rev() {
        for galaxy in galaxies.iter_mut() {
            if galaxy.0 == i as u128 {
                galaxy.0 += shift;
            }
        }
    }
    let mut shifts_y = vec![];
    for y in 0..=max_y {
        if galaxies.iter().any(|(_, gy)| *gy == y) {
            shifts_y.push(shifts_y.last().copied().unwrap_or(0));
        } else {
            shifts_y.push(shifts_y.last().copied().unwrap_or(0) + (factor - 1));
        }
    }
    for (i, shift) in shifts_y.into_iter().enumerate().rev() {
        for galaxy in galaxies.iter_mut() {
            if galaxy.1 == i as u128 {
                galaxy.1 += shift;
            }
        }
    }
}

const fn path_length(galaxy1: (u128, u128), galaxy2: (u128, u128)) -> u128 {
    galaxy1.0.abs_diff(galaxy2.0) + galaxy1.1.abs_diff(galaxy2.1)
}

fn calculate_generic(input: &str, factor: u128) -> String {
    let mut galaxies = parse(input);
    expand(&mut galaxies, factor);
    let total: u128 = galaxies
        .iter()
        .enumerate()
        .map(|(i, galaxy1)| {
            galaxies
                .iter()
                .skip(i + 1)
                .map(|galaxy2| path_length(*galaxy1, *galaxy2))
                .sum::<u128>()
        })
        .sum();
    total.to_string()
}

pub fn part1(input: &str) -> String {
    calculate_generic(input, 2)
}

pub fn part2(input: &str) -> String {
    calculate_generic(input, 1_000_000)
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(DATA1), 374.to_string());
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(calculate_generic(DATA1, 10), 1030.to_string());
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(calculate_generic(DATA1, 100), 8410.to_string());
    }
}
