use std::collections::HashMap;

use aoc_2021::{input, Coordinate2D};
use pathfinding::prelude::dijkstra;

fn main() {
    let input = input!("15");

    let mut grid: HashMap<Coordinate2D, i64> = HashMap::new();
    for (y, s) in input.lines().enumerate() {
        for (x, c) in s.chars().enumerate() {
            grid.insert(
                Coordinate2D(x as _, y as _),
                c.to_string().parse::<i64>().unwrap(),
            );
        }
    }

    dbg!(solve(&grid));

    let mut big_grid = HashMap::new();
    let Coordinate2D(size_x, size_y) = get_target(&grid) + Coordinate2D(1, 1);
    for i in 0..5 {
        for j in 0..5 {
            for (&Coordinate2D(x, y), &val) in grid.iter() {
                big_grid.insert(
                    Coordinate2D(x + i * size_x, y + j * size_y),
                    wrap(val + i + j),
                );
            }
        }
    }

    dbg!(solve(&big_grid));
}

fn solve(grid: &HashMap<Coordinate2D, i64>) -> i64 {
    let target = get_target(grid);

    dijkstra(
        &Coordinate2D(0, 0),
        |x| {
            x.adjacent()
                .into_iter()
                .filter_map(|c| grid.get(&c).map(|&x| (c, x)))
        },
        |&x| x == target,
    )
    .unwrap()
    .1
}

fn get_target(grid: &HashMap<Coordinate2D, i64>) -> Coordinate2D {
    let target = Coordinate2D(
        grid.keys().map(|x| x.0).max().unwrap(),
        grid.keys().map(|x| x.1).max().unwrap(),
    );
    target
}

fn wrap(x: i64) -> i64 {
    (x - 1) % 9 + 1
}
