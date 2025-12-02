use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input1.txt");

fn main() {
    let ranges = parse_input(INPUT);
    part1(&ranges);
    part2(&ranges);
}

fn parse_input(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .trim()
        .split(',')
        .map(|range_str| {
            let (start, end) = range_str
                .split_once('-')
                .expect("Input must match format 'start-end'");

            let start = start.parse().expect("Invalid start number");
            let end = end.parse().expect("Invalid end number");

            start..=end
        })
        .collect()
}

fn part1(ranges: &[RangeInclusive<u64>]) {
    let invalid_ids_sum: u64 = ranges
        .iter()
        .cloned()
        .flatten()
        .map(|x| (x, x.to_string()))
        .filter(|(_, s)| s.len() % 2 == 0) // Only even-length strings can be A+A
        .filter(|(_, s)| {
            let (left, right) = s.split_at(s.len() / 2);
            left == right
        })
        .map(|(x, _)| x)
        .sum();

    println!("Part 1: {invalid_ids_sum}");
}

fn part2(ranges: &[RangeInclusive<u64>]) {
    let mut invalid_ids_sum = 0;
    'range_loop: for (x, x_str) in ranges.iter().cloned().flatten().map(|x| (x, x.to_string())) {
        let mut i = x_str.len();
        while i > 1 {
            if x_str.len() % i == 0 {
                let parts: Vec<String> = x_str
                    .chars()
                    .collect::<Vec<_>>()
                    .chunks(x_str.len() / i)
                    .map(|chunk| chunk.iter().collect())
                    .collect();

                let first_part = &parts[0];
                for (i, part) in parts.iter().skip(1).enumerate() {
                    if part != first_part {
                        break;
                    }

                    if i == parts.len() - 2 {
                        invalid_ids_sum += x;
                        continue 'range_loop;
                    }
                }
            }
            i -= 1;
        }
    }

    println!("Part 2: {invalid_ids_sum}")
}
