use regex::Regex;
use std::collections::VecDeque;
use std::str::FromStr;
use std::sync::LazyLock;

use crate::solutions::solution;

pub struct Day7Solver;

static CLOUD_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^id=(\d+), energy=(\d+), connections=(\d+(?:,\d+)*)$").unwrap());

struct Cloud {
    id: u32,
    energy: u32,
    connections: Vec<u32>,
}

impl FromStr for Cloud {
    type Err = std::convert::Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let caps = CLOUD_REGEX.captures(line).unwrap();

        let id = caps[1].parse().unwrap();
        let energy = caps[2].parse().unwrap();

        let connections = caps[3].split(',').map(|s| s.parse().unwrap()).collect();

        Ok(Cloud {
            id,
            energy,
            connections,
        })
    }
}

fn part1(input: &str) -> String {
    apply_energy(input, 5)
}

fn part2(input: &str) -> String {
    apply_energy(input, 10)
}

fn apply_energy(input: &str, initial_energy: u32) -> String {
    let mut clouds: Vec<Cloud> = input
        .lines()
        .map(Cloud::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let start = clouds.iter().max_by_key(|cl| cl.connections.len()).unwrap();

    let mut energies: VecDeque<(u32, u32)> = VecDeque::from([(initial_energy, start.id)]);

    while let Some((energy, id)) = energies.pop_front() {
        let cloud = clouds.get_mut(id as usize - 1).unwrap();
        cloud.energy += energy;

        if energy > 1 {
            energies.extend(cloud.connections.iter().map(|c| (energy - 1, *c)));
        }
    }

    clouds.iter().map(|c| c.energy).sum::<u32>().to_string()
}

impl solution::Solver for Day7Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1),
            part2: part2(&input.part2),
            part3: "".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use super::*;

    #[test]
    fn test_solve() {
        let p1 = r#"id=1, energy=5, connections=2,5,7
id=2, energy=9, connections=6,7
id=3, energy=3, connections=4
id=4, energy=8, connections=3
id=5, energy=1, connections=6,7
id=6, energy=7, connections=1,7
id=7, energy=6, connections=5"#;

        let p2 = r#"id=1, energy=5, connections=2,3,5,7
id=2, energy=9, connections=1,3,4,5,6,7
id=3, energy=3, connections=1,2,4,6,7
id=4, energy=8, connections=1,2,7
id=5, energy=1, connections=2,3,4,6
id=6, energy=7, connections=1,2,3,4
id=7, energy=6, connections=1,2,4,6"#;

        let input = solution::Input {
            part1: p1.into(),
            part2: p2.into(),
            part3: "".into(),
        };

        let solution = Day7Solver.solve(input);
        assert_eq!(solution.part1, "102");
        assert_eq!(solution.part2, "1214633");
        assert_eq!(solution.part3, "");
    }
}
