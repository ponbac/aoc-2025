use crate::{Direction, Point};
use std::fmt::{self, Display};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

/// A 2D grid structure commonly used for AoC problems.
///
/// The grid uses (x, y) coordinates where:
/// - x increases to the right (column index)
/// - y increases downward (row index)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    cells: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    /// Creates a new grid with the given dimensions, filled with the default value.
    pub fn new(width: usize, height: usize, default: T) -> Self
    where
        T: Clone,
    {
        Self {
            cells: vec![default; width * height],
            width,
            height,
        }
    }

    /// Creates a grid from a vector of rows.
    pub fn from_rows(rows: Vec<Vec<T>>) -> Self {
        let height = rows.len();
        let width = rows.first().map_or(0, |r| r.len());
        let cells = rows.into_iter().flatten().collect();
        Self {
            cells,
            width,
            height,
        }
    }

    /// Returns the width of the grid.
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid.
    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the dimensions as (width, height).
    #[inline]
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Converts (x, y) coordinates to a linear index.
    #[inline]
    fn to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    /// Returns true if the given coordinates are within bounds.
    #[inline]
    pub fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }

    /// Returns true if the given point is within bounds.
    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        self.in_bounds(point.x, point.y)
    }

    /// Gets a reference to the cell at (x, y), if in bounds.
    #[inline]
    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        if self.in_bounds(x, y) {
            Some(&self.cells[self.to_index(x as usize, y as usize)])
        } else {
            None
        }
    }

    /// Gets a mutable reference to the cell at (x, y), if in bounds.
    #[inline]
    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        if self.in_bounds(x, y) {
            let idx = self.to_index(x as usize, y as usize);
            Some(&mut self.cells[idx])
        } else {
            None
        }
    }

    /// Gets a reference to the cell at the given point, if in bounds.
    #[inline]
    pub fn get_point(&self, point: Point) -> Option<&T> {
        self.get(point.x, point.y)
    }

    /// Gets a mutable reference to the cell at the given point, if in bounds.
    #[inline]
    pub fn get_point_mut(&mut self, point: Point) -> Option<&mut T> {
        self.get_mut(point.x, point.y)
    }

    /// Sets the value at (x, y). Panics if out of bounds.
    #[inline]
    pub fn set(&mut self, x: usize, y: usize, value: T) {
        let idx = self.to_index(x, y);
        self.cells[idx] = value;
    }

    /// Sets the value at the given point. Panics if out of bounds.
    #[inline]
    pub fn set_point(&mut self, point: Point, value: T) {
        self.set(point.x as usize, point.y as usize, value);
    }

    /// Returns an iterator over all cells with their (x, y) coordinates.
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.cells.iter().enumerate().map(|(i, cell)| {
            let x = i % self.width;
            let y = i / self.width;
            (x, y, cell)
        })
    }

    /// Returns a mutable iterator over all cells with their (x, y) coordinates.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        let width = self.width;
        self.cells.iter_mut().enumerate().map(move |(i, cell)| {
            let x = i % width;
            let y = i / width;
            (x, y, cell)
        })
    }

    /// Returns an iterator over all points in the grid.
    pub fn points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.height).flat_map(move |y| {
            (0..self.width).map(move |x| Point::new(x as isize, y as isize))
        })
    }

    /// Returns an iterator over a specific row.
    pub fn row(&self, y: usize) -> impl Iterator<Item = &T> {
        let start = y * self.width;
        self.cells[start..start + self.width].iter()
    }

    /// Returns an iterator over a specific column.
    pub fn col(&self, x: usize) -> impl Iterator<Item = &T> {
        (0..self.height).map(move |y| &self.cells[y * self.width + x])
    }

    /// Returns an iterator over all rows.
    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.height).map(move |y| self.row(y))
    }

    /// Returns the neighbors of a point in the given directions.
    pub fn neighbors<'a>(
        &'a self,
        point: Point,
        directions: &'a [Direction],
    ) -> impl Iterator<Item = (Point, &'a T)> {
        directions.iter().filter_map(move |&dir| {
            let neighbor = point + dir;
            self.get_point(neighbor).map(|cell| (neighbor, cell))
        })
    }

    /// Returns the 4 cardinal neighbors of a point (up, right, down, left).
    pub fn cardinal_neighbors(&self, point: Point) -> impl Iterator<Item = (Point, &T)> {
        self.neighbors(point, &Direction::ALL_BASIC)
    }

    /// Returns all 8 neighbors of a point (including diagonals).
    pub fn all_neighbors(&self, point: Point) -> impl Iterator<Item = (Point, &T)> {
        let all_dirs: Vec<_> = Direction::ALL_BASIC
            .iter()
            .chain(Direction::ALL_DIAGONAL.iter())
            .copied()
            .collect();
        let results: Vec<_> = all_dirs
            .into_iter()
            .filter_map(|dir| {
                let neighbor = point + dir;
                self.get_point(neighbor).map(|cell| (neighbor, cell))
            })
            .collect();
        results.into_iter()
    }

    /// Finds all positions matching the predicate.
    pub fn find_all<P>(&self, predicate: P) -> Vec<Point>
    where
        P: Fn(&T) -> bool,
    {
        self.iter()
            .filter(|(_, _, cell)| predicate(cell))
            .map(|(x, y, _)| Point::new(x as isize, y as isize))
            .collect()
    }

    /// Finds the first position matching the predicate.
    pub fn find<P>(&self, predicate: P) -> Option<Point>
    where
        P: Fn(&T) -> bool,
    {
        self.iter()
            .find(|(_, _, cell)| predicate(cell))
            .map(|(x, y, _)| Point::new(x as isize, y as isize))
    }

    /// Counts cells matching the predicate.
    pub fn count<P>(&self, predicate: P) -> usize
    where
        P: Fn(&T) -> bool,
    {
        self.cells.iter().filter(|cell| predicate(cell)).count()
    }

    /// Maps each cell to a new value using the provided function.
    pub fn map<U, F>(&self, f: F) -> Grid<U>
    where
        F: Fn(&T) -> U,
    {
        Grid {
            cells: self.cells.iter().map(f).collect(),
            width: self.width,
            height: self.height,
        }
    }

    /// Returns the underlying cells as a flat slice.
    pub fn as_slice(&self) -> &[T] {
        &self.cells
    }

    /// Returns the underlying cells as a mutable flat slice.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.cells
    }
}

