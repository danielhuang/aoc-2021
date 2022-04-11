use std::collections::HashSet;

use aoc_2021::{input, print_hashset, Coordinate2D};
use itertools::Itertools;

fn main() {
    let input = input!("13");

    let (points, folds) = input.split("\n\n").collect_tuple().unwrap();
    let mut points: HashSet<_> = points
        .lines()
        .map(|x| {
            x.split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .map(|(x, y)| Coordinate2D(x, y))
        .collect();
    let folds: Vec<(_, i64)> = folds
        .lines()
        .map(|x| x.strip_prefix("fold along ").unwrap())
        .map(|x| x.split('=').collect_tuple().unwrap())
        .map(|(a, b)| (a == "y", b.parse().unwrap()))
        .collect();

    for f in folds {
        points = fold(&points, f);
    }

    print_hashset(&points);
}

fn fold(x: &HashSet<Coordinate2D>, fold: (bool, i64)) -> HashSet<Coordinate2D> {
    let (is_y, fold_location) = fold;
    x.iter()
        .copied()
        .map(|Coordinate2D(x, y)| {
            if is_y {
                Coordinate2D(x, -(y - fold_location).abs() + fold_location)
            } else {
                Coordinate2D(-(x - fold_location).abs() + fold_location, y)
            }
        })
        .collect()
}
