use std::ops::{Range, RangeInclusive};

use aoc_2021::{input, Coordinate2D};
use itertools::Itertools;
use regex::Regex;
use stringtools::Stringtools;

fn main() {
    let input = input!("17");
    let [lx, hx, ly, hy] = input
        .match_regex(
            &Regex::new("target area: x=(-?[0-9]+)..(-?[0-9]+), y=(-?[0-9]+)..(-?[0-9]+)").unwrap(),
        )
        .unwrap()
        .map(|x| x.parse().unwrap());

    let data = (0..1000)
        .cartesian_product(0..1000)
        .filter_map(|x| {
            let x = Coordinate2D(x.0, x.1);
            calc(x, lx, ly, hx, hy)
        })
        .max();

    dbg!(&data);

    let part2 = (-1000..1000)
        .cartesian_product(-1000..1000)
        .filter(|x| {
            let x = Coordinate2D(x.0, x.1);
            calc(x, lx, ly, hx, hy).is_some()
        })
        .inspect(|x| {
            dbg!(&x);
        })
        .count();

    dbg!(&part2);
}

fn calc(mut velocity: Coordinate2D, lx: i64, ly: i64, hx: i64, hy: i64) -> Option<i64> {
    let mut pos = Coordinate2D(0, 0);
    let mut highest = 0;
    while !check_bound(pos, lx, ly, hx, hy) {
        if pos.1 > highest {
            // dbg!(&highest, &pos.1);
            highest = pos.1;
        }
        pos += velocity;
        match velocity.0.cmp(&0) {
            std::cmp::Ordering::Less => velocity.0 += 1,
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => velocity.0 -= 1,
        }
        velocity.1 -= 1;
        if pos.1 < ly {
            return None;
        }
    }
    Some(highest)
}

fn check_bound(point: Coordinate2D, lx: i64, ly: i64, hx: i64, hy: i64) -> bool {
    let Coordinate2D(x, y) = point;
    // dbg!(x, y, lx, ly, hx, hy);
    (lx..=hx).contains(&x) && (ly..=hy).contains(&y)
}
