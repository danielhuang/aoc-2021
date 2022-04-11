use defaultmap::DefaultHashMap;
use derive_more::{Add, AddAssign, Sub, SubAssign, Sum};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::ops::Mul;

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! input {
    ($day:literal) => {
        include_str!(concat!($day, ".sample.txt",))
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! input {
    ($day:literal) => {
        include_str!(concat!($day, ".input.txt"))
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Add, AddAssign, Sub, SubAssign, Sum, Hash, PartialOrd, Ord)]
pub struct Coordinate2D(pub i64, pub i64);
impl Coordinate2D {
    pub const ADJACENT: [Self; 4] = [Self(0, 1), Self(1, 0), Self(0, -1), Self(-1, 0)];

    pub const ADJACENT_CORNERS: [Self; 8] = [
        Self(0, 1),
        Self(1, 0),
        Self(0, -1),
        Self(-1, 0),
        Self(1, 1),
        Self(-1, 1),
        Self(1, -1),
        Self(-1, -1),
    ];

    pub fn x(self) -> i64 {
        self.0
    }

    pub fn y(self) -> i64 {
        self.1
    }

    pub fn adjacent(self) -> [Self; 4] {
        Self::ADJACENT.map(|x| self + x)
    }

    pub fn adjacent_corners(self) -> [Self; 8] {
        Self::ADJACENT_CORNERS.map(|x| self + x)
    }
}

impl Mul<i64> for Coordinate2D {
    type Output = Coordinate2D;

    fn mul(self, rhs: i64) -> Self::Output {
        Coordinate2D(self.0 * rhs, self.1 * rhs)
    }
}

pub fn print_grid<T: Clone + Debug>(grid: &DefaultHashMap<Coordinate2D, T>) {
    let min_x = grid.keys().map(|x| x.0).min().unwrap();
    let max_x = grid.keys().map(|x| x.0).max().unwrap();
    let min_y = grid.keys().map(|x| x.1).min().unwrap();
    let max_y = grid.keys().map(|x| x.1).max().unwrap();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let c = Coordinate2D(x, y);
            let data = format!("{:?}", grid[c]).chars().next().unwrap();
            print!("{}", data);
        }
        println!();
    }
}

pub fn print_hashmap<T: Clone + Debug>(grid: &HashMap<Coordinate2D, T>) {
    let min_x = grid.keys().map(|x| x.0).min().unwrap();
    let max_x = grid.keys().map(|x| x.0).max().unwrap();
    let min_y = grid.keys().map(|x| x.1).min().unwrap();
    let max_y = grid.keys().map(|x| x.1).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = Coordinate2D(x, y);
            let data: String = format!("{:?}          ", grid[&c])
                .chars()
                .take(3)
                .collect();
            print!("{}", data);
        }
        println!();
    }
}

pub fn print_hashset(grid: &HashSet<Coordinate2D>) {
    let min_x = grid.iter().map(|x| x.0).min().unwrap();
    let max_x = grid.iter().map(|x| x.0).max().unwrap();
    let min_y = grid.iter().map(|x| x.1).min().unwrap();
    let max_y = grid.iter().map(|x| x.1).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = Coordinate2D(x, y);
            print!("{}", if grid.contains(&c) { "█" } else { " " });
        }
        println!();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Add, AddAssign, Sub, SubAssign, Sum, Hash)]
pub struct Coordinate3D(pub i64, pub i64, pub i64);

impl Mul<i64> for Coordinate3D {
    type Output = Coordinate3D;

    fn mul(self, rhs: i64) -> Self::Output {
        Coordinate3D(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Matrix3 {
    pub cols: [Coordinate3D; 3],
}

impl Mul<Coordinate3D> for Matrix3 {
    type Output = Coordinate3D;

    fn mul(self, rhs: Coordinate3D) -> Self::Output {
        self.cols[0] * rhs.0 + self.cols[1] * rhs.1 + self.cols[2] * rhs.2
    }
}
