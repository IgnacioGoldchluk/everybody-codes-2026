use crate::solutions::solution;

pub struct Day1Solver;

fn sum_pines(input: &str, mapping: fn(u32) -> u32) -> String {
    input
        .trim()
        .split(",")
        .map(|num| num.parse::<u32>().unwrap())
        .map(mapping)
        .sum::<u32>()
        .to_string()
}

fn part1(input: &str) -> String {
    sum_pines(input, next_square_distance)
}

/// Returns the distance to the next perfect square
fn next_square_distance(num: u32) -> u32 {
    let side = num.isqrt();
    if side * side == num {
        0
    } else {
        (side + 1) * (side + 1) - num
    }
}

/// Returns the distance to the next frame perimeter
fn next_perimeter_distance(num: u32) -> u32 {
    num.div_ceil(4).max(2) * 4 - num
}

/// Returns the distance to the next almost filled square
fn next_almost_filled_square(num: u32) -> u32 {
    // Make sure it's always odd
    let side = (num.isqrt() + 1) | 1;
    side * side - num - 1
}

fn part2(input: &str) -> String {
    sum_pines(input, next_perimeter_distance)
}

fn part3(input: &str) -> String {
    sum_pines(input, next_almost_filled_square)
}

impl solution::Solver for Day1Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1),
            part2: part2(&input.part2),
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
        let p1 = "3,11,12,16,24,25";
        let p2 = "3,11,12,16,24,25";
        let p3 = "3,11,12,16,24,25";
        let input = solution::Input {
            part1: p1.into(),
            part2: p2.into(),
            part3: p3.into(),
        };

        let solution = Day1Solver.solve(input);
        assert_eq!(solution.part1, "11");
        assert_eq!(solution.part2, "9");
        assert_eq!(solution.part3, "61");
    }
}
