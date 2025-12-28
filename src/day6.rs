use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<Problem> {
    let mut problems = Vec::<Problem>::new();
    let mut vertical = Vec::<String>::new();

    input.lines().for_each(|line| {
        if line.contains("*") || line.contains("+") {
            // ops
            line.trim()
                .split_ascii_whitespace()
                .enumerate()
                .for_each(|(i, op)| match op {
                    "+" => problems[i].op = Op::Plus,
                    "*" => problems[i].op = Op::Mul,
                    _ => panic!("unknown operator"),
                });
        } else {
            // init verticals
            if vertical.is_empty() {
                vertical.resize(line.len(), "".to_string());
            }

            for (i, c) in line.chars().enumerate() {
                vertical[i].push(c);
            }

            line.split_ascii_whitespace()
                .enumerate()
                .for_each(|(i, number)| {
                    while problems.len() < i + 1 {
                        problems.push(Problem {
                            args: vec![],
                            args_vertical: vec![],
                            op: Op::Mul,
                        })
                    }

                    problems[i].args.push(number.parse::<u64>().unwrap());
                });
        }
    });

    vertical.iter().fold(0, |i, number| {
        if number.trim().is_empty() {
            i + 1
        } else {
            problems[i]
                .args_vertical
                .push(number.trim().parse::<u64>().unwrap());
            i
        }
    });

    problems
}

#[derive(Debug)]
struct Problem {
    args: Vec<u64>,
    args_vertical: Vec<u64>,
    op: Op,
}

#[derive(Debug)]
enum Op {
    Mul,
    Plus,
}

#[aoc(day6, part1)]
fn part1(input: &Vec<Problem>) -> String {
    // println!("{:?}", input);

    input
        .iter()
        .fold(0, |acc, problem| match problem.op {
            Op::Mul => acc + problem.args.iter().fold(1, |a, b| a * *b),
            Op::Plus => acc + problem.args.iter().fold(0, |a, b| a + *b),
        })
        .to_string()
}

#[aoc(day6, part2)]
fn part2(input: &Vec<Problem>) -> String {
    input
        .iter()
        .fold(0, |acc, problem| match problem.op {
            Op::Mul => acc + problem.args_vertical.iter().fold(1, |a, b| a * *b),
            Op::Plus => acc + problem.args_vertical.iter().fold(0, |a, b| a + *b),
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                &vec![
                    "123 328  51 64 ",
                    " 45 64  387 23 ",
                    "  6 98  215 314",
                    "*   +   *   +  "
                ]
                .join("\n")
            )),
            "4277556"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                &vec![
                    "123 328  51 64 ",
                    " 45 64  387 23 ",
                    "  6 98  215 314",
                    "*   +   *   +  "
                ]
                .join("\n")
            )),
            "3263827"
        );
    }
}
