use anyhow::{anyhow, Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq)]
struct Rotation {
    direction: Direction,
    amount: i32,
}

#[aoc_generator(day1)]
fn parse(input: &str) -> Result<Vec<Rotation>> {
    input
        .lines()
        .map(|line| {
            let first_char = line.chars().next().context("Empty line in input")?;

            let direction = match first_char {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => return Err(anyhow!("Invalid direction: {}", first_char)),
            };

            let amount = line[1..]
                .parse::<i32>()
                .context(format!("Failed to parse amount from '{}'", &line[1..]))?;

            Ok(Rotation { direction, amount })
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct State {
    pos: i32,
    zeroes: i32,
}

#[aoc(day1, part1)]
fn part1(input: &[Rotation]) -> String {
    let state = State { pos: 50, zeroes: 0 };

    input
        .iter()
        .fold(state, |mut state, rotation| {
            let next_pos = match rotation.direction {
                Direction::Left => state.pos - rotation.amount,
                Direction::Right => state.pos + rotation.amount,
            } % 100;

            if next_pos == 0 {
                state.zeroes += 1
            }

            state.pos = next_pos;
            state
        })
        .zeroes
        .to_string()
}

#[aoc(day1, part2)]
fn part2(input: &[Rotation]) -> String {
    let state = State { pos: 50, zeroes: 0 };

    input
        .iter()
        .fold(state, |mut state, rotation| {
            let next_pos = match rotation.direction {
                Direction::Left => state.pos - rotation.amount,
                Direction::Right => state.pos + rotation.amount,
            }
            .rem_euclid(100);

            let zero_crosses = match rotation.direction {
                Direction::Right => {
                    // Count how many times we pass through 0 going right
                    (state.pos + rotation.amount) / 100
                }
                Direction::Left => {
                    // Count how many times we pass through 0 going left
                    if state.pos == 0 {
                        // Starting at 0, we only cross 0 again after 100 steps
                        rotation.amount / 100
                    } else if rotation.amount <= state.pos {
                        // No wrap-around, only count if we land exactly on 0
                        if (state.pos - rotation.amount) == 0 {
                            1
                        } else {
                            0
                        }
                    } else {
                        // Wrap-around: first crossing + additional full rotations
                        let steps_past_zero = rotation.amount - state.pos;
                        1 + steps_past_zero / 100
                    }
                }
            };

            state.zeroes += zero_crosses;

            state.pos = next_pos;

            state
        })
        .zeroes
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn part1_example() {
        assert_eq!(
            parse("L68").unwrap(),
            vec![Rotation {
                direction: Direction::Left,
                amount: 68
            }]
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE).unwrap()), "6");
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(part2(&parse("R1000").unwrap()), "10");
    }

    #[test]
    fn part2_example_3() {
        assert_eq!(part2(&parse("L50\nR100").unwrap()), "2");
    }
}
