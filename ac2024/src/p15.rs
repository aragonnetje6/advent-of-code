use nom::{
    branch::alt,
    character::complete,
    combinator::{map, value},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Box,
    Robot,
    BoxL,
    BoxR,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    x: usize,
    y: usize,
}

impl Robot {
    const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn parse_map(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(
        complete::newline,
        many1(alt((
            value(Tile::Empty, complete::char('.')),
            value(Tile::Wall, complete::char('#')),
            value(Tile::Box, complete::char('O')),
            value(Tile::Robot, complete::char('@')),
        ))),
    )(input)
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    map(
        separated_list1(
            complete::newline,
            many1(alt((
                value(Move::Up, complete::char('^')),
                value(Move::Left, complete::char('<')),
                value(Move::Right, complete::char('>')),
                value(Move::Down, complete::char('v')),
            ))),
        ),
        |x| x.into_iter().flatten().collect(),
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Vec<Tile>>, Vec<Move>)> {
    separated_pair(parse_map, complete::multispace1, parse_moves)(input)
}

fn find_robot(map: &[Vec<Tile>]) -> Option<Robot> {
    map.iter().enumerate().find_map(|(y, line)| {
        line.iter()
            .enumerate()
            .find_map(|(x, item)| matches!(*item, Tile::Robot).then(|| Robot::new(x, y)))
    })
}

fn can_move(map: &mut [Vec<Tile>], x: usize, y: usize, direction: Move) -> bool {
    match map[y][x] {
        Tile::Empty => true,
        Tile::Wall => false,
        Tile::Box => {
            let (other_x, other_y) = get_obstacle(x, y, direction);
            can_move(map, other_x, other_y, direction)
        }
        Tile::Robot => unreachable!(),
        Tile::BoxL => match direction {
            Move::Up | Move::Down => {
                let (other_x, other_y) = get_obstacle(x, y, direction);
                can_move(map, other_x, other_y, direction)
                    && can_move(map, other_x + 1, other_y, direction)
            }
            Move::Left => can_move(map, x - 1, y, direction),
            Move::Right => can_move(map, x + 2, y, direction),
        },
        Tile::BoxR => match direction {
            Move::Up | Move::Down => {
                let (other_x, other_y) = get_obstacle(x, y, direction);
                can_move(map, other_x, other_y, direction)
                    && can_move(map, other_x - 1, other_y, direction)
            }
            Move::Left => can_move(map, x - 2, y, direction),
            Move::Right => can_move(map, x + 1, y, direction),
        },
    }
}

fn do_move(map: &mut [Vec<Tile>], x: usize, y: usize, direction: Move) {
    let (other_x, other_y) = get_obstacle(x, y, direction);
    match map[y][x] {
        Tile::Empty => (),
        Tile::Wall => unreachable!(),
        Tile::Box => {
            do_move(map, other_x, other_y, direction);
            map[other_y][other_x] = map[y][x];
            map[y][x] = Tile::Empty;
        }
        Tile::Robot => unreachable!(),
        Tile::BoxL => {
            match direction {
                Move::Up | Move::Down => {
                    do_move(map, other_x, other_y, direction);
                    do_move(map, other_x + 1, other_y, direction);
                }
                Move::Left => do_move(map, x - 1, y, direction),
                Move::Right => do_move(map, x + 2, y, direction),
            }
            map[y][x] = Tile::Empty;
            map[y][x + 1] = Tile::Empty;
            map[other_y][other_x] = Tile::BoxL;
            map[other_y][other_x + 1] = Tile::BoxR;
        }
        Tile::BoxR => {
            match direction {
                Move::Up | Move::Down => {
                    do_move(map, other_x, other_y, direction);
                    do_move(map, other_x - 1, other_y, direction);
                }
                Move::Left => do_move(map, x - 2, y, direction),
                Move::Right => do_move(map, x + 1, y, direction),
            }
            map[y][x] = Tile::Empty;
            map[y][x - 1] = Tile::Empty;
            map[other_y][other_x - 1] = Tile::BoxL;
            map[other_y][other_x] = Tile::BoxR;
        }
    }
}

fn try_move(map: &mut [Vec<Tile>], x: usize, y: usize, direction: Move) -> bool {
    #![allow(clippy::match_on_vec_items)]
    let (other_x, other_y) = get_obstacle(x, y, direction);
    if can_move(map, other_x, other_y, direction) {
        do_move(map, other_x, other_y, direction);
        map[other_y][other_x] = Tile::Robot;
        map[y][x] = Tile::Empty;
        true
    } else {
        false
    }
}

const fn get_obstacle(x: usize, y: usize, direction: Move) -> (usize, usize) {
    match direction {
        Move::Up => (x, y - 1),
        Move::Down => (x, y + 1),
        Move::Left => (x - 1, y),
        Move::Right => (x + 1, y),
    }
}

fn get_gps(map: &[Vec<Tile>]) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, tile)| {
                matches!(tile, Tile::Box | Tile::BoxL).then_some(y * 100 + x)
            })
        })
        .sum::<usize>()
}

fn display(map: &[Vec<Tile>]) {
    for line in map {
        println!(
            "{}",
            line.iter()
                .map(|x| match x {
                    Tile::Empty => '.',
                    Tile::Wall => '#',
                    Tile::Box => 'O',
                    Tile::Robot => '@',
                    Tile::BoxL => '[',
                    Tile::BoxR => ']',
                })
                .collect::<String>()
        );
    }
}

pub fn part1(input: &str) -> String {
    let (_, (mut map, moves)) = parse_input(input).expect("parsing error");
    let mut robot = find_robot(&map).expect("no robot");

    for direction in moves {
        if try_move(&mut map, robot.x, robot.y, direction) {
            match direction {
                Move::Up => robot.y -= 1,
                Move::Down => robot.y += 1,
                Move::Left => robot.x -= 1,
                Move::Right => robot.x += 1,
            }
        }
    }
    get_gps(&map).to_string()
}

fn widen(map: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    map.iter()
        .map(|line| {
            line.iter()
                .copied()
                .flat_map(|x| match x {
                    Tile::Empty => [Tile::Empty, Tile::Empty],
                    Tile::Wall => [Tile::Wall, Tile::Wall],
                    Tile::Box => [Tile::BoxL, Tile::BoxR],
                    Tile::Robot => [Tile::Robot, Tile::Empty],
                    Tile::BoxL | Tile::BoxR => unreachable!(),
                })
                .collect()
        })
        .collect()
}

pub fn part2(input: &str) -> String {
    let (_, (mut map, moves)) = parse_input(input).expect("parsing error");
    map = widen(&map);
    let mut robot = find_robot(&map).expect("no robot");
    for direction in moves {
        if try_move(&mut map, robot.x, robot.y, direction) {
            match direction {
                Move::Up => robot.y -= 1,
                Move::Down => robot.y += 1,
                Move::Left => robot.x -= 1,
                Move::Right => robot.x += 1,
            }
        }
    }
    get_gps(&map).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    const DATA2: &str = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(DATA1), 10092.to_string());
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(DATA2), 2028.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 9021.to_string());
    }
}
