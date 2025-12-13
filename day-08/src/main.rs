use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use itertools::Itertools;

const INPUT: &str = include_str!("../input1.txt");

fn main() {
    let points = parse_input(INPUT);
    println!("Part 1: {}", part1(&points, 1000));
    println!("Part 2: {}", part2(&points));
}

fn part1(points: &[Point], connections: usize) -> usize {
    let mut uf = sorted_pairs(points).take(connections).fold(
        UnionFind::new(points.len()),
        |mut uf, (_, a, b)| {
            uf.union(a, b);
            uf
        },
    );

    // Count circuit sizes and get product of three largest
    (0..points.len())
        .map(|i| uf.find(i))
        .fold(HashMap::new(), |mut acc, root| {
            *acc.entry(root).or_insert(0) += 1;
            acc
        })
        .into_values()
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .product()
}

fn part2(points: &[Point]) -> isize {
    let connections_needed = points.len() - 1;

    let (a, b) = sorted_pairs(points)
        .scan(UnionFind::new(points.len()), |uf, (_, a, b)| {
            Some(uf.union(a, b).then_some((a, b)))
        })
        .flatten()
        .take(connections_needed)
        .last()
        .expect("Should have made at least one connection");

    points[a].x * points[b].x
}

/// Generate all point pairs sorted by distance (shortest first)
fn sorted_pairs(points: &[Point]) -> impl Iterator<Item = (f64, usize, usize)> {
    (0..points.len())
        .tuple_combinations()
        .map(|(i, j)| (points[i].euclidean_distance(&points[j]), i, j))
        .sorted_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal))
}

#[derive(Clone)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// Returns true if merge happened (x and y were in different sets)
    fn union(&mut self, x: usize, y: usize) -> bool {
        let (root_x, root_y) = (self.find(x), self.find(y));

        if root_x == root_y {
            return false;
        }

        match self.rank[root_x].cmp(&self.rank[root_y]) {
            Ordering::Less => self.parent[root_x] = root_y,
            Ordering::Greater => self.parent[root_y] = root_x,
            Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
        true
    }
}

#[derive(Debug, Clone)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl FromStr for Point {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<isize> = s.trim().split(',').map(|n| n.parse()).try_collect()?;

        match coords.as_slice() {
            [x, y, z] => Ok(Self {
                x: *x,
                y: *y,
                z: *z,
            }),
            _ => Err("Expected exactly 3 coordinates".into()),
        }
    }
}

impl Point {
    fn euclidean_distance(&self, other: &Self) -> f64 {
        let (dx, dy, dz) = (
            (self.x - other.x) as f64,
            (self.y - other.y) as f64,
            (self.z - other.z) as f64,
        );
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().expect("valid point"))
        .collect()
}
