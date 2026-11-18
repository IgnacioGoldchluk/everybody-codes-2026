use crate::solutions::solution;
use regex::Regex;

pub struct Day11Solver;

fn part2(input: &str) -> String {
    let (portals, connections) = portals_connections(input);

    let initial_strain = portal_distances(portals, 3, &[]);
    let base_dist = distance_matrix(portals, 3, &connections);

    let mut min_teleportations = usize::MAX;
    for u in 0..portals {
        for v in (u + 1)..portals {
            let mut total = 0;
            for i in 0..portals {
                for j in 0..portals {
                    let d = base_dist[i][j]
                        .min(base_dist[i][u] + 1 + base_dist[v][j])
                        .min(base_dist[i][v] + 1 + base_dist[u][j]);
                    total += d;
                }
            }
            min_teleportations = min_teleportations.min(total);
        }
    }

    (initial_strain - min_teleportations).to_string()
}

fn part1(input: &str) -> String {
    let (portals, connections) = portals_connections(input);

    (base_distance(portals) - portal_distances(portals, 1, &connections)).to_string()
}

fn portals_connections(input: &str) -> (usize, Vec<(usize, usize)>) {
    let (portal, connections) = input.split_once("\n\n").unwrap();
    let re = Regex::new(r"portals: (\d+)").unwrap();
    let num_portals: usize = re.captures(portal).unwrap()[1].parse().unwrap();

    (num_portals, parse_connections(connections))
}

fn distance_matrix(
    num_portals: usize,
    connected_neighbors: usize,
    connections: &[(usize, usize)],
) -> Vec<Vec<usize>> {
    // Use usize::MAX / 2 to avoid overflow
    let mut distances = vec![vec![usize::MAX / 2; num_portals]; num_portals];
    for i in 0..num_portals {
        distances[i][i] = 0;

        for neighbor in 1..=connected_neighbors {
            distances[i][(i + neighbor) % num_portals] = 1;
            distances[i][(i + num_portals - neighbor) % num_portals] = 1;
        }
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

    distances
}

fn portal_distances(
    num_portals: usize,
    connected_neighbors: usize,
    connections: &[(usize, usize)],
) -> usize {
    distance_matrix(num_portals, connected_neighbors, connections)
        .iter()
        .flatten()
        .sum()
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
        let p1 = r#"portals: 10

5 10
2 7"#;

        let input = solution::Input {
            part1: p1.into(),
            part2: p1.into(),
            part3: "".into(),
        };

        let solution = Day11Solver.solve(input);
        assert_eq!(solution.part1, "60");
        assert_eq!(solution.part2, "6");
        assert_eq!(solution.part3, "");
    }
}
