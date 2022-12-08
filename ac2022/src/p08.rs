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

fn mark_visible(trees: &[Vec<u8>]) -> Vec<Vec<bool>> {
    let mut visible = vec![vec![false; trees[0].len()]; trees.len()];
    visible
        .first_mut()
        .unwrap()
        .iter_mut()
        .for_each(|x| *x = true);
    visible
        .last_mut()
        .unwrap()
        .iter_mut()
        .for_each(|x| *x = true);
    visible
        .iter_mut()
        .for_each(|line| *line.first_mut().unwrap() = true);
    visible
        .iter_mut()
        .for_each(|line| *line.last_mut().unwrap() = true);
    for (i, line) in trees.iter().enumerate().skip(1) {
        let mut prev_max = line.first().unwrap();
        for (j, height) in line.iter().enumerate().skip(1) {
            if height > prev_max {
                prev_max = height;
                visible[i][j] = true;
            }
        }
        let mut prev_max = line.last().unwrap();
        for (j, height) in line.iter().enumerate().rev().skip(1) {
            if height > prev_max {
                prev_max = height;
                visible[i][j] = true;
            }
        }
    }
    for j in 1..trees[0].len() - 1 {
        let mut prev_max = trees[0][j];
        for (i, height) in trees.iter().map(|line| line[j]).enumerate().skip(1) {
            if height > prev_max {
                prev_max = height;
                visible[i][j] = true;
            }
        }
        let mut prev_max = trees.last().unwrap()[j];
        for (i, height) in trees.iter().map(|line| line[j]).enumerate().rev().skip(1) {
            if height > prev_max {
                prev_max = height;
                visible[i][j] = true;
            }
        }
    }
    visible
}

pub fn part1(input: &str) -> usize {
    let data = parse(input);
    mark_visible(&data)
        .iter()
        .map(|line| line.iter().filter(|x| **x).count())
        .sum()
}

fn get_view_from(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let mut result = 1;
    let height = trees[x][y];
    for new_x in x + 1..trees.len() {
        if trees[new_x][y] >= height || new_x == trees.len() - 1 {
            result *= x.abs_diff(new_x);
            break;
        }
    }
    for new_y in y + 1..trees[0].len() {
        if trees[x][new_y] >= height || new_y == trees[0].len() - 1 {
            result *= y.abs_diff(new_y);
            break;
        }
    }
    for new_x in (0..x).rev() {
        if trees[new_x][y] >= height || new_x == 0 {
            result *= x.abs_diff(new_x);
            break;
        }
    }
    for new_y in (0..y).rev() {
        if trees[x][new_y] >= height || new_y == 0 {
            result *= y.abs_diff(new_y);
            break;
        }
    }
    result
}

pub fn part2(input: &str) -> usize {
    let data = parse(input);
    data.iter()
        .enumerate()
        .skip(1)
        .take(data.len() - 2)
        .map(|(x, line)| {
            line.iter()
                .enumerate()
                .skip(1)
                .take(line.len() - 2)
                .map(|(y, _)| get_view_from(&data, x, y))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"30373
25512
65332
33549
35390
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 8);
    }
}