// Parsing from string for char grids
impl FromStr for Grid<char> {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Vec<char>> = s
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect();

        Ok(Self::from_rows(rows))
    }
}

// Parsing from string for grids of any type that implements FromStr
impl<T> Grid<T>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    /// Parses a grid where each cell is a single character parsed into type T.
    pub fn parse_chars(s: &str) -> Result<Self, T::Err> {
        let mut rows = Vec::new();
        for line in s.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
            let row: Result<Vec<T>, _> = line.chars().map(|c| c.to_string().parse::<T>()).collect();
            rows.push(row?);
        }
        Ok(Self::from_rows(rows))
    }

    /// Parses a grid where cells are separated by whitespace.
    pub fn parse_whitespace(s: &str) -> Result<Self, T::Err> {
        let mut rows = Vec::new();
        for line in s.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
            let row: Result<Vec<T>, _> = line.split_whitespace().map(|s| s.parse::<T>()).collect();
            rows.push(row?);
        }
        Ok(Self::from_rows(rows))
    }

    /// Parses a grid where cells are separated by a custom delimiter.
    pub fn parse_delimited(s: &str, delimiter: &str) -> Result<Self, T::Err> {
        let mut rows = Vec::new();
        for line in s.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
            let row: Result<Vec<T>, _> = line
                .split(delimiter)
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse::<T>())
                .collect();
            rows.push(row?);
        }
        Ok(Self::from_rows(rows))
    }
}

