use crate::solutions::solution;

pub struct Day3Solver;

fn part1(input: &str) -> String {
    input
        .lines()
        .map(to_decimal_notation)
        .sum::<u64>()
        .to_string()
}

fn part3(input: &str) -> String {
    input
        .lines()
        .map(to_decimal_notation_pt3)
        .sum::<u64>()
        .to_string()
}

fn to_decimal_notation_pt3(input: &str) -> u64 {
    let base_weighted: Vec<u64> = input
        .strip_prefix("ISO2-")
        .unwrap()
        .split(" ")
        .map(|d| d.parse::<u64>().unwrap())
        .collect();

    if base_weighted.len() != 2 {
        panic!("Expected two values, got {:?}", base_weighted);
    }

    let base: Vec<u8> = to_reverse_digits(base_weighted[0]);
    let weights: Vec<u8> = to_reverse_digits(base_weighted[1]);

    let value = decimal_value(&weights, &base);

    for lower_weighted in (0..base_weighted[1]).rev() {
        if decimal_value(&to_reverse_digits(lower_weighted), &base) == value {
            return 0;
        }
    }
    value
}

fn to_decimal_notation(votes: &str) -> u64 {
    let base_weighted: Vec<u64> = votes
        .strip_prefix("ISO-")
        .unwrap()
        .split(" ")
        .map(|d| d.parse::<u64>().unwrap())
        .collect();

    if base_weighted.len() != 2 {
        panic!("Expected two values, got {:?}", base_weighted);
    }

    let base: Vec<u8> = to_reverse_digits(base_weighted[0]);
    let weights: Vec<u8> = to_reverse_digits(base_weighted[1]);

    decimal_value(&weights, &base)
}

fn decimal_value(weights: &[u8], base: &[u8]) -> u64 {
    weights.iter().enumerate().fold(0, |acc, (idx, val)| {
        let b_digit = base[idx % base.len()] as u64;
        acc + (*val as u64) * (b_digit.pow(idx as u32))
    })
}

fn to_reverse_digits(value: u64) -> Vec<u8> {
    value
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as u8)
        .rev()
        .collect()
}

impl solution::Solver for Day3Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1),
            part2: part1(&input.part2),
            part3: part3(&input.part3),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use super::*;

    #[test]
    fn test_solve() {
        let p1 = r#"ISO-3210 7654
ISO-8742 2468
ISO-1111 1111"#;

        let p2 = r#"ISO-3210 7654
ISO-8742 2468
ISO-1 1111
ISO-13 777
ISO-34 140856
ISO-789 2465112
ISO-2341 945"#;

        let p3 = r#"ISO2-3 23
ISO2-61 149
ISO2-978 99
ISO2-5887 5923
ISO2-76988 123749
ISO2-435657 929977
ISO2-3949 870737"#;

        let input = solution::Input {
            part1: p1.into(),
            part2: p2.into(),
            part3: p3.into(),
        };

        let solution = Day3Solver.solve(input);
        assert_eq!(solution.part1, "1478");
        assert_eq!(solution.part2, "1161463");
        assert_eq!(solution.part3, "50987");
    }
}
