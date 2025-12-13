use aoc::{Grid, Point};
use itertools::Itertools;
use std::collections::VecDeque;

const INPUT: &str = include_str!("../input1.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Outside,
    Boundary,
    Inside,
}

fn main() {
    part1();
    part2();
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (x, y) = line.trim().split_once(',').unwrap();
            Point::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

/// Normalize points to 0-based coordinates, returns (min_x, min_y) offset
fn normalize_points(points: &mut [Point]) -> (isize, isize) {
    let min_x = points.iter().map(|p| p.x).min().unwrap_or(0);
    let min_y = points.iter().map(|p| p.y).min().unwrap_or(0);

    for point in points.iter_mut() {
        point.x -= min_x;
        point.y -= min_y;
    }

    (min_x, min_y)
}

fn part1() {
    let points = parse_input(INPUT);

    let max_area = points
        .iter()
        .tuple_combinations()
        .map(|(&p1, &p2)| {
            let dx = p2.x - p1.x;
            let dy = p2.y - p1.y;
            (dx.abs() + 1) * (dy.abs() + 1)
        })
        .max()
        .unwrap();

    println!("Part 1: {max_area}")
}

/// Build a compressed grid representation of the polygon
/// Returns the grid, sorted unique x-coordinates, and sorted unique y-coordinates
fn build_compressed_grid(points: &[Point]) -> (Grid<Cell>, Vec<isize>, Vec<isize>) {
    // Collect all unique x and y coordinates
    let mut x_coords: Vec<isize> = points.iter().map(|p| p.x).collect();
    let mut y_coords: Vec<isize> = points.iter().map(|p| p.y).collect();

    // Add coordinates for cells between vertices (for proper flood fill)
    // We need to add intermediate points to properly represent the grid
    let mut extra_x = Vec::new();
    let mut extra_y = Vec::new();

    for &x in &x_coords {
        extra_x.push(x - 1);
        extra_x.push(x + 1);
    }
    for &y in &y_coords {
        extra_y.push(y - 1);
        extra_y.push(y + 1);
    }

    x_coords.extend(extra_x);
    y_coords.extend(extra_y);

    // Add boundary points for flood fill from outside
    let min_x = *x_coords.iter().min().unwrap() - 1;
    let max_x = *x_coords.iter().max().unwrap() + 1;
    let min_y = *y_coords.iter().min().unwrap() - 1;
    let max_y = *y_coords.iter().max().unwrap() + 1;

    x_coords.push(min_x);
    x_coords.push(max_x);
    y_coords.push(min_y);
    y_coords.push(max_y);

    x_coords.sort();
    x_coords.dedup();
    y_coords.sort();
    y_coords.dedup();

    let width = x_coords.len();
    let height = y_coords.len();

    // Create grid initialized to Inside (we'll mark outside via flood fill)
    let mut grid: Grid<Cell> = Grid::new(width, height, Cell::Inside);

    // Helper to find index in coordinate arrays
    let find_x = |x: isize| x_coords.binary_search(&x).unwrap();
    let find_y = |y: isize| y_coords.binary_search(&y).unwrap();

    // Draw boundary lines between consecutive points
    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];

        // Mark the line between p1 and p2
        if p1.x == p2.x {
            // Vertical line
            let x_idx = find_x(p1.x);
            let y1_idx = find_y(p1.y);
            let y2_idx = find_y(p2.y);
            let (start_y, end_y) = if y1_idx < y2_idx {
                (y1_idx, y2_idx)
            } else {
                (y2_idx, y1_idx)
            };
            for y_idx in start_y..=end_y {
                grid.set(x_idx, y_idx, Cell::Boundary);
            }
        } else if p1.y == p2.y {
            // Horizontal line
            let y_idx = find_y(p1.y);
            let x1_idx = find_x(p1.x);
            let x2_idx = find_x(p2.x);
            let (start_x, end_x) = if x1_idx < x2_idx {
                (x1_idx, x2_idx)
            } else {
                (x2_idx, x1_idx)
            };
            for x_idx in start_x..=end_x {
                grid.set(x_idx, y_idx, Cell::Boundary);
            }
        } else {
            panic!("Points must be axis-aligned: {:?} -> {:?}", p1, p2);
        }
    }

    // Flood fill from outside (top-left corner which should be outside)
    flood_fill_outside(&mut grid);

    (grid, x_coords, y_coords)
}

