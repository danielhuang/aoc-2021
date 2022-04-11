#![feature(box_syntax, box_patterns)]

use std::{fmt::Debug, ops::Add};

use aoc_2021::input;
use itertools::Itertools;
use serde_json::Value;

fn main() {
    let input = input!("18");
    let input = input
        .lines()
        .map(parse)
        .map(|x| x.to_snailfish_number(0))
        .collect_vec();

    let data = input.iter().cloned().reduce(|a, b| a + b).unwrap();

    dbg!(&data);
    dbg!(data.magnitude());

    let part2 = input
        .iter()
        .cloned()
        .tuple_combinations()
        .map(|(a, b)| (a.clone() + b.clone()).magnitude().max((b + a).magnitude()))
        .max()
        .unwrap();

    dbg!(part2);
}

enum Tree {
    Num(i64),
    Pair(Box<Tree>, Box<Tree>),
}

impl Tree {
    fn to_snailfish_number(&self, depth: usize) -> SnailfishNumber {
        let mut result = vec![];
        match self {
            Tree::Num(x) => {
                result.push((depth, *x));
            }
            Tree::Pair(box a, box b) => {
                result.extend(a.to_snailfish_number(depth + 1).data);
                result.extend(b.to_snailfish_number(depth + 1).data);
            }
        };
        SnailfishNumber { data: result }
    }
}

#[derive(Clone)]
struct SnailfishNumber {
    data: Vec<(usize, i64)>,
}

impl Debug for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "snail values only: ")?;
        write!(
            f,
            "{}",
            self.data
                .iter()
                .map(|x| x.1.to_string())
                .collect_vec()
                .join(",")
        )?;

        Ok(())
    }
}

impl SnailfishNumber {
    fn explode(&mut self) -> bool {
        for i in 0..self.data.len() {
            if self.data[i].0 > 4 {
                let a = self.data[i];
                let b = self.data[i + 1];
                assert_eq!(a.0, b.0);
                if i > 0 {
                    if let Some(x) = self.data.get_mut(i - 1) {
                        x.1 += a.1;
                    }
                }
                if let Some(x) = self.data.get_mut(i + 2) {
                    x.1 += b.1;
                }
                self.data[i].0 -= 1;
                self.data[i].1 = 0;
                self.data.remove(i + 1);
                return true;
            }
        }
        false
    }

    fn split(&mut self) -> bool {
        for i in 0..self.data.len() {
            if self.data[i].1 >= 10 {
                let a = self.data[i];
                self.data[i].0 += 1;
                self.data[i].1 = a.1 / 2;
                self.data.insert(i + 1, (a.0 + 1, a.1 / 2));
                if a.1 % 2 == 1 {
                    self.data[i + 1].1 += 1;
                }
                return true;
            }
        }
        false
    }

    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    fn magnitude(mut self) -> i64 {
        while self.data.len() > 1 {
            for i in 0..(self.data.len() - 1) {
                if self.data[i].0 == self.data[i + 1].0 {
                    let mag = self.data[i].1 * 3 + self.data[i + 1].1 * 2;
                    self.data[i].1 = mag;
                    self.data[i].0 -= 1;
                    self.data.remove(i + 1);
                    break;
                }
            }
        }
        assert_eq!(self.data[0].0, 0);
        self.data[0].1
    }
}

impl Add for SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let data = self
            .data
            .into_iter()
            .map(|x| (x.0 + 1, x.1))
            .chain(rhs.data.into_iter().map(|x| (x.0 + 1, x.1)))
            .collect();
        let mut result = Self { data };
        result.reduce();
        result
    }
}

impl Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(arg0) => write!(f, "{}", arg0),
            Self::Pair(arg0, arg1) => write!(f, "[{:?},{:?}]", arg0, arg1),
        }
    }
}

fn json_to_snail(x: &Value) -> Tree {
    match x {
        Value::Number(x) => Tree::Num(x.as_i64().unwrap()),
        Value::Array(x) => {
            if let [a, b] = x.as_slice() {
                Tree::Pair(box json_to_snail(a), box json_to_snail(b))
            } else {
                unreachable!()
            }
        }
        _ => unreachable!(),
    }
}

fn parse(x: &str) -> Tree {
    json_to_snail(&serde_json::from_str(x).unwrap())
}
