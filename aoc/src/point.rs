use crate::direction::Direction;
use std::{
    num::ParseIntError,
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// Creates a Point from a tuple of (x, y)
    pub fn from_tuple(tuple: (isize, isize)) -> Self {
        Self::new(tuple.0, tuple.1)
    }

    /// Converts the Point to a tuple of (x, y)
    pub fn as_tuple(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    /// Returns a new Point moved one step in the given direction
    pub fn step(&self, direction: Direction) -> Self {
        let (dx, dy) = direction.as_step();
        Self::new(self.x + dx, self.y + dy)
    }

    /// Returns Manhattan distance to another point
    pub fn manhattan_distance(&self, other: &Point) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }

    /// Returns all adjacent points (including diagonals if specified)
    pub fn neighbors(&self, include_diagonals: bool) -> Vec<Point> {
        let directions = if include_diagonals {
            Direction::ALL_BASIC
                .iter()
                .chain(Direction::ALL_DIAGONAL.iter())
                .collect::<Vec<_>>()
        } else {
            Direction::ALL_BASIC.iter().collect()
        };

        directions.into_iter().map(|&dir| self.step(dir)).collect()
    }

    /// Returns true if the point is within the given bounds (inclusive)
    pub fn in_bounds(&self, width: isize, height: isize) -> bool {
        self.x >= 0 && self.x < width && self.y >= 0 && self.y < height
    }

    /// Wraps the point around the given bounds
    pub fn wrap_around(&mut self, width: isize, height: isize) {
        self.x = ((self.x % width) + width) % width;
        self.y = ((self.y % height) + height) % height;
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.trim().split_once(',').unwrap();
        Ok(Self::new(x.parse()?, y.parse()?))
    }
}

// Implement conversion from tuple
impl From<(isize, isize)> for Point {
    fn from(tuple: (isize, isize)) -> Self {
        Self::from_tuple(tuple)
    }
}

impl From<(usize, usize)> for Point {
    fn from(tuple: (usize, usize)) -> Self {
        Self::new(tuple.0 as isize, tuple.1 as isize)
    }
}

// Implement addition with Direction
impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, direction: Direction) -> Point {
        self.step(direction)
    }
}

// Implement addition assignment with Direction
impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, direction: Direction) {
        let (dx, dy) = direction.as_step();
        self.x += dx;
        self.y += dy;
    }
}

// Implement subtraction between Points
impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

// Implement subtraction assignment between Points
impl SubAssign<Point> for Point {
    fn sub_assign(&mut self, other: Point) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

// Display implementation for pretty printing
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

macro_rules! impl_numeric_ops {
    ($($t:ty),*) => {
        $(
            impl Add<($t, $t)> for Point {
                type Output = Point;

                fn add(self, rhs: ($t, $t)) -> Point {
                    Point::new(
                        self.x + rhs.0 as isize,
                        self.y + rhs.1 as isize,
                    )
                }
            }

            impl AddAssign<($t, $t)> for Point {
                fn add_assign(&mut self, rhs: ($t, $t)) {
                    self.x += rhs.0 as isize;
                    self.y += rhs.1 as isize;
                }
            }

            impl Sub<($t, $t)> for Point {
                type Output = Point;

                fn sub(self, rhs: ($t, $t)) -> Point {
                    Point::new(
                        self.x - rhs.0 as isize,
                        self.y - rhs.1 as isize,
                    )
                }
            }

            impl SubAssign<($t, $t)> for Point {
                fn sub_assign(&mut self, rhs: ($t, $t)) {
                    self.x -= rhs.0 as isize;
                    self.y -= rhs.1 as isize;
                }
            }
        )*
    };
}

impl_numeric_ops!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
