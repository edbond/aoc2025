use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Vec<u64>> {
    // split to lines
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c as u64 - (b'0' as u64))
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>()
}

fn max_joltage(input: &Vec<u64>) -> usize {
    let mut max: usize = 0;

    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let n = (input[i] * 10 + input[j]) as usize;
            if n > max {
                max = n;
            }
        }
    }
    max
}

fn max_joltage_rec(input: &Vec<u64>, len: &u64) -> u64 {
    // between 0 and input.len() - len - 1 find leftmost maximum digit
    max_rec_helper(input, 0, *len, vec![])
}

fn max_rec_helper(
    input: &Vec<u64>,
    start_idx: usize,
    remaining: u64,
    mut current_idx: Vec<u8>,
) -> u64 {
    if remaining == 0 {
        let n = current_idx
            .iter()
            .fold(0, |acc, &x| acc * 10 + input[x as usize] as u64);
        return n;
    }

    // find leftmost maximum digit
    let end_idx = (input.len()) as usize - remaining as usize;
    let (i, digit) = input[start_idx..=end_idx]
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|(_, &digit)| digit)
        .unwrap();

    // 811111111111119
    // 012345678901234
    // println!(
    //     "start_idx: {}, end_idx: {}, i: {}, digit: {}",
    //     start_idx, end_idx, i, digit
    // );

    current_idx.push((start_idx + i) as u8);

    max_rec_helper(input, start_idx + i + 1, remaining - 1, current_idx)
}

// 10^13 =         10,000,000,000,000
// 2^32  =              4,294,967,296
// 2^64  = 18,446,744,073,709,551,616

#[aoc(day3, part1)]
fn part1(input: &Vec<Vec<u64>>) -> String {
    input
        .into_iter()
        .fold(0, |acc, row| acc + max_joltage(&row))
        .to_string()
}

#[aoc(day3, part2)]
fn part2(input: &Vec<Vec<u64>>) -> String {
    input
        .into_iter()
        .fold(0, |acc, row| acc + max_joltage_rec(&row, &12))
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "987654321111111
        811111111111119
        234234234234278
        818181911112111"
            )),
            "357"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "987654321111111
        811111111111119
        234234234234278
        818181911112111"
            )),
            "3121910778619"
        );
    }

    #[test]
    fn part2_single() {
        assert_eq!(part2(&parse("987654321111111")), "987654321111");
        assert_eq!(part2(&parse("811111111111119")), "811111111119");
        assert_eq!(part2(&parse("234234234234278")), "434234234278");
        assert_eq!(part2(&parse("818181911112111")), "888911112111");
    }
}
