use aoc_2021::input;
use itertools::Itertools;

fn main() {
    let input = input!("10");
    let input = input.lines().collect_vec();

    let part1: usize = input
        .iter()
        .map(|&x| process_line(x))
        .map(|x| match x {
            Status::Incomplete(_) => 0,
            Status::Corrupted(')') => 3,
            Status::Corrupted(']') => 57,
            Status::Corrupted('}') => 1197,
            Status::Corrupted('>') => 25137,
            _ => unreachable!(),
        })
        .sum();

    let mut incomplete_scores: Vec<_> = input
        .iter()
        .map(|&x| process_line(x))
        .filter_map(|x| match x {
            Status::Corrupted(_) => None,
            Status::Incomplete(rest) => {
                let mut score: usize = 0;
                for char in mirror(&rest) {
                    score *= 5;
                    score += match char {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    }
                }
                Some(score)
            }
        })
        .collect();

    incomplete_scores.sort_unstable();

    dbg!(part1);
    dbg!(incomplete_scores[incomplete_scores.len() / 2]);
}

#[derive(Debug)]
enum Status {
    Incomplete(Vec<char>),
    Corrupted(char),
}

fn rtl(x: char) -> Option<char> {
    match x {
        '}' => Some('{'),
        ']' => Some('['),
        ')' => Some('('),
        '>' => Some('<'),
        _ => None,
    }
}

fn ltr(x: char) -> Option<char> {
    match x {
        '{' => Some('}'),
        '[' => Some(']'),
        '(' => Some(')'),
        '<' => Some('>'),
        _ => None,
    }
}

fn mirror(x: &[char]) -> impl Iterator<Item = char> + '_ {
    x.iter().rev().map(|&x| ltr(x).unwrap())
}

fn process_line(line: &str) -> Status {
    let mut stack = Vec::new();
    for c in line.chars() {
        if let Some(other) = rtl(c) {
            if stack.pop().unwrap() != other {
                return Status::Corrupted(c);
            }
        } else {
            stack.push(c);
        }
    }
    Status::Incomplete(stack)
}
