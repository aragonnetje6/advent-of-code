// use std::collections::{BinaryHeap, HashSet};
//
// fn parse(input: &str) -> Vec<Vec<u8>> {
//     input
//         .lines()
//         .map(|line| {
//             line.chars()
//                 .map(|c| c.to_string().parse().unwrap())
//                 .collect()
//         })
//         .collect()
// }
//
// type Point = (usize, usize);
// type CostPoint = (u64, usize, usize);
//
// fn dijkstra(grid: &Vec<Vec<u8>>) -> Vec<Point> {
//     let mut queue = BinaryHeap::new();
//     let mut evaluated = HashSet::new();
//     queue.push((1, 0, 0));
//     loop {
//         queue.pop()
//     }
// }
//
// fn dijkstra_evaluate(grid: &Vec<Vec<u8>>, entered_from: CostPoint, target: CostPoint) -> Vec<CostPoint> {
//     [(target.1+1, target.2), (target.1-1, target.2), (target.1, target.2+1), (target.1, target.2-1)].iter().filter(|(x, y)| x != entered_from.1 || y != entered_from.2 ).map(|(x, y)| (grid[x][y] as u64 + target.0, x, y))
// }
//
// pub fn part1(input: &str) -> u64 {
//     let data = parse(input);
// }
// pub fn part2(input: &str) -> u64 {
//     todo!();
// }
//
// #[cfg(test)]
// mod test {
//     use super::*;
//
//     const DATA1: &str = r"1163751742
// 1381373672
// 2136511328
// 3694931569
// 7463417111
// 1319128137
// 1359912421
// 3125421639
// 1293138521
// 2311944581
// ";
//
//     #[test]
//     fn test_part1() {
//         assert_eq!(part1(DATA1), 21);
//     }
//
//     #[test]
//     fn test_part2() {
//         assert_eq!(part2(DATA1), 8);
//     }
// }
