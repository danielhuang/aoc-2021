use std::collections::HashMap;

use aoc_2021::input;
use defaultmap::DefaultHashMap;
use itertools::Itertools;

fn main() {
    let input = input!("14");

    let template = input.lines().next().unwrap().to_string();

    let rules: HashMap<_, _> = input
        .lines()
        .skip(2)
        .map(|x| x.split(" -> ").collect_tuple().unwrap())
        .collect();

    let mut counts = DefaultHashMap::new(0);
    for (a, b) in template.chars().tuple_windows() {
        counts[(a, b)] += 1;
    }

    for _ in 0..40 {
        counts = tick(&counts, &rules);
    }

    let mut quantities = DefaultHashMap::new(0);
    for (&(a, b), count) in counts.iter() {
        quantities[a] += count;
        quantities[b] += count;
    }
    quantities[template.chars().next().unwrap()] += 1;
    quantities[template.chars().last().unwrap()] += 1;

    for (_, count) in quantities.iter_mut() {
        assert!(*count % 2 == 0, "{}", count);
        *count /= 2;
    }

    let (min, max) = quantities
        .iter()
        .map(|(_, &x)| x)
        .minmax()
        .into_option()
        .unwrap();

    dbg!(max - min);
}

fn tick(
    counts: &DefaultHashMap<(char, char), usize>,
    rules: &HashMap<&'static str, &'static str>,
) -> DefaultHashMap<(char, char), usize> {
    let mut result = DefaultHashMap::new(0);

    for (&(l, r), &count) in counts.iter() {
        let m = rules[[l, r].into_iter().collect::<String>().as_str()]
            .chars()
            .exactly_one()
            .unwrap();
        result[(l, m)] += count;
        result[(m, r)] += count;
    }

    result
}
