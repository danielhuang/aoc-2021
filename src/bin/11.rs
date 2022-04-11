use std::collections::{HashMap, HashSet};

use aoc_2021::{input, print_hashmap, Coordinate2D};

fn main() {
    let input = input!("11");

    let mut grid1: HashMap<Coordinate2D, i64> = HashMap::new();
    for (y, s) in input.lines().enumerate() {
        for (x, c) in s.chars().enumerate() {
            grid1.insert(
                Coordinate2D(x as _, y as _),
                c.to_string().parse::<i64>().unwrap(),
            );
        }
    }

    let mut grid = grid1.clone();

    let mut count = 0;
    for i in 0..100 {
        println!("{}", i);
        print_hashmap(&grid);
        let t = tick(&mut grid);
        count += t;
    }
    dbg!(count);

    let mut grid = grid1.clone();
    print_hashmap(&grid);

    for i in 0.. {
        let t = tick(&mut grid);
        count += t;
        if t == grid.values().count() {
            dbg!(i + 1);
            break;
        }
    }
}

fn tick(result: &mut HashMap<Coordinate2D, i64>) -> usize {
    for (_, value) in result.iter_mut() {
        *value += 1;
    }

    let mut flashed = HashSet::new();

    while let Some((&c, _)) = result
        .iter()
        .find(|&(&c, &x)| !flashed.contains(&c) && x > 9)
    {
        flashed.insert(c);
        for adj in c.adjacent_corners() {
            if !flashed.contains(&adj) {
                if let Some(flash_target) = result.get_mut(&adj) {
                    *flash_target += 1;
                }
            }
        }
    }

    for &c in &flashed {
        result.insert(c, 0);
    }

    flashed.len()
}
