use std::{
    cmp::min,
    collections::{HashMap, HashSet},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline},
    combinator::{all_consuming, map, value},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start {
        x: i64,
        y: i64,
    },
    Pipe {
        x: i64,
        y: i64,
        connects: [(i64, i64); 2],
        directions: [Direction; 2],
    },
}

impl Tile {
    fn new_pipe(x: i64, y: i64, directions: [Direction; 2]) -> Self {
        let connects = directions.map(|dir| match dir {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        });
        Self::Pipe {
            x,
            y,
            connects,
            directions,
        }
    }

    const fn new_start(x: i64, y: i64) -> Self {
        Self::Start { x, y }
    }

    const fn x(&self) -> i64 {
        *match self {
            Self::Start { x, .. } | Self::Pipe { x, .. } => x,
        }
    }

    const fn y(&self) -> i64 {
        *match self {
            Self::Start { y, .. } | Self::Pipe { y, .. } => y,
        }
    }

    fn get_neighbours<'b>(&self, map: &'b HashMap<(i64, i64), Self>) -> Vec<&'b Self> {
        match self {
            Self::Start { x, y } => [(*x, y - 1), (*x, y + 1), (x - 1, *y), (x + 1, *y)]
                .iter()
                .filter_map(|x| map.get(x))
                .filter(|v| match v {
                    Self::Start { .. } => unreachable!(),
                    Self::Pipe { connects, .. } => connects.contains(&(*x, *y)),
                })
                .collect::<Vec<_>>(),
            Self::Pipe { connects, .. } => connects.iter().filter_map(|p| map.get(p)).collect(),
        }
    }

    fn get_directions(&self, map: &HashMap<(i64, i64), Self>) -> [Direction; 2] {
        match self {
            Self::Start { x, y } => {
                let temp = [(*x, y - 1), (*x, y + 1), (x - 1, *y), (x + 1, *y)]
                    .iter()
                    .filter_map(|x| map.get(x))
                    .filter(|v| match v {
                        Self::Start { .. } => unreachable!(),
                        Self::Pipe { connects, .. } => connects.contains(&(*x, *y)),
                    })
                    .map(|t| match self.x().cmp(&t.x()) {
                        std::cmp::Ordering::Equal => {
                            if self.y() > t.y() {
                                Direction::Up
                            } else {
                                Direction::Down
                            }
                        }
                        std::cmp::Ordering::Greater => Direction::Left,
                        std::cmp::Ordering::Less => Direction::Right,
                    })
                    .collect::<Vec<_>>();
                [temp[0], temp[1]]
            }
            Self::Pipe { directions, .. } => *directions,
        }
    }
}

fn tile(input: Span) -> IResult<Span, Option<Tile>> {
    alt((
        value(None, char('.')),
        map(
            alt((
                map(tag("S"), |c: Span| {
                    Tile::new_start(
                        i64::try_from(c.get_utf8_column()).expect("conversion failed") - 1,
                        i64::from(c.location_line()) - 1,
                    )
                }),
                map(tag("|"), |c: Span| {
                    Tile::new_pipe(
                        i64::try_from(c.get_utf8_column()).expect("conversion failed") - 1,
                        i64::from(c.location_line()) - 1,
                        [Direction::Up, Direction::Down],
                    )
                }),
                map(tag("-"), |c: Span| {
                    Tile::new_pipe(
                        i64::try_from(c.get_utf8_column()).expect("conversion failed") - 1,
                        i64::from(c.location_line()) - 1,
                        [Direction::Left, Direction::Right],
                    )
                }),
                map(tag("L"), |c: Span| {
                    Tile::new_pipe(
                        i64::try_from(c.get_utf8_column()).expect("conversion failed") - 1,
                        i64::from(c.location_line()) - 1,
                        [Direction::Up, Direction::Right],
                    )
                }),
                map(tag("J"), |c: Span| {
                    Tile::new_pipe(
                        i64::try_from(c.get_utf8_column()).expect("conversion failed") - 1,
                        i64::from(c.location_line()) - 1,
                        [Direction::Up, Direction::Left],
                    )
                }),
                map(tag("7"), |c: Span| {
                    Tile::new_pipe(
                        i64::try_from(c.get_utf8_column()).expect("conversion failed") - 1,
                        i64::from(c.location_line()) - 1,
                        [Direction::Left, Direction::Down],
                    )
                }),
                map(tag("F"), |c: Span| {
                    Tile::new_pipe(
                        i64::try_from(c.get_utf8_column()).expect("conversion failed") - 1,
                        i64::from(c.location_line()) - 1,
                        [Direction::Right, Direction::Down],
                    )
                }),
            )),
            Some,
        ),
    ))(input)
}

fn grid(input: Span) -> IResult<Span, Vec<Tile>> {
    all_consuming(terminated(
        map(
            separated_list1(
                newline,
                map(many1(tile), |v| {
                    v.into_iter().flatten().collect::<Vec<Tile>>()
                }),
            ),
            |v| v.into_iter().flatten().collect(),
        ),
        newline,
    ))(input)
}

fn parse(input: &str) -> Vec<Tile> {
    grid(Span::new(input)).expect("parsing failed").1
}