/// Flood fill from the edges to mark all outside cells
fn flood_fill_outside(grid: &mut Grid<Cell>) {
    let width = grid.width();
    let height = grid.height();

    let mut queue = VecDeque::new();

    // Start from all edge cells that are not boundaries
    for x in 0..width {
        if grid[(x, 0)] != Cell::Boundary {
            queue.push_back((x, 0));
        }
        if grid[(x, height - 1)] != Cell::Boundary {
            queue.push_back((x, height - 1));
        }
    }
    for y in 0..height {
        if grid[(0, y)] != Cell::Boundary {
            queue.push_back((0, y));
        }
        if grid[(width - 1, y)] != Cell::Boundary {
            queue.push_back((width - 1, y));
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        if grid[(x, y)] != Cell::Inside {
            continue;
        }

        grid.set(x, y, Cell::Outside);

        // Add neighbors
        if x > 0 {
            queue.push_back((x - 1, y));
        }
        if x + 1 < width {
            queue.push_back((x + 1, y));
        }
        if y > 0 {
            queue.push_back((x, y - 1));
        }
        if y + 1 < height {
            queue.push_back((x, y + 1));
        }
    }
}

/// Check if an entire rectangle (defined by two opposite corners) is valid
/// All points in the rectangle must be inside or on the boundary
fn is_rectangle_valid(
    p1: Point,
    p2: Point,
    grid: &Grid<Cell>,
    x_coords: &[isize],
    y_coords: &[isize],
) -> bool {
    let min_x = p1.x.min(p2.x);
    let max_x = p1.x.max(p2.x);
    let min_y = p1.y.min(p2.y);
    let max_y = p1.y.max(p2.y);

    // Find the range of compressed grid cells that this rectangle covers
    let x_start = match x_coords.binary_search(&min_x) {
        Ok(idx) => idx,
        Err(idx) => idx,
    };
    let x_end = match x_coords.binary_search(&max_x) {
        Ok(idx) => idx,
        Err(idx) => idx - 1,
    };
    let y_start = match y_coords.binary_search(&min_y) {
        Ok(idx) => idx,
        Err(idx) => idx,
    };
    let y_end = match y_coords.binary_search(&max_y) {
        Ok(idx) => idx,
        Err(idx) => idx - 1,
    };

    // Check all cells in the range
    for x_idx in x_start..=x_end {
        for y_idx in y_start..=y_end {
            // Only check cells that are actually within the rectangle bounds
            let cell_x = x_coords[x_idx];
            let cell_y = y_coords[y_idx];

            if cell_x >= min_x
                && cell_x <= max_x
                && cell_y >= min_y
                && cell_y <= max_y
                && grid[(x_idx, y_idx)] == Cell::Outside
            {
                return false;
            }
        }
    }

    true
}

fn part2() {
    let mut points = parse_input(INPUT);
    normalize_points(&mut points);

    let (grid, x_coords, y_coords) = build_compressed_grid(&points);

    // Find the largest valid rectangle
    let mut max_area = 0isize;

    for (i, &p1) in points.iter().enumerate() {
        for &p2 in points.iter().skip(i + 1) {
            let area = (p2.x - p1.x).abs() + 1;
            let height = (p2.y - p1.y).abs() + 1;
            let rect_area = area * height;

            // Early skip if this can't beat current max
            if rect_area <= max_area {
                continue;
            }

            if is_rectangle_valid(p1, p2, &grid, &x_coords, &y_coords) {
                max_area = rect_area;
            }
        }
    }

    println!("Part 2: {max_area}");
}
