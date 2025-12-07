use aoc::Grid;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input1.txt");
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

fn main() {
    part1();
    part2();
}

fn count_splits(grid: &Grid<char>) -> usize {
    let start = grid.find(|&c| c == 'S').expect("No start position found");

    let mut beams = HashSet::new();
    beams.insert(start.x as usize);

    let mut split_count = 0;
    for y in (start.y as usize + 1)..grid.height() {
        let mut new_beams = HashSet::new();
        for &x in &beams {
            match grid[(x, y)] {
                '^' => {
                    split_count += 1;
                    if x > 0 {
                        new_beams.insert(x - 1);
                    }
                    if x < grid.width() - 1 {
                        new_beams.insert(x + 1);
                    }
                }
                _ => {
                    new_beams.insert(x);
                }
            }
        }

        beams = new_beams;
    }

    split_count
}

fn part1() {
    let grid = INPUT.parse().unwrap();
    println!("Part 1: {}", count_splits(&grid));
}

fn count_timelines(grid: &Grid<char>) -> usize {
    let start = grid.find(|&c| c == 'S').expect("No start position found");

    // Map from x-coordinate to number of timelines at that position
    let mut timelines: HashMap<usize, usize> = HashMap::new();
    timelines.insert(start.x as usize, 1);

    for y in (start.y as usize + 1)..grid.height() {
        let mut new_timelines: HashMap<usize, usize> = HashMap::new();
        for (&x, &count) in &timelines {
            match grid[(x, y)] {
                '^' => {
                    if x > 0 {
                        *new_timelines.entry(x - 1).or_default() += count;
                    }
                    if x < grid.width() - 1 {
                        *new_timelines.entry(x + 1).or_default() += count;
                    }
                }
                _ => {
                    *new_timelines.entry(x).or_default() += count;
                }
            }
        }
        timelines = new_timelines;
    }

    timelines.values().sum()
}

fn part2() {
    let grid = INPUT.parse().unwrap();
    println!("Part 2: {}", count_timelines(&grid));
}
