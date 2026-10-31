use std::collections::HashSet;

use crate::solutions::solution;

pub struct Day2Solver;

fn part1(input: &str) -> String {
    input.lines().map(to_blade_number).sum::<u64>().to_string()
}

fn to_blade_number(line: &str) -> u64 {
    // Algorithm copied from https://github.com/maneatingape/everybody-codes-rust/blob/main/src/event2026/quest02.rs
    let mut digits = HashSet::new();
    let mut harder = [[true; 10]; 10];

    for comparison in line.split(" ") {
        let (greater, smaller) = greater_smaller(comparison);
        digits.extend([greater, smaller]);
        harder[smaller as usize][greater as usize] = false;
    }

    let mut number: u64 = 0;
    while let Some(&digit) = digits.iter().find(|&&smaller| {
        digits
            .iter()
            .all(|&greater| harder[smaller as usize][greater as usize])
    }) {
        digits.remove(&digit);
        number = 10 * number + (digit as u64);
    }

    number
}

fn greater_smaller(comparison: &str) -> (u8, u8) {
    let numbers = if comparison.contains(">") {
        comparison
            .split(">")
            .map(|n| n.parse::<u8>().unwrap())
            .collect::<Vec<u8>>()
    } else {
        comparison
            .split("<")
            .map(|n| n.parse::<u8>().unwrap())
            .collect::<Vec<u8>>()
    };

    if numbers.len() != 2 {
        panic!("Invalid number comparison: {}", comparison);
    }

    if comparison.contains(">") {
        (numbers[0], numbers[1])
    } else {
        (numbers[1], numbers[0])
    }
}

impl solution::Solver for Day2Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1),
            part2: part1(&input.part2),
            part3: part1(&input.part3),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use super::*;

    #[test]
    fn test_solve() {
        let p1 = r#"4<3 2<5 3>1 1>2 1<5 4<1 3>2 4<2 3>5 4<5
4>1 2>1 3>2 1<5 4>5 2<5 3>1 4>3 2<4 5<3
5>1 5>2 4>1 2<4 3>2 3<4 1>2 3>5 1<3 4>5"#;

        let p2 = r#"2<5 3>1 1>2 1<5 4<1 3>2 4<2 3>5 4<5
2>1 3>2 1<5 4>5 2<5 3>1 4>3 2<4 5<3
5>1 5>2 4>1 3>2 3<4 1>2 3>5 1<3 4>5"#;

        let p3 = r#"3>1 1>2 1<5 4<2 3>5
2>1 2<5 3>1 4>3 2<4 5<3
5>1 3<4 1>2 3>5"#;

        let input = solution::Input {
            part1: p1.into(),
            part2: p2.into(),
            part3: p3.into(),
        };

        let solution = Day2Solver.solve(input);
        assert_eq!(solution.part1, "122157");
        assert_eq!(solution.part2, "122157");
        assert_eq!(solution.part3, "122157");
    }
}
