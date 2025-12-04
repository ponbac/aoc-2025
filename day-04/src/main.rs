use aoc::Point;

const INPUT: &str = include_str!("../input1.txt");

fn main() {
    let grid = parse_input(INPUT);
    part1(&grid);
    part2(grid);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Object {
    Roll,
    Empty,
}

impl Object {
    fn is_roll(&self) -> bool {
        matches!(self, Object::Roll)
    }
}

type Grid = Vec<Vec<Object>>;

fn parse_input(input: &str) -> Grid {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Object::Empty,
                    '@' => Object::Roll,
                    _ => panic!("Invalid character in input"),
                })
                .collect()
        })
        .collect()
}

fn count_roll_neighbors(grid: &Grid, p: Point) -> usize {
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    p.neighbors(true)
        .iter()
        .filter(|n| n.in_bounds(width, height) && grid[n.y as usize][n.x as usize].is_roll())
        .count()
}

/// Returns the next state of the grid and the number of removed rolls
fn evolve(grid: &Grid) -> (Grid, usize) {
    let mut removed_count = 0;
    let next_grid = grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &obj)| {
                    if !obj.is_roll() {
                        return Object::Empty;
                    }

                    if count_roll_neighbors(grid, (x, y).into()) < 4 {
                        removed_count += 1;
                        Object::Empty
                    } else {
                        Object::Roll
                    }
                })
                .collect()
        })
        .collect();

    (next_grid, removed_count)
}

fn part1(grid: &Grid) {
    let (_, count) = evolve(grid);
    println!("Part 1: {count}");
}

fn part2(mut grid: Grid) {
    let mut total_removed = 0;
    loop {
        let (next_grid, removed) = evolve(&grid);
        if removed == 0 {
            break;
        }
        total_removed += removed;
        grid = next_grid;
    }

    println!("Part 2: {total_removed}");
}
