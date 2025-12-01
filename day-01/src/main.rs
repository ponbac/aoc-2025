use std::str::FromStr;

const INPUT: &str = include_str!("../input1.txt");
const TARGET_MOD: isize = 100;
const START_POS: isize = 50;

fn main() {
    let moves = parse_input(INPUT);

    let (pos1, hits1) = part1(&moves);
    println!("PART 1: pos={pos1} | times_at_zero={hits1}");

    let (pos2, hits2) = part2(&moves);
    println!("PART 2: pos={pos2} | times_at_zero={hits2}");
}

// ----------------- Part 1 -----------------

fn part1(moves: &[Move]) -> (isize, usize) {
    moves.iter().fold((START_POS, 0), |(pos, hits), m| {
        let pos = pos + m.delta();
        let hits = hits + is_multiple_of_target(pos) as usize;
        (pos, hits)
    })
}

// ----------------- Part 2 -----------------

fn part2(moves: &[Move]) -> (isize, usize) {
    moves.iter().fold((START_POS, 0), |(pos, hits), m| {
        let (new_pos, segment_hits) = match m.dir {
            Direction::Right => {
                let new_pos = pos + m.steps;
                // counts multiples in (pos, new_pos]
                let cnt = new_pos.div_euclid(TARGET_MOD) - pos.div_euclid(TARGET_MOD);
                (new_pos, cnt as usize)
            }
            Direction::Left => {
                let new_pos = pos - m.steps;
                // counts multiples in [new_pos, pos)
                // which is equal to counting in (new_pos - 1, pos - 1]
                let cnt = (pos - 1).div_euclid(TARGET_MOD) - (new_pos - 1).div_euclid(TARGET_MOD);
                (new_pos, cnt as usize)
            }
        };

        (new_pos, hits + segment_hits)
    })
}

fn is_multiple_of_target(x: isize) -> bool {
    // Same behavior as x.abs() % 100 == 0 for equality-to-zero checks,
    // but clearer about modular intent and handles negatives well.
    x.rem_euclid(TARGET_MOD) == 0
}

// ----------------- Domain types -----------------

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Move {
    dir: Direction,
    steps: isize,
}

impl Move {
    fn delta(self) -> isize {
        match self.dir {
            Direction::Left => -self.steps,
            Direction::Right => self.steps,
        }
    }
}

// ----------------- Parsing -----------------

fn parse_input(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Move>().expect("failed to parse line"))
        .collect()
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err("invalid direction"),
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(format!("line too short: {s}"));
        }

        let (dir_str, steps_str) = s.split_at(1);

        let dir = dir_str.parse::<Direction>().map_err(|e| e.to_string())?;

        let steps = steps_str
            .parse::<isize>()
            .map_err(|_| format!("invalid step count: {steps_str}"))?;

        Ok(Move { dir, steps })
    }
}
