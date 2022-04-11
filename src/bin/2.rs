use aoc_2021::{input, Coordinate2D};
use itertools::Itertools;

fn main() {
    let input = input!("2");
    let input = input
        .trim()
        .lines()
        .map(|x| x.split(' ').collect_tuple().unwrap())
        .map(|(a, b)| (a, b.parse::<i64>().unwrap()));

    let mut aim = 0;
    let mut pos = Coordinate2D(0, 0);
    for (item, num) in input {
        match item {
            "forward" => {
                pos.0 += num;
                pos.1 += num * aim;
            }
            "up" => {
                aim += num;
            }
            "down" => {
                aim -= num;
            }
            _ => unreachable!(),
        }
    }

    dbg!(pos.0 * pos.1);
}
