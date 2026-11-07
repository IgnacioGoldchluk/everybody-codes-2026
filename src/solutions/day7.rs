use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::LazyLock;

use crate::solutions::solution;

pub struct Day7Solver;

static CLOUD_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^id=(\d+), energy=(\d+), connections=(\d+(?:,\d+)*)$").unwrap());

#[derive(Clone)]
struct Cloud {
    id: usize,
    energy: u32,
    connections: Vec<usize>,
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
    let clouds = parse_clouds(input);
    let cloud = clouds.iter().max_by_key(|c| c.connections.len()).unwrap();

    apply_energy(&clouds, 5, cloud.id, u32::MAX).to_string()
}

fn part2(input: &str) -> String {
    let clouds = parse_clouds(input);
    let cloud = clouds.iter().max_by_key(|c| c.connections.len()).unwrap();

    apply_energy(&clouds, 10, cloud.id, u32::MAX).to_string()
}

fn part3(input: &str) -> String {
    let clouds = parse_clouds(input);

    clouds
        .iter()
        .map(|cloud| apply_energy(&clouds, 10, cloud.id, part3_max_energy()))
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
fn part3_max_energy() -> u32 {
    100
}

#[cfg(not(test))]
fn part3_max_energy() -> u32 {
    1_000_000
}

fn parse_clouds(input: &str) -> Vec<Cloud> {
    input
        .lines()
        .map(Cloud::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn apply_energy(
    clouds: &[Cloud],
    initial_energy: u32,
    initial_cloud_id: usize,
    max_energy: u32,
) -> u32 {
    let mut energies: Vec<(u32, bool)> = clouds.iter().map(|c| (c.energy, false)).collect();

    let mut current: HashMap<(u32, usize), u32> =
        HashMap::from([((initial_energy, initial_cloud_id), 1)]);
    let mut next: HashMap<(u32, usize), u32> = HashMap::new();

    while !current.is_empty() {
        for ((energy, id), count) in current.drain() {
            if let Some((existing_energy, false)) = energies.get_mut(id - 1) {
                *existing_energy += energy * count;

                if energy > 1 {
                    for connection in &clouds[id - 1].connections {
                        if !energies[connection - 1].1 {
                            *next.entry((energy - 1, *connection)).or_insert(0) += count;
                        }
                    }
                }
            }
        }

        for (idx, (energy, already_exploded)) in energies.iter().enumerate() {
            if *energy >= max_energy && !*already_exploded {
                for connection in &clouds[idx].connections {
                    if clouds[connection - 1].energy < max_energy {
                        *next.entry((10, *connection)).or_insert(0) += 1;
                    }
                }
            }
        }

        for (energy, exploded) in energies.iter_mut() {
            *exploded = *energy >= max_energy;
        }

        (current, next) = (next, current);
    }

    energies
        .iter()
        .filter_map(|(e, exploded)| if !*exploded { Some(e) } else { None })
        .sum()
}

impl solution::Solver for Day7Solver {
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

        let p3 = r#"id=1, energy=5, connections=2,5,7
id=2, energy=9, connections=6,7
id=3, energy=3, connections=4
id=4, energy=8, connections=3
id=5, energy=1, connections=6,7
id=6, energy=7, connections=1,7
id=7, energy=6, connections=5"#;

        let input = solution::Input {
            part1: p1.into(),
            part2: p2.into(),
            part3: p3.into(),
        };

        let solution = Day7Solver.solve(input);
        assert_eq!(solution.part1, "102");
        assert_eq!(solution.part2, "1214633");
        assert_eq!(solution.part3, "187");
    }
}
