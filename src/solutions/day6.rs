use std::collections::{HashMap, HashSet};

use crate::solutions::solution;

pub struct Day6Solver;

fn part2(input: &str) -> String {
    (seen_ingredients(input, "NULL", true).len() - 1).to_string()
}

fn part1(input: &str) -> String {
    ((seen_ingredients(input, "NULL", false)).len() - 1).to_string()
}

fn part3(input: &str) -> String {
    let nulls = ["NULLv1", "NULLv2", "NULLv3"].map(|null| seen_ingredients(input, null, true));

    let candidates: HashSet<&str> = nulls.iter().flatten().copied().collect();

    candidates
        .iter()
        .filter(|candidate| {
            nulls
                .iter()
                .filter(|seen| seen.contains(*candidate))
                .count()
                >= 2
        })
        .count()
        .to_string()
}

fn seen_ingredients<'a>(input: &'a str, start: &'a str, reverse_graph: bool) -> HashSet<&'a str> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    input.lines().for_each(|line| {
        let (product, ingredients) = line.split_once(":").unwrap();

        for ingredient in ingredients.split(",") {
            let (product, req) = if reverse_graph {
                (ingredient, product)
            } else {
                (product, ingredient)
            };

            graph.entry(product).or_default().push(req);
        }
    });

    let mut queue = Vec::from([start]);
    let mut seen = HashSet::new();

    while let Some(ingredient) = queue.pop() {
        seen.insert(ingredient);
        if let Some(new_ingredients) = graph.get(ingredient) {
            for new_ingredient in new_ingredients {
                if !seen.contains(*new_ingredient) {
                    queue.push(new_ingredient)
                }
            }
        }
    }

    seen
}

impl solution::Solver for Day6Solver {
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
        let p1 = r#"BRDS:FLWR,SLTT,YSTS
FLWR:SLTT,YSTS
SLTT:YSTS,MLKK,BTTR
NULL:BRDS,FLWR,SLTT
YSTS:BRDS,FLWR
MLKK:SPCS,NULL
HNYN:BRDS,FLWR,SLTT,YSTS"#;

        let p3 = r#"MEAL:BRDS,CAKE,DOUG
BRDS:DOUG,FLWR
CAKE:DOUG,FLWR,NULLv2,NULLv3
DOUG:FLWR,YSTS,SLTT,NULLv1
FLWR:BRDS,DOUG
YSTS:SPCS
SOUP:BRDS,CAKE,DOUG,FLWR"#;

        let input = solution::Input {
            part1: p1.into(),
            part2: p1.into(),
            part3: p3.into(),
        };

        let solution = Day6Solver.solve(input);
        assert_eq!(solution.part1, "7");
        assert_eq!(solution.part2, "6");
        assert_eq!(solution.part3, "3");
    }
}
