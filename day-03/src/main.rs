use std::str::FromStr;

const INPUT: &str = include_str!("../input1.txt");
const EXAMPLE: &str = r#"
987654321111111
811111111111119
234234234234278
818181911112111
"#;

fn main() {
    let banks = parse_input(INPUT);
    part1(&banks);
    part2(&banks);
}

fn parse_input(input: &str) -> Vec<Bank> {
    input
        .trim()
        .lines()
        .map(|l| l.parse().expect("could not parse bank"))
        .collect()
}

fn part1(banks: &[Bank]) {
    let max_joltages: Vec<u64> = banks.iter().map(|bank| bank.max_joltage(2)).collect();
    println!("Part 1: {}", max_joltages.iter().sum::<u64>())
}

fn part2(banks: &[Bank]) {
    let max_joltages: Vec<u64> = banks.iter().map(|bank| bank.max_joltage(12)).collect();
    println!("Part 2: {}", max_joltages.iter().sum::<u64>())
}

#[derive(Debug)]
struct Bank {
    batteries: Vec<u32>,
}

impl Bank {
    pub fn max_joltage(&self, len: u32) -> u64 {
        let batteries = &self.batteries;

        let max_index = batteries
            .iter()
            .take(batteries.len().saturating_sub(len as usize - 1))
            .enumerate()
            .max_by(|&(i1, &v1), &(i2, &v2)| v1.cmp(&v2).then(i2.cmp(&i1)))
            .map(|(i, _)| i)
            .expect("no max");

        let mut number_str = batteries[max_index].to_string();
        let mut current_index = max_index;
        let mut n_to_take = len - 1;

        while n_to_take > 0 {
            let remaining_after_this = (n_to_take - 1) as usize;
            let end_index = batteries.len() - remaining_after_this;
            let potential_numbers = &batteries[current_index + 1..end_index];

            let (local_idx, &max_digit) = potential_numbers
                .iter()
                .enumerate()
                .max_by(|&(i1, &v1), &(i2, &v2)| v1.cmp(&v2).then(i2.cmp(&i1)))
                .expect("no candidates");

            current_index = current_index + 1 + local_idx;
            number_str.push_str(&max_digit.to_string());
            n_to_take -= 1;
        }

        number_str
            .parse::<u64>()
            .expect("could not parse max joltage string")
    }
}

impl FromStr for Bank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            batteries: s
                .chars()
                .map(|c| c.to_digit(10).ok_or_else(|| format!("invalid digit: {c}")))
                .collect::<Result<_, _>>()?,
        })
    }
}
