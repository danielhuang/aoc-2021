use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    mem::take,
};

use aoc_2021::{input, Coordinate2D};

fn main() {
    let input = input!("9");

    let mut grid = HashMap::new();
    for (y, s) in input.lines().enumerate() {
        for (x, c) in s.chars().enumerate() {
            grid.insert(
                Coordinate2D(x as _, y as _),
                c.to_string().parse::<i64>().unwrap(),
            );
        }
    }

    let mut low_points = HashSet::new();

    let mut count = 0;
    for (&c, &v) in grid.iter() {
        if c.adjacent()
            .into_iter()
            .all(|c2| grid.get(&c2).is_none() || grid.get(&c2).copied().unwrap() > v)
        {
            low_points.insert(c);
            count += v + 1;
        }
    }

    let mut basins = vec![];

    for &point in &low_points {
        let mut found: HashSet<Coordinate2D> = HashSet::new();
        let mut pending = HashSet::new();
        pending.insert(point);
        while !pending.is_empty() {
            found.extend(&pending);
            for p in take(&mut pending) {
                for adj in p.adjacent() {
                    if !found.contains(&adj) {
                        if let Some(&val) = grid.get(&adj) {
                            if val < 9 {
                                pending.insert(adj);
                            }
                        }
                    }
                }
            }
        }
        basins.push(found);
    }

    dbg!(count);

    basins.sort_by_key(|x| Reverse(x.len()));

    dbg!(&basins.iter().take(3).map(|x| x.len()).product::<usize>());
}
