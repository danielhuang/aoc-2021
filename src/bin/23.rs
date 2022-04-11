use std::collections::{HashMap, HashSet};

use aoc_2021::{input, print_grid, Coordinate2D};
use cached::proc_macro::cached;
use defaultmap::DefaultHashMap;
use pathfinding::prelude::dijkstra;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
enum Tile {
    Wall,
    Path,
    Amphipod(char),
}

const DEBUG: bool = false;

fn main() {
    let input = input!("23");

    let grid = parse(input);

    let template = parse(
        "#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########",
    );

    // dbg!(search(&grid, &HashSet::new(), &HashMap::new()));
    let mut count = 0;

    let result = dijkstra(
        &grid,
        move |x| {
            let x = x.clone();
            // print(&x);
            count += 1;
            if count % 10000 == 0 || DEBUG {
                dbg!(count);
                print(&x);
            }
            all_moves(&x).into_iter().map(move |(a, b, cost)| {
                let at = x[a];
                let bt = x[b];
                let mut x2 = x.clone();
                x2[b] = at;
                x2[a] = bt;
                (x2, cost)
            })
        },
        |x| x == &template,
    )
    .unwrap();

    dbg!(&result);

    for x in result.0 {
        print(&x);
        println!();
    }

    dbg!(result.1);
}

fn parse(input: &str) -> DefaultHashMap<Coordinate2D, Tile> {
    let mut grid = DefaultHashMap::new(Tile::Wall);
    for (y, s) in input.lines().enumerate() {
        for (x, c) in s.chars().enumerate() {
            grid[Coordinate2D(x as _, y as _)] = match c {
                '#' | ' ' => Tile::Wall,
                '.' => Tile::Path,
                'A'..='D' => Tile::Amphipod(c),
                _ => unreachable!(),
            }
        }
    }
    grid
}

fn distance(
    x: &DefaultHashMap<Coordinate2D, Tile>,
    src: Coordinate2D,
    dest: Coordinate2D,
) -> Option<usize> {
    dijkstra(
        &src,
        |c| {
            c.adjacent()
                .into_iter()
                .filter(|&c2| x[c2] == Tile::Path)
                .map(|c2| (c2, 1))
        },
        |&c| c == dest,
    )
    .map(|x| x.1)
}

fn all_moves(
    grid: &DefaultHashMap<Coordinate2D, Tile>,
) -> Vec<(Coordinate2D, Coordinate2D, usize)> {
    let mut result = vec![];
    for (&pos, &tile) in grid.iter() {
        if let Tile::Amphipod(t) = tile {
            let all = vec![
                Coordinate2D(1, 1),
                Coordinate2D(2, 1),
                Coordinate2D(4, 1),
                Coordinate2D(6, 1),
                Coordinate2D(8, 1),
                Coordinate2D(10, 1),
                Coordinate2D(11, 1),
            ];
            let mut targets = match tile {
                Tile::Amphipod('A') => vec![
                    Coordinate2D(3, 2),
                    Coordinate2D(3, 3),
                    Coordinate2D(3, 4),
                    Coordinate2D(3, 5),
                ],
                Tile::Amphipod('B') => vec![
                    Coordinate2D(5, 2),
                    Coordinate2D(5, 3),
                    Coordinate2D(5, 4),
                    Coordinate2D(5, 5),
                ],
                Tile::Amphipod('C') => vec![
                    Coordinate2D(7, 2),
                    Coordinate2D(7, 3),
                    Coordinate2D(7, 4),
                    Coordinate2D(7, 5),
                ],
                Tile::Amphipod('D') => vec![
                    Coordinate2D(9, 2),
                    Coordinate2D(9, 3),
                    Coordinate2D(9, 4),
                    Coordinate2D(9, 5),
                ],
                _ => continue,
            };

            // if targets.get(3).copied() == Some(pos) {
            //     continue;
            // }
            // if pos.1 == 1 {
            //     // in hallway
            //     if distance(grid, pos, targets[1]).is_none() && grid[targets[1]] == Tile::Path {
            //         continue;
            //     }
            // }
            if pos.1 == 1 && (grid[targets[3]] != Tile::Path && grid[targets[3]] != tile) {
                targets = vec![];
            }

            if targets.iter().copied().all(|x| grid[x] == tile) {
                // println!("opt===================================");
                // dbg!(targets, tile);
                // print(&grid);
                // println!();
                continue;
            }

            if targets
                .iter()
                .copied()
                .any(|x| grid[x] != Tile::Path && grid[x] != tile)
            {
                // println!("ops");
                // dbg!(&targets);
                targets = vec![];
            }

            if targets.contains(&pos) {
                targets = vec![];
            }

            let mut final_target = None;
            for &target in &targets {
                if grid[target] == Tile::Path {
                    final_target = Some(target);
                }
            }
            if let Some(t) = final_target {
                targets = vec![t];
            } else {
                targets = vec![];
            }

            if let Some(target) = targets.get(1).copied() {
                if grid[target] == Tile::Path {
                    targets = vec![target];
                }
            }
            let combined = if pos.1 == 1 {
                targets
            } else {
                [all, targets].concat()
            };
            result.extend(
                combined
                    .into_iter()
                    .filter_map(|c| distance(grid, pos, c).map(|d| (c, d)))
                    .filter(|&(c, _)| pos != c)
                    .map(|(c, d)| {
                        (
                            pos,
                            c,
                            d * match t {
                                'A' => 1,
                                'B' => 10,
                                'C' => 100,
                                'D' => 1000,
                                _ => unreachable!(),
                            },
                        )
                    }),
            );
        }
    }
    result
}

