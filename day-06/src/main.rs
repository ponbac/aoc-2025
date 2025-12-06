const INPUT: &str = include_str!("../input1.txt");
const EXAMPLE: &str = r#"
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
"#;

fn main() {
    println!("Part 1: {}", solve(EXAMPLE, false));
    println!("Part 2: {}", solve(INPUT, true));
}

struct Section {
    numbers: Vec<u64>,
    sign: char,
}

impl Section {
    fn evaluate(&self) -> u64 {
        match self.sign {
            '+' => self.numbers.iter().sum(),
            '*' => self.numbers.iter().product(),
            _ => unreachable!(),
        }
    }
}

fn split_into_section_grids(input: &str) -> Vec<Vec<Vec<char>>> {
    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut sections = vec![];
    let mut prev_split = 0;

    for x in 0..grid[0].len() {
        if grid.iter().all(|row| row.get(x) == Some(&' ')) {
            if x > prev_split {
                sections.push(grid.iter().map(|row| row[prev_split..x].to_vec()).collect());
            }
            prev_split = x + 1;
        }
    }
    if prev_split < grid[0].len() {
        sections.push(grid.iter().map(|row| row[prev_split..].to_vec()).collect());
    }

    sections
}

fn parse_section(grid: &[Vec<char>], vertical: bool) -> Section {
    let sign = grid
        .last()
        .unwrap()
        .iter()
        .find(|c| !c.is_whitespace())
        .copied()
        .unwrap();
    let data: Vec<&[char]> = grid[..grid.len() - 1]
        .iter()
        .map(|r| r.as_slice())
        .collect();

    let numbers = if vertical {
        let max_x = data.iter().map(|r| r.len()).max().unwrap_or(0);
        (0..max_x)
            .map(|x| {
                data.iter()
                    .map(|row| row.get(max_x - 1 - x).unwrap_or(&' '))
                    .collect::<String>()
                    .trim()
                    .parse()
                    .unwrap()
            })
            .collect()
    } else {
        data.iter()
            .map(|row| row.iter().collect::<String>().trim().parse().unwrap())
            .collect()
    };

    Section { numbers, sign }
}

fn solve(input: &str, vertical: bool) -> u64 {
    split_into_section_grids(input)
        .iter()
        .map(|g| parse_section(g, vertical).evaluate())
        .sum()
}
