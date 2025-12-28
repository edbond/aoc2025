use std::{collections::HashSet, ops::RangeInclusive};

use aoc_runner_derive::{aoc, aoc_generator};

type IngredientsDatabase = Vec<(u64, u64)>;

struct Day5Input {
    ingredients_database: IngredientsDatabase,
    available_ingredients: Vec<u64>,
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Day5Input {
    enum Mode {
        Database,
        Ingredients,
    }

    let mut mode = Mode::Database;
    let mut ingredients_database = IngredientsDatabase::new();
    let mut available_ingredients = Vec::new();

    for line in input.lines().map(|line| line.trim()) {
        match mode {
            Mode::Database => {
                if line == "" {
                    mode = Mode::Ingredients;
                } else {
                    let range = line.split_once('-').unwrap();
                    let start = range.0.parse().unwrap();
                    let end = range.1.parse().unwrap();
                    ingredients_database.push((start, end));
                }
            }
            Mode::Ingredients => {
                available_ingredients.push(line.parse().unwrap());
            }
        }
    }

    Day5Input {
        ingredients_database,
        available_ingredients,
    }
}

#[aoc(day5, part1)]
fn part1(input: &Day5Input) -> String {
    let mut result = 0;
    for i in input.available_ingredients.iter() {
        for (start, end) in &input.ingredients_database {
            if i >= start && i <= end {
                result += 1;
                break;
            }
        }
    }
    result.to_string()
}

#[aoc(day5, part2)]
fn part2(input: &Day5Input) -> String {
    // sort intervals by start
    let mut intervals = input.ingredients_database.clone();
    intervals.sort_by_key(|(start, _)| *start);

    #[derive(Debug)]
    struct Acc {
        current: (u64, u64),
        result: Vec<(u64, u64)>,
    }

    // fold over intervals and see if we can combine current with next one
    let mut acc = intervals.iter().fold(
        Acc {
            current: intervals[0],
            result: vec![],
        },
        |mut acc, (start, end)| {
            // if next starts before current ends
            // we have overlap and need to extend current one
            // join 2 intervals
            if *start <= acc.current.1 {
                acc.current.1 = acc.current.1.max(*end);
            } else {
                // no overlap, push current to acc results
                // and assign new current
                acc.result.push(acc.current);
                acc.current = (*start, *end);
            }
            acc
        },
    );

    acc.result.push(acc.current);

    // println!("acc: {:?}", acc);

    acc.result
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT)), "3");
    }

    #[test]
    fn part2_tests() {
        struct TestCase<'a> {
            input: &'a str,
            want: &'a str,
        }

        let test_cases = vec![
            TestCase {
                input: "3-5",
                want: "3",
            },
            TestCase {
                input: "3-5
            4-6",
                want: "4",
            },
            TestCase {
                input: "3-5
                8-10",
                want: "6",
            },
        ];

        for tc in test_cases {
            assert_eq!(part2(&parse(tc.input)), tc.want);
        }
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "3-5
        10-14
        16-20
        12-18"
            )),
            "14"
        );
    }
}