pub fn part1(input: &str) -> String {
    let tile_map: HashMap<(i64, i64), Tile> = parse(input)
        .into_iter()
        .map(|t| ((t.x(), t.y()), t))
        .collect();
    let start = tile_map
        .values()
        .find(|t| matches!(t, Tile::Start { .. }))
        .expect("no start");
    let mut prev_tile1 = start;
    let mut prev_tile2 = start;
    let mut tile1 = *start
        .get_neighbours(&tile_map)
        .first()
        .expect("no start neighbours");
    let mut tile2 = *start
        .get_neighbours(&tile_map)
        .last()
        .expect("no start neighbours");
    let mut maxlen = 1;
    while tile1 != tile2 {
        (tile1, prev_tile1) = (
            *tile1
                .get_neighbours(&tile_map)
                .iter()
                .find(|x| **x != prev_tile1)
                .expect("loop broken"),
            tile1,
        );
        if tile1 == tile2 {
            break;
        }
        (tile2, prev_tile2) = (
            *tile2
                .get_neighbours(&tile_map)
                .iter()
                .find(|x| **x != prev_tile2)
                .expect("loop broken"),
            tile2,
        );
        maxlen += 1;
    }
    maxlen.to_string()
}

pub fn part2(input: &str) -> String {
    let full_tile_map: HashMap<(i64, i64), Tile> = parse(input)
        .into_iter()
        .map(|t| ((t.x(), t.y()), t))
        .collect();
    let start = full_tile_map
        .values()
        .find(|t| matches!(t, Tile::Start { .. }))
        .expect("no start");
    let main_loop = get_main_loop(start, &full_tile_map);
    let tile_map: HashMap<(i64, i64), Tile> =
        main_loop.iter().map(|t| ((t.x(), t.y()), **t)).collect();

    let x_min = tile_map.values().map(Tile::x).min().expect("no tiles") - 1;
    let x_max = tile_map.values().map(Tile::x).max().expect("no tiles") + 1;
    let y_min = tile_map.values().map(Tile::y).min().expect("no tiles") - 1;
    let y_max = tile_map.values().map(Tile::y).max().expect("no tiles") + 1;

    let outside = floodfill(x_min, x_max, y_min, y_max, &tile_map);
    let all_tiles: HashSet<(i64, i64)> = (x_min..=x_max)
        .flat_map(|x| (y_min..=y_max).map(move |y| (x, y)))
        .collect();
    let not_outside: HashSet<(i64, i64)> = all_tiles.difference(&outside).copied().collect();
    let inside: HashSet<(i64, i64)> = not_outside
        .difference(&main_loop.iter().map(|t| (t.x(), t.y())).collect())
        .copied()
        .collect();
    inside.len().to_string()
}

fn get_main_loop<'a>(start: &'a Tile, tile_map: &'a HashMap<(i64, i64), Tile>) -> Vec<&'a Tile> {
    let mut main_loop = vec![start];
    let mut prev_tile = start;
    let mut tile = *start
        .get_neighbours(tile_map)
        .first()
        .expect("no start neighbours");
    while tile != start {
        main_loop.push(tile);
        (tile, prev_tile) = (
            *tile
                .get_neighbours(tile_map)
                .iter()
                .find(|x| **x != prev_tile)
                .expect("loop broken"),
            tile,
        );
    }
    main_loop
}

fn floodfill(
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    tile_map: &HashMap<(i64, i64), Tile>,
) -> HashSet<(i64, i64)> {
    let mut todo_stack = vec![(x_min, y_min)];
    let mut reachable = HashSet::from([(x_min, y_min)]);
    let mut checked = HashSet::new();

    while let Some(current) = todo_stack.pop() {
        checked.insert(current);
        for point in [
            (current.0 + 1, current.1),
            (current.0 - 1, current.1),
            (current.0, current.1 + 1),
            (current.0, current.1 - 1),
        ] {
            if point.0 < x_min
                || point.0 > x_max
                || point.1 < y_min
                || point.1 > y_max
                || checked.contains(&point)
            {
                continue;
            }
            if walkable(current, point, tile_map) {
                reachable.insert(point);
                todo_stack.push(point);
            }
        }
    }
    reachable
}

fn walkable(current: (i64, i64), point: (i64, i64), tile_map: &HashMap<(i64, i64), Tile>) -> bool {
    if current.0 == point.0 {
        tile_map
            .get(&(current.0, min(current.1, point.1)))
            .is_none_or(|t| !t.get_directions(tile_map).contains(&Direction::Left))
    } else {
        tile_map
            .get(&(min(current.0, point.0), current.1))
            .is_none_or(|t| !t.get_directions(tile_map).contains(&Direction::Up))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r".....
.S-7.
.|.|.
.L-J.
.....
";

    const DATA2: &str = r"-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";

    const DATA3: &str = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

    const DATA4: &str = r"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(DATA1), 4.to_string());
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(DATA2), 4.to_string());
    }

    #[test]
    fn test_part1_3() {
        assert_eq!(part1(DATA3), 8.to_string());
    }

    #[test]
    fn test_part1_4() {
        assert_eq!(part1(DATA4), 8.to_string());
    }

    const DATA5: &str = r"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

    const DATA6: &str = r"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
";

    const DATA7: &str = r".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

    const DATA8: &str = r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    #[test]
    fn test_part2_1() {
        assert_eq!(part2(DATA5), 4.to_string());
    }
    #[test]
    fn test_part2_2() {
        assert_eq!(part2(DATA6), 4.to_string());
    }
    #[test]
    fn test_part2_3() {
        assert_eq!(part2(DATA7), 8.to_string());
    }
    #[test]
    fn test_part2_4() {
        assert_eq!(part2(DATA8), 10.to_string());
    }
}
