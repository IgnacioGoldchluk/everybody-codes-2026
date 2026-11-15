use crate::solutions::solution;
use regex::Regex;

pub struct Day11Solver;

fn part1(input: &str) -> String {
    let (portals, connections) = portals_connections(input);

    (base_distance(portals) - portal_distances(portals, &connections)).to_string()
}

fn portals_connections(input: &str) -> (usize, Vec<(usize, usize)>) {
    let (portal, connections) = input.split_once("\n\n").unwrap();
    let re = Regex::new(r"portals: (\d+)").unwrap();
    let num_portals: usize = re.captures(portal).unwrap()[1].parse().unwrap();

    (num_portals, parse_connections(connections))
}

fn portal_distances(num_portals: usize, connections: &[(usize, usize)]) -> usize {
    // Use usize::MAX / 2 to avoid overflow
    let mut distances = vec![vec![usize::MAX / 2; num_portals]; num_portals];
    for i in 0..num_portals {
        distances[i][i] = 0;
        distances[i][(i + 1) % num_portals] = 1;
        distances[i][(i + num_portals - 1) % num_portals] = 1;
    }

    for &(from, to) in connections {
        distances[from][to] = 1;
        distances[to][from] = 1;
    }

    // Floyd-Warshall
    for k in 0..num_portals {
        for i in 0..num_portals {
            for j in 0..num_portals {
                distances[i][j] = distances[i][j].min(distances[i][k] + distances[k][j]);
            }
        }
    }

    distances.iter().flatten().sum()
}

fn parse_connections(connections: &str) -> Vec<(usize, usize)> {
    connections
        .lines()
        .map(|line| {
            let (portal1, portal2) = line.split_once(" ").unwrap();

            (
                portal1.parse::<usize>().unwrap() - 1,
                portal2.parse::<usize>().unwrap() - 1,
            )
        })
        .collect()
}

fn base_distance(num_portals: usize) -> usize {
    if num_portals.is_multiple_of(2) {
        num_portals.pow(3) / 4
    } else {
        num_portals * (num_portals.pow(2) - 1) / 4
    }
}

impl solution::Solver for Day11Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1),
            part2: "".into(),
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
        let p1 = r#"portals: 10

5 10
2 7"#;

        let input = solution::Input {
            part1: p1.into(),
            part2: "".into(),
            part3: "".into(),
        };

        let solution = Day11Solver.solve(input);
        assert_eq!(solution.part1, "60");
        assert_eq!(solution.part2, "");
        assert_eq!(solution.part3, "");
    }
}
