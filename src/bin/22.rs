use std::{
    collections::HashSet,
    ops::{Range},
};

use aoc_2021::{input, Coordinate3D};
use itertools::Itertools;
use lazy_regex::regex_captures;
use rustc_hash::FxHashSet;


fn main() {
    let input = input!("22");
    let input=  input.lines().map(|x| {
        let (_, state, x_min, x_max, y_min, y_max, z_min, z_max) = regex_captures!(r#"(on|off) x=(-?[0-9]+)\.\.(-?[0-9]+),y=(-?[0-9]+)\.\.(-?[0-9]+),z=(-?[0-9]+)\.\.(-?[0-9]+)"#, x).unwrap();

        (state == "on", Cuboid { min_x: x_min.parse().unwrap(), max_x: x_max.parse().unwrap(), min_y: y_min.parse().unwrap(), max_y: y_max.parse().unwrap(), min_z: z_min.parse().unwrap(), max_z: z_max.parse().unwrap() })

    }).collect_vec();
    dbg!(&input);

    let mut field = HashSet::new();
    for &(on, cuboid) in &input {
        if cuboid.max() <= 50 {
            for point in cuboid.enumerate() {
                if on {
                    field.insert(point);
                } else {
                    field.remove(&point);
                }
            }
        }
    }

    dbg!(field.len());

    let critical_x = find_critical(&input, |x| [x.min_x, x.max_x + 1]);
    let critical_y = find_critical(&input, |x| [x.min_y, x.max_y + 1]);
    let critical_z = find_critical(&input, |x| [x.min_z, x.max_z + 1]);

    dbg!(&critical_x, &critical_y, &critical_z);

    let mut compressed_grid = FxHashSet::default();
    for &(on, cuboid) in &input {
        dbg!(&cuboid);
        for compressed_x in compress_range(&critical_x, cuboid.min_x, cuboid.max_x + 1) {
            for compressed_y in compress_range(&critical_y, cuboid.min_y, cuboid.max_y + 1) {
                for compressed_z in compress_range(&critical_z, cuboid.min_z, cuboid.max_z + 1) {
                    if on {
                        compressed_grid.insert((compressed_x, compressed_y, compressed_z));
                    } else {
                        compressed_grid.remove(&(compressed_x, compressed_y, compressed_z));
                    }
                }
            }
        }
    }

    let mut vol = 0;
    dbg!(compressed_grid.len());
    for (compressed_x, compressed_y, compressed_z) in compressed_grid {
        let len_x = critical_x[compressed_x + 1] - critical_x[compressed_x];
        let len_y = critical_y[compressed_y + 1] - critical_y[compressed_y];
        let len_z = critical_z[compressed_z + 1] - critical_z[compressed_z];

        vol += len_x * len_y * len_z;
    }
    dbg!(&vol);
}

fn find_critical<I: IntoIterator<Item = i64>>(
    input: &[(bool, Cuboid)],
    mut f: impl FnMut(Cuboid) -> I,
) -> Vec<i64> {
    input
        .iter()
        .copied()
        .flat_map(|(_, x)| f(x))
        .sorted()
        .dedup()
        .collect()
}

fn compress_range(critical: &[i64], min: i64, max: i64) -> Range<usize> {
    let c_min = critical.binary_search(&min).unwrap();
    let c_max = critical.binary_search(&max).unwrap();

    c_min..c_max
}

#[derive(Debug, Clone, Copy)]
struct Cuboid {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
}

impl Cuboid {
    fn enumerate(&self) -> impl Iterator<Item = Coordinate3D> {
        (self.min_x..=self.max_x)
            .cartesian_product(self.min_y..=self.max_y)
            .cartesian_product(self.min_z..=self.max_z)
            .map(|((x, y), z)| Coordinate3D(x, y, z))
    }

    fn max(&self) -> i64 {
        [
            self.min_x, self.max_x, self.min_y, self.max_y, self.min_z, self.max_z,
        ]
        .into_iter()
        .map(|x| x.abs())
        .max()
        .unwrap()
    }
}
