use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

type Board = HashMap<(i16, i16), bool>;

#[aoc_generator(day4)]
fn parse(input: &str) -> Board {
    let mut board = Board::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            if c == '@' {
                board.insert((x as i16, y as i16), true);
            }
        }
    }
    board
}

fn to_remove(input: &Board) -> Vec<(i16, i16)> {
    let mut pos: Vec<(i16, i16)> = Vec::new();

    for ((x, y), c) in input.iter() {
        if !*c {
            continue;
        }

        let options: Vec<(i16, i16)> = vec![
            ((x + 1), *y),
            ((x - 1), *y),
            (*x, (y + 1)),
            (*x, (y - 1)),
            ((x + 1), (y - 1)),
            ((x - 1), (y + 1)),
            ((x + 1), (y + 1)),
            ((x - 1), (y - 1)),
        ];

        let around = options.iter().fold(
            0,
            |acc, k| {
                if input.contains_key(k) {
                    acc + 1
                } else {
                    acc
                }
            },
        );

        if around < 4 {
            pos.push((*x, *y));
        }
    }
    pos
}

#[aoc(day4, part1)]
fn part1(input: &Board) -> String {
    to_remove(input).len().to_string()
}

#[aoc(day4, part2)]
fn part2(input: &Board) -> String {
    let mut next: Board = input.clone();

    let mut total = 0;

    loop {
        let removed = to_remove(&next);
        if removed.is_empty() {
            break;
        }
        total += removed.len();
        for (x, y) in removed {
            next.remove(&(x, y));
        }
    }
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "13");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "43");
    }
}
