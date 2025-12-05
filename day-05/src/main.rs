use std::ops::RangeInclusive;

use itertools::Itertools;

const INPUT: &str = include_str!("../input1.txt");
const EXAMPLE: &str = r#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

fn main() {
    part1();
    part2();
}

fn part1() {
    let (fresh_ranges_str, ingredients_str) = INPUT.trim().split_once("\n\n").expect("no new line");
    let ranges: Vec<RangeInclusive<u64>> = fresh_ranges_str
        .lines()
        .map(|line| line.split_once("-").expect("no '-' found"))
        .map(|(left, right)| left.parse().unwrap()..=right.parse().unwrap())
        .collect();
    let ingredients: Vec<u64> = ingredients_str
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut n_fresh_ingredients = 0;
    'outer: for ingredient in ingredients {
        for range in &ranges {
            if range.contains(&ingredient) {
                n_fresh_ingredients += 1;
                continue 'outer;
            }
        }
    }

    println!("Part 1: {n_fresh_ingredients}");
}

fn part2() {
    let (fresh_ranges_str, _) = INPUT.trim().split_once("\n\n").expect("no new line");
    let ranges: Vec<RangeInclusive<u64>> = fresh_ranges_str
        .lines()
        .map(|line| line.split_once("-").expect("no '-' found"))
        .map(|(left, right)| left.parse().unwrap()..=right.parse().unwrap())
        .sorted_by_key(|range| *range.start())
        .collect();

    let mut merged_ranges = vec![];
    let mut current_range = ranges.first().unwrap().clone();
    for range in ranges.iter().skip(1) {
        if range.start() <= current_range.end() {
            current_range = *current_range.start()..=*current_range.end().max(range.end());
        } else {
            merged_ranges.push(current_range);
            current_range = range.clone();
        }
    }
    merged_ranges.push(current_range);

    let total_fresh_ingredients = merged_ranges
        .iter()
        .map(|range| (range.end() - range.start()) + 1)
        .sum::<u64>();

    println!("Part 2: {total_fresh_ingredients}");
}
