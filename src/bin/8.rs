use std::collections::{HashMap, HashSet};

use aoc_2021::input;
use itertools::Itertools;

fn main() {
    let input = input!("8");
    let input: Vec<(_, _)> = input
        .lines()
        .map(|x| {
            x.split(" | ")
                .map(|x| x.split_whitespace().collect_vec())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    dbg!(&input);

    dbg!(input
        .iter()
        .map(|x| &x.1)
        .flatten()
        .filter(|x| { matches!(x.len(), 2 | 4 | 3 | 7) })
        .count());

    let lookup_table: HashMap<_, _> = [
        ("abcefg", 0),
        ("cf", 1),
        ("acdeg", 2),
        ("acdfg", 3),
        ("bcdf", 4),
        ("abdfg", 5),
        ("abdefg", 6),
        ("acf", 7),
        ("abcdefg", 8),
        ("abcdfg", 9),
    ]
    .into_iter()
    .collect();

    let mut total = 0;

    for (patterns, output) in input.iter() {
        for permutation in "abcdefg".chars().permutations(7) {
            let permuted_map: HashMap<_, _> =
                permutation.into_iter().zip("abcdefg".chars()).collect();

            let permuted_numbers: Vec<i32> =
                get_numbers(patterns, &lookup_table, &permuted_map).collect_vec();

            if permuted_numbers.iter().collect::<HashSet<_>>().len() == 10 {
                let permuted_output: String = get_numbers(output, &lookup_table, &permuted_map)
                    .map(|x| x.to_string())
                    .collect();
                total += permuted_output.parse::<i64>().unwrap();
            }
        }
    }
    dbg!(total);
}

fn get_numbers<'a>(
    x: &'a [&'a str],
    lookup_table: &'a HashMap<&'static str, i32>,
    permuted_map: &'a HashMap<char, char>,
) -> impl Iterator<Item = i32> + 'a {
    x.iter()
        .map(move |x| {
            x.chars()
                .map(|x| permuted_map[&x])
                .sorted()
                .collect::<String>()
        })
        .filter_map(move |x| lookup_table.get(x.as_str()))
        .copied()
}
