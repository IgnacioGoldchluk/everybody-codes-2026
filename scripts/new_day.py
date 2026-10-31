import argparse
import os
import textwrap


def create_input_files(day: str):
    os.mkdir(os.path.join("inputs", day))
    for part in ["1", "2", "3"]:
        # Create empty files
        with open(os.path.join("inputs", day, part), "w"):
            pass


def create_module(day: str):
    module_template = textwrap.dedent(f"""
    use crate::solutions::solution;

    pub struct Day{day}Solver;

    impl solution::Solver for Day{day}Solver {{
        fn solve(&self, input: solution::Input) -> solution::Solution {{
            solution::Solution {{
                part1: "".into(),
                part2: "".into(),
                part3: "".into(),
            }}
        }}
    }}

    #[cfg(test)]
    mod tests {{
        use super::solution::Solver;
        use super::*;

        #[test]
        fn test_solve() {{

            let input = solution::Input {{
                part1: "".into(),
                part2: "".into(),
                part3: "".into(),
            }};

            let solution = Day{day}Solver.solve(input);
            assert_eq!(solution.part1, "");
            assert_eq!(solution.part2, "");
            assert_eq!(solution.part3, "");
        }}
    }}""")

    with open(os.path.join("src", "solutions", f"day{day}.rs"), "w") as f:
        f.write(module_template)


def add_module_import(day: str):
    with open(os.path.join("src", "solutions", "mod.rs"), "a") as f:
        f.write(f"pub mod day{day};")


def add_to_main(day: str):
    with open(os.path.join("src", "main.rs"), "r") as f:
        contents = list(f.readlines())

    to_add = f"{day} => Box::new(solutions::day{day}::Day{day}Solver {{}}),\n"

    for idx, line_contents in enumerate(contents):
        if "Unreachable" in line_contents:
            contents.insert(idx, to_add)

            with open(os.path.join("src", "main.rs"), "w") as f:
                f.writelines(contents)
            return

    raise RuntimeError("main.rs has invalid format")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--day", type=int, help="Day to add")

    namespace = parser.parse_args()
    day = namespace.day
    if day < 1 or day > 20:
        raise ValueError(f"Day must be between 1 and 20, got {day}")

    day = str(day)

    create_input_files(day)
    create_module(day)
    add_module_import(day)
    add_to_main(day)
