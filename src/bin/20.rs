use std::collections::HashSet;

use aoc_2021::{input, print_hashset, Coordinate2D};
use itertools::Itertools;

fn main() {
    let input = input!("20");
    let (convolver, image) = input.split_once("\n\n").unwrap();

    dbg!(&convolver);

    let convolver = convolver
        .chars()
        .map(|x| match x {
            '.' => false,
            '#' => true,
            _ => unreachable!(),
        })
        .collect_vec();

    let image: HashSet<_> = image
        .lines()
        .enumerate()
        .flat_map(|(y, s)| {
            s.chars().enumerate().filter_map(move |(x, c)| {
                (match c {
                    '.' => false,
                    '#' => true,
                    _ => unreachable!(),
                })
                .then(|| Coordinate2D(x as _, y as _))
            })
        })
        .collect();

    let convolved = convolve_twice(&image, &convolver);
    print_hashset(&image);
    println!();
    print_hashset(&convolved);
    dbg!(convolved.len());

    let mut convolved = image;
    for i in 0..(50 / 2) {
        convolved = convolve_twice(&convolved, &convolver);
        dbg!(i);
    }
    dbg!(convolved.len());
}

fn convolve_single(
    c: Coordinate2D,
    mut get: impl FnMut(Coordinate2D) -> bool,
    convolver: &[bool],
) -> bool {
    let mut s = String::new();
    for y in -1..=1 {
        for x in -1..=1 {
            s.push(if get(c + Coordinate2D(x, y)) {
                '1'
            } else {
                '0'
            });
        }
    }
    let i = usize::from_str_radix(&s, 2).unwrap();
    convolver[i]
}

fn convolve_twice(x: &HashSet<Coordinate2D>, convolver: &[bool]) -> HashSet<Coordinate2D> {
    let mut must_visit = HashSet::new();
    must_visit.extend(x.iter().copied());
    for _ in 0..3 {
        for c in must_visit.clone() {
            must_visit.extend(c.adjacent_corners());
        }
    }

    let mut result = HashSet::new();
    for new_location in must_visit {
        let data = convolve_single(
            new_location,
            |c| convolve_single(c, |c| x.contains(&c), convolver),
            convolver,
        );
        if data {
            result.insert(new_location);
        }
    }

    result
}
