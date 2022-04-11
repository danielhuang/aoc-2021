#![feature(destructuring_assignment)]

use aoc_2021::{input, Coordinate2D};
use defaultmap::DefaultHashMap;
use itertools::Itertools;

fn main() {
    let input = input!("5");

    let input: Vec<(i64, i64, i64, i64)> = input
        .lines()
        .map(|x| {
            x.split(" -> ")
                .map(|x| x.split(','))
                .flatten()
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    let mut grid = DefaultHashMap::new(0);
    let mut grid2 = DefaultHashMap::new(0);

    for &(mut x1, mut y1, mut x2, mut y2) in input.iter() {
        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                grid[Coordinate2D(x1, y)] += 1;
                grid2[Coordinate2D(x1, y)] += 1;
            }
        }
        if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                grid[Coordinate2D(x, y1)] += 1;
                grid2[Coordinate2D(x, y1)] += 1;
            }
        }
        if y2 < y1 {
            (x2, x1) = (x1, x2);
            (y2, y1) = (y1, y2);
        }
        if y2 - y1 == x2 - x1 {
            for i in 0..=(y2 - y1) {
                grid2[Coordinate2D(x1 + i, y1 + i)] += 1;
            }
        }
        if y2 - y1 == x1 - x2 {
            for i in 0..=(y2 - y1) {
                grid2[Coordinate2D(x1 - i, y1 + i)] += 1;
            }
        }
    }

    dbg!(grid.values().filter(|&&x| x > 1).count());
    dbg!(grid2.values().filter(|&&x| x > 1).count());
}
