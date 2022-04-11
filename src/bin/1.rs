use aoc_2021::input;
use itertools::Itertools;

fn main() {
    let input = input!("1");

    let input = input.trim().lines().map(|x| x.parse::<i32>().unwrap());

    dbg!(input
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count());
}
