use cached::proc_macro::cached;
use std::cmp::{max, min};

use aoc_2021::input;
use itertools::Itertools;

fn main() {
    let input = input!("21");
    let (a, b) = input
        .lines()
        .map(|x| x.split_once(": ").unwrap().1.parse::<usize>().unwrap() - 1)
        .collect_tuple()
        .unwrap();

    dbg!(part1(a, b));
    dbg!(part2(a, b, 21, 21));
}

fn part1(mut a: usize, mut b: usize) -> usize {
    let mut dice = Dice::default();
    dbg!(a, b);
    let mut score_a = 0;
    let mut score_b = 0;
    while max(score_a, score_b) < 1000 {
        let roll_a = dice.roll3();
        a = (a + roll_a) % 10;
        score_a += a + 1;

        if score_a >= 1000 {
            break;
        }

        let roll_b = dice.roll3();
        b = (b + roll_b) % 10;
        score_b += b + 1;
    }
    dbg!(a, b, score_a, score_b);
    let losing_score = min(score_a, score_b);
    losing_score * dice.rolls
}

#[cached]
fn part2(a: usize, b: usize, remaining_a: isize, remaining_b: isize) -> (usize, usize) {
    if remaining_a <= 0 {
        return (1, 0);
    }

    if remaining_b <= 0 {
        return (0, 1);
    }

    let mut count_a = 0;
    let mut count_b = 0;

    for roll_a in all_roll3() {
        let a = (a + roll_a) % 10;
        let remaining_a = remaining_a - (a + 1) as isize;

        if remaining_a <= 0 {
            count_a += 1;
        } else {
            for roll_b in all_roll3() {
                let b = (b + roll_b) % 10;
                let remaining_b = remaining_b - (b + 1) as isize;

                let (new_a, new_b) = part2(a, b, remaining_a, remaining_b);
                count_a += new_a;
                count_b += new_b;
            }
        }
    }

    (count_a, count_b)
}

fn all_roll3() -> impl Iterator<Item = usize> {
    (1..=3)
        .cartesian_product(1..=3)
        .cartesian_product(1..=3)
        .map(|((a, b), c)| a + b + c)
}

#[derive(Default)]
struct Dice {
    cur: usize,
    rolls: usize,
}

impl Dice {
    fn roll(&mut self) -> usize {
        self.cur += 1;
        if self.cur > 100 {
            self.cur = 1;
        }
        self.rolls += 1;
        self.cur
    }

    fn roll3(&mut self) -> usize {
        let a = self.roll();
        let b = self.roll();
        let c = self.roll();
        a + b + c
    }
}
