use std::collections::HashSet;

use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, newline, one_of};
use nom::combinator::map_res;
use nom::multi::{count, separated_list1};
use nom::IResult;

type Point = (usize, usize);

fn number(i: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(i)
}

fn coord(i: &str) -> IResult<&str, Point> {
    let (i, x) = number(i)?;
    let (i, _) = char(',')(i)?;
    let (i, y) = number(i)?;
    Ok((i, (x, y)))
}

fn points(i: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(newline, coord)(i)
}

#[derive(Debug, Clone, Copy)]
enum FoldInstruction {
    X(usize),
    Y(usize),
}

fn fold_instruction(i: &str) -> IResult<&str, FoldInstruction> {
    let (i, _) = tag("fold along ")(i)?;
    let (i, axis) = one_of("xy")(i)?;
    let (i, _) = char('=')(i)?;
    let (i, magnitude) = number(i)?;
    Ok((
        i,
        match axis {
            'x' => FoldInstruction::X(magnitude),
            'y' => FoldInstruction::Y(magnitude),
            _ => unreachable!(),
        },
    ))
}

fn fold_instructions(i: &str) -> IResult<&str, Vec<FoldInstruction>> {
    separated_list1(newline, fold_instruction)(i)
}

fn parse_input(i: &str) -> IResult<&str, (Vec<Point>, Vec<FoldInstruction>)> {
    let (i, dots) = points(i)?;
    let (i, _) = count(newline, 2)(i)?;
    let (i, folds) = fold_instructions(i)?;
    Ok((i, (dots, folds)))
}

fn execute_fold(dots: &mut Vec<Point>, fold_instruction: &FoldInstruction) {
    match fold_instruction {
        FoldInstruction::X(x) => {
            for dot in dots {
                if dot.0 > *x {
                    dot.0 = x - (dot.0 - x);
                }
            }
        }
        FoldInstruction::Y(y) => {
            for dot in dots {
                if dot.1 > *y {
                    dot.1 = y - (dot.1 - y);
                }
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let (_, (mut dots, folds)) = parse_input(input).unwrap();
    execute_fold(&mut dots, folds.first().unwrap());
    dots.iter().collect::<HashSet<_>>().len()
}

fn render_grid(dots: &Vec<Point>) -> String {
    let max_x = dots.iter().map(|x| x.0).max().unwrap();
    let max_y = dots.iter().map(|x| x.1).max().unwrap();

    let mut grid = vec![vec![' '; max_x + 1]; max_y + 1];
    for dot in dots {
        grid[dot.1][dot.0] = '#';
    }
    grid.iter()
        .map(|line| line.iter().collect())
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn part2(input: &str) -> String {
    let (_, (mut dots, folds)) = parse_input(input).unwrap();
    for fold in folds {
        execute_fold(&mut dots, &fold);
    }
    format!("\n{}", render_grid(&dots))
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 17);
    }

    #[test]
    fn test_part2() {
        part2(DATA);
    }
}