// Index by (usize, usize)
impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.cells[self.to_index(x, y)]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let idx = self.to_index(x, y);
        &mut self.cells[idx]
    }
}

// Index by Point
impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        &self[(point.x as usize, point.y as usize)]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self[(point.x as usize, point.y as usize)]
    }
}

// Index by &Point
impl<T> Index<&Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: &Point) -> &Self::Output {
        &self[(point.x as usize, point.y as usize)]
    }
}

impl<T> IndexMut<&Point> for Grid<T> {
    fn index_mut(&mut self, point: &Point) -> &mut Self::Output {
        &mut self[(point.x as usize, point.y as usize)]
    }
}

// Display for char grids
impl Display for Grid<char> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self[(x, y)])?;
            }
            if y < self.height - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

// Display for any grid type that implements Display
impl<T: Display> Grid<T> {
    /// Formats the grid with a custom separator between cells.
    pub fn display_with_separator(&self, separator: &str) -> String {
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if x > 0 {
                    result.push_str(separator);
                }
                result.push_str(&format!("{}", self[(x, y)]));
            }
            if y < self.height - 1 {
                result.push('\n');
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
    .......S.......
    ...............
    .......^.......
    ...............
    ......^.^......
    ...............
    .....^.^.^.....
    ...............
    ....^.^...^....
    ...............
    ...^.^...^.^...
    ...............
    ..^...^.....^..
    ...............
    .^.^.^.^.^...^.
    ...............
    "#;

    #[test]
    fn test_parse_char_grid() {
        let grid: Grid<char> = EXAMPLE.parse().unwrap();
        assert_eq!(grid.width(), 15);
        assert_eq!(grid.height(), 16);
        assert_eq!(grid[(7, 0)], 'S');
        assert_eq!(grid[(7, 2)], '^');
        assert_eq!(grid[(0, 0)], '.');
    }

    #[test]
    fn test_find_all() {
        let grid: Grid<char> = EXAMPLE.parse().unwrap();
        let carets = grid.find_all(|&c| c == '^');
        assert!(!carets.is_empty());
        assert!(carets.contains(&Point::new(7, 2)));
    }

    #[test]
    fn test_find() {
        let grid: Grid<char> = EXAMPLE.parse().unwrap();
        let start = grid.find(|&c| c == 'S');
        assert_eq!(start, Some(Point::new(7, 0)));
    }

    #[test]
    fn test_neighbors() {
        let grid: Grid<char> = EXAMPLE.parse().unwrap();
        let point = Point::new(7, 2);
        let neighbors: Vec<_> = grid.cardinal_neighbors(point).collect();
        assert_eq!(neighbors.len(), 4);
    }

    #[test]
    fn test_in_bounds() {
        let grid: Grid<char> = EXAMPLE.parse().unwrap();
        assert!(grid.in_bounds(0, 0));
        assert!(grid.in_bounds(14, 15));
        assert!(!grid.in_bounds(-1, 0));
        assert!(!grid.in_bounds(15, 0));
    }

    #[test]
    fn test_display() {
        let input = ".#.\n#.#\n.#.";
        let grid: Grid<char> = input.parse().unwrap();
        assert_eq!(grid.to_string(), input);
    }

    #[test]
    fn test_new_grid() {
        let grid: Grid<i32> = Grid::new(5, 3, 0);
        assert_eq!(grid.width(), 5);
        assert_eq!(grid.height(), 3);
        assert_eq!(grid[(0, 0)], 0);
    }

    #[test]
    fn test_map() {
        let grid: Grid<char> = ".#.\n#.#".parse().unwrap();
        let mapped = grid.map(|&c| c == '#');
        assert!(!mapped[(0, 0)]);
        assert!(mapped[(1, 0)]);
    }

    #[test]
    fn test_parse_digits() {
        let input = "123\n456\n789";
        let grid: Grid<u8> = Grid::parse_chars(input).unwrap();
        assert_eq!(grid[(0, 0)], 1);
        assert_eq!(grid[(2, 2)], 9);
    }
}
