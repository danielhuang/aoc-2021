#![feature(const_for)]

use std::collections::HashSet;

use aoc_2021::{input, Coordinate3D, Matrix3};
use itertools::Itertools;

fn main() {
    let input = input!("19");

    let input = input
        .split("\n\n")
        .map(|x| {
            x.lines()
                .skip(1)
                .map(|x| {
                    let (x, y, z) = x
                        .split(',')
                        .map(|x| x.parse().unwrap())
                        .collect_tuple()
                        .unwrap();
                    Coordinate3D(x, y, z)
                })
                .collect::<HashSet<_>>()
        })
        .collect_vec();

    let mut ocean = input[0].clone();
    let mut found = HashSet::new();

    while found.len() < input.len() {
        for beacons in input.iter() {
            'a: for target_beacon in ocean.clone() {
                for &beacon in beacons {
                    for rotation in all_rotations() {
                        let translation = beacon - target_beacon;
                        let transformed: HashSet<_> = beacons
                            .iter()
                            .copied()
                            .map(|x| x - beacon)
                            .map(|x| rotation * x)
                            .map(|x| x + beacon)
                            .map(|x| x - translation)
                            .collect();
                        let intersection = transformed.intersection(&ocean).copied().collect_vec();
                        if intersection.len() >= 12 {
                            // println!("found {} beacons", intersection.len());
                            ocean.extend(transformed);
                            let pos =
                                rotation * (Coordinate3D(0, 0, 0) - beacon) + beacon - translation;
                            dbg!(&pos);
                            found.insert(pos);
                            break 'a;
                        }
                    }
                }
            }
            // println!("next");
        }
    }

    // dbg!(&ocean);
    dbg!(&ocean.len());

    let distance = found
        .iter()
        .tuple_combinations()
        .map(|(&a, &b)| b - a)
        .map(|x| x.0.abs() + x.1.abs() + x.2.abs())
        .max()
        .unwrap();
    dbg!(distance);
}

fn all_rotations() -> Vec<Matrix3> {
    let base = [
        Coordinate3D(1, 0, 0),
        Coordinate3D(0, 1, 0),
        Coordinate3D(0, 0, 1),
        Coordinate3D(-1, 0, 0),
        Coordinate3D(0, -1, 0),
        Coordinate3D(0, 0, -1),
    ];
    let mut result = vec![];
    for &v1 in &base {
        for &v2 in &base {
            if v1 != v2 && v1 * -1 != v2 {
                let Coordinate3D(a, b, c) = v1;
                let Coordinate3D(x, y, z) = v2;
                let cross = Coordinate3D(b * z - c * y, c * x - a * z, a * y - b * x);
                result.push(Matrix3 {
                    cols: [v1, v2, cross],
                })
            }
        }
    }
    assert_eq!(result.len(), 24);
    result
}