fn search(
    x: &DefaultHashMap<Coordinate2D, Tile>,
    seen: &HashSet<DefaultHashMap<Coordinate2D, Tile>>,
) -> usize {
    let template = parse(
        "#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########",
    );
    print(x);
    dbg!(seen.len());
    println!();
    let mut seen2 = seen.clone();
    if x == &template {
        return 0;
    }
    let mut min = usize::MAX / 2;
    if seen2.contains(x) {
        return min;
    }
    assert!(seen2.len() < 10000);
    for (a, b, _) in all_moves(x) {
        let at = x[a];
        let bt = x[b];
        if bt != Tile::Amphipod('A')
            && (b == Coordinate2D(3, 2) || b == Coordinate2D(3, 3))
            && !(b == Coordinate2D(3, 2) && a == Coordinate2D(3, 3))
        {
            continue;
        }
        if bt != Tile::Amphipod('B')
            && (b == Coordinate2D(5, 2) || b == Coordinate2D(5, 3))
            && !(b == Coordinate2D(5, 2) && a == Coordinate2D(5, 3))
        {
            continue;
        }
        if bt != Tile::Amphipod('C')
            && (b == Coordinate2D(7, 2) || b == Coordinate2D(7, 3))
            && !(b == Coordinate2D(7, 2) && a == Coordinate2D(7, 3))
        {
            continue;
        }
        if bt != Tile::Amphipod('D')
            && (b == Coordinate2D(9, 2) || b == Coordinate2D(9, 3))
            && !(b == Coordinate2D(9, 2) && a == Coordinate2D(9, 3))
        {
            continue;
        }
        let mut x2 = x.clone();
        x2[b] = at;
        x2[a] = bt;
        seen2.insert(x.clone());
        let r = search(&x2, &seen2);
        min = min.min(
            r + match at {
                Tile::Amphipod('A') => 1,
                Tile::Amphipod('B') => 10,
                Tile::Amphipod('C') => 100,
                Tile::Amphipod('D') => 1000,
                _ => unreachable!(),
            },
        )
    }
    min
}

fn print(grid: &DefaultHashMap<Coordinate2D, Tile>) {
    let min_x = grid.keys().map(|x| x.0).min().unwrap();
    let max_x = grid.keys().map(|x| x.0).max().unwrap();
    let min_y = grid.keys().map(|x| x.1).min().unwrap();
    let max_y = grid.keys().map(|x| x.1).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = Coordinate2D(x, y);
            let data = match grid[c] {
                Tile::Wall => '█',
                Tile::Path => ' ',
                Tile::Amphipod(c) => c,
            };
            print!("{}", data);
        }
        println!();
    }
}
