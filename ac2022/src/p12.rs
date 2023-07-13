use nom::{
    character::{
        complete::{anychar, newline},
        is_alphabetic,
    },
    combinator::verify,
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Normal(u32),
    Start,
    End,
}

fn tile(input: &str) -> IResult<&str, Tile> {
    let (input, c) = verify(anychar, |x| is_alphabetic(*x as u8))(input)?;
    let tile = match c {
        'S' => Tile::Start,
        'E' => Tile::End,
        x => Tile::Normal(u32::from(x)),
    };
    Ok((input, tile))
}

fn line(input: &str) -> IResult<&str, Vec<Tile>> {
    many1(tile)(input)
}

fn grid(input: &str) -> IResult<&str, Grid> {
    separated_list1(newline, line)(input)
}

type Grid = Vec<Vec<Tile>>;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point(usize, usize);

impl Point {
    fn get_height(&self, grid: &Grid) -> u32 {
        let &Self(x, y) = self;
        match *grid[x].get(y).unwrap() {
            Tile::Normal(x) => x,
            Tile::Start => u32::from('a'),
            Tile::End => u32::from('z'),
        }
    }

    fn reversed_successors(&self, grid: &Grid) -> Vec<Self> {
        let &Self(x, y) = self;
        let mut out = Vec::with_capacity(4);
        if x > 0 {
            out.push(Self(x - 1, y));
        }
        if x < grid.len() - 1 {
            out.push(Self(x + 1, y));
        }
        if y > 0 {
            out.push(Self(x, y - 1));
        }
        if y < grid[0].len() - 1 {
            out.push(Self(x, y + 1));
        }
        out.into_iter()
            .filter(|neighbour| {
                i64::from(self.get_height(grid)) - i64::from(neighbour.get_height(grid)) <= 1
            })
            .collect()
    }

    fn successors(&self, grid: &Grid) -> Vec<Self> {
        let &Self(x, y) = self;
        let mut out = Vec::with_capacity(4);
        if x > 0 {
            out.push(Self(x - 1, y));
        }
        if x < grid.len() - 1 {
            out.push(Self(x + 1, y));
        }
        if y > 0 {
            out.push(Self(x, y - 1));
        }
        if y < grid[0].len() - 1 {
            out.push(Self(x, y + 1));
        }
        out.into_iter()
            .filter(|neighbour| {
                i64::from(neighbour.get_height(grid)) - i64::from(self.get_height(grid)) <= 1
            })
            .collect()
    }

    fn successors_with_cost(&self, grid: &Grid) -> Vec<(Self, usize)> {
        self.successors(grid)
            .iter()
            .map(|&point| (point, 1))
            .collect()
    }

    const fn heuristic(&self, &Self(x_end, y_end): &Self) -> usize {
        let &Self(x, y) = self;
        x_end.abs_diff(x) + y_end.abs_diff(y)
    }
}

fn find_tile(grid: &Grid, target: &Tile) -> Option<Point> {
    grid.iter().enumerate().find_map(|(x, row)| {
        row.iter().enumerate().find_map(|(y, tile)| {
            if *tile == *target {
                Some(Point(x, y))
            } else {
                None
            }
        })
    })
}

pub fn part1(input: &str) -> String {
    let (_, data) = grid(input).unwrap();
    let start = find_tile(&data, &Tile::Start).unwrap();
    let end = find_tile(&data, &Tile::End).unwrap();
    let (_path, cost) = pathfinding::prelude::astar(
        &start,
        |point| point.successors_with_cost(&data),
        |point| point.heuristic(&end),
        |point| *point == end,
    )
    .unwrap();
    cost.to_string()
}

pub fn part2(input: &str) -> String {
    let (_, data) = grid(input).unwrap();
    let end = find_tile(&data, &Tile::End).unwrap();
    let path = pathfinding::prelude::bfs(
        &end,
        |point| point.reversed_successors(&data),
        |point| point.get_height(&data) == u32::from('a'),
    )
    .unwrap();
    (path.len() - 1).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), "31");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), "29");
    }
}
