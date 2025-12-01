#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn as_step(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::UpRight => (1, -1),
            Direction::Right => (1, 0),
            Direction::DownRight => (1, 1),
            Direction::Down => (0, 1),
            Direction::DownLeft => (-1, 1),
            Direction::Left => (-1, 0),
            Direction::UpLeft => (-1, -1),
        }
    }

    pub const ALL_BASIC: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    pub const ALL_DIAGONAL: [Direction; 4] = [
        Direction::UpRight,
        Direction::DownRight,
        Direction::DownLeft,
        Direction::UpLeft,
    ];
}

impl std::ops::Add<Direction> for (isize, isize) {
    type Output = (isize, isize);
    fn add(self, rhs: Direction) -> Self::Output {
        let step = rhs.as_step();
        (self.0 + step.0, self.1 + step.1)
    }
}

impl std::ops::Sub<Direction> for (isize, isize) {
    type Output = (isize, isize);
    fn sub(self, rhs: Direction) -> Self::Output {
        let step = rhs.as_step();
        (self.0 - step.0, self.1 - step.1)
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' | '^' => Direction::Up,
            'R' | '>' => Direction::Right,
            'D' | 'v' => Direction::Down,
            'L' | '<' => Direction::Left,
            _ => panic!("Invalid direction character: {}", c),
        }
    }
}
