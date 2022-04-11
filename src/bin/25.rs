use std::mem::replace;

use aoc_2021::{input, Coordinate2D};
use defaultmap::DefaultHashMap;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Down,
    Right,
}

fn main() {
    let input = input!("25");

    let mut grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'v' => Tile::Down,
                    '>' => Tile::Right,
                    '.' => Tile::Empty,
                    _ => unimplemented!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let mut count = 1;
    // dbg!(&grid);
    // while grid != tick2(&tick1(&grid)) {
    //     grid = tick2(&tick1(&grid));
    //     count += 1;
    // }
    while {
        let grid2 = tick2(&tick1(&grid));
        let result = grid2 != grid;
        grid = grid2;
        result
    } {
        count += 1;
    }
    dbg!(count);
}

fn print(grid: &[Vec<Tile>]) {
    for line in grid {
        for c in line {
            match c {
                Tile::Empty => print!("."),
                Tile::Down => print!("v"),
                Tile::Right => print!(">"),
            }
        }
        println!();
    }
    println!();
}

fn tick1(grid: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    let mut new_grid = grid.to_vec();
    for (i, line) in grid.iter().enumerate() {
        let i = i as isize;
        for (j, &item) in line.iter().enumerate() {
            let j = j as isize;
            match item {
                Tile::Empty => {}
                Tile::Down => {}
                Tile::Right => {
                    if grid[(i % (grid.len() as isize)) as usize]
                        [((j + 1) % (grid[0].len() as isize)) as usize]
                        == Tile::Empty
                    {
                        new_grid[i as usize][j as usize] = Tile::Empty;
                        new_grid[(i % (grid.len() as isize)) as usize]
                            [((j + 1) % (grid[0].len() as isize)) as usize] = item;
                    }
                }
            }
        }
    }
    new_grid
}

fn tick2(grid: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    let mut new_grid = grid.to_vec();
    for (i, line) in grid.iter().enumerate() {
        let i = i as isize;
        for (j, &item) in line.iter().enumerate() {
            let j = j as isize;
            match item {
                Tile::Empty => {}
                Tile::Down => {
                    if grid[((i + 1) % (grid.len() as isize)) as usize]
                        [(j % (grid[0].len() as isize)) as usize]
                        == Tile::Empty
                    {
                        new_grid[i as usize][j as usize] = Tile::Empty;
                        new_grid[((i + 1) % (grid.len() as isize)) as usize]
                            [(j % (grid[0].len() as isize)) as usize] = item;
                    }
                }
                Tile::Right => {}
            }
        }
    }
    new_grid
}
