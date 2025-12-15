use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

/// Represents a range of IDs to check.
///
/// The range is inclusive on both ends: [start, end].
struct Pair {
    start: u64,
    end: u64,
}

/// Converts a number into a vector of its digit characters.
///
/// # Examples
/// - `digits_of(1234)` → `['1', '2', '3', '4']`
/// - `digits_of(99)` → `['9', '9']`
fn digits_of(id: u64) -> Vec<char> {
    id.to_string().chars().collect()
}

#[aoc_generator(day2)]
fn parse(input: &str) -> anyhow::Result<Vec<Pair>> {
    input
        .split(',')
        .map(|pair| {
            let i = pair.find('-').context("Failed to find delimiter '-'")?;
            Ok(Pair {
                start: pair[..i]
                    .parse::<u64>()
                    .context("Failed to parse start")?,
                end: pair[i + 1..]
                    .parse::<u64>()
                    .context("Failed to parse end")?,
            })
        })
        .collect()
}

/// Checks if a number's digits can be split in half with both halves equal.
///
/// # Examples
/// - `1010` → true (splits into "10" and "10")
/// - `1188511885` → true (splits into "11885" and "11885")
/// - `1234` → false (splits into "12" and "34" which are different)
/// - `123` → false (odd number of digits, can't be split evenly)
fn has_repeating_halves(id: u64) -> bool {
    let digits = digits_of(id);

    if digits.len() % 2 != 0 {
        return false;
    }

    let mid = digits.len() / 2;
    let left = &digits[..mid];
    let right = &digits[mid..];
    left == right
}

/// Checks if a number's digits form a repeating pattern.
///
/// Returns true if the digits can be split into equal-length chunks where all chunks are identical.
///
/// # Examples
/// - `123123123` → true (repeating pattern "123")
/// - `1212` → true (repeating pattern "12")
/// - `111` → true (repeating pattern "1")
/// - `999` → true (repeating pattern "9")
/// - `1234` → false (no repeating pattern)
fn has_repeating_pattern(id: u64) -> bool {
    let digits = digits_of(id);
    (1..digits.len())
        .filter(|&chunk_size| {
            digits.len() % chunk_size == 0
        })
        .any(|chunk_size| {
            let first = &digits[0..chunk_size];
            digits.chunks_exact(chunk_size).all(|chunk| chunk == first)
        })
}

/// Finds all IDs in the range that have repeating halves.
///
/// Returns a vector of all numbers in the range [start, end] where the digits
/// can be split in half with both halves equal.
fn find_ids_with_repeating_halves(pair: &Pair) -> Vec<u64> {
    (pair.start..=pair.end)
        .filter(|&x| has_repeating_halves(x))
        .collect()
}

/// Finds all IDs in the range that have repeating patterns.
///
/// Returns a vector of all numbers in the range [start, end] where the digits
/// form a repeating pattern.
fn find_ids_with_repeating_pattern(pair: &Pair) -> Vec<u64> {
    (pair.start..=pair.end)
        .filter(|&x| has_repeating_pattern(x))
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Pair]) -> String {
    input
        .iter()
        .flat_map(|pair| find_ids_with_repeating_halves(pair))
        .sum::<u64>()
        .to_string()
}

#[aoc(day2, part2)]
fn part2(input: &[Pair]) -> String {
    input
        .iter()
        .flat_map(|pair| find_ids_with_repeating_pattern(pair))
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_has_repeating_halves() {
        assert!(has_repeating_halves(1010));
        assert!(has_repeating_halves(1188511885));
    }

    #[test]
    fn test_has_repeating_pattern() {
        assert!(has_repeating_pattern(123123123));
        assert!(has_repeating_pattern(1212121212));
    }

    #[test]
    fn test_find_ids_with_repeating_halves() {
        assert_eq!(
            find_ids_with_repeating_halves(&Pair { start: 11, end: 22 }),
            vec![11, 22]
        );
        assert_eq!(
            find_ids_with_repeating_halves(&Pair {
                start: 95,
                end: 115
            }),
            vec![99]
        );

        // 998-1012 has one invalid ID, 1010.
        assert_eq!(
            find_ids_with_repeating_halves(&Pair {
                start: 998,
                end: 1012
            }),
            vec![1010]
        );
        // 1188511880-1188511890 has one invalid ID, 1188511885.
        assert_eq!(
            find_ids_with_repeating_halves(&Pair {
                start: 1188511880,
                end: 1188511890
            }),
            vec![1188511885]
        );
        // 222220-222224 has one invalid ID, 222222.
        assert_eq!(
            find_ids_with_repeating_halves(&Pair {
                start: 222220,
                end: 222224
            }),
            vec![222222]
        );
        // 1698522-1698528 contains no invalid IDs.
        assert_eq!(
            find_ids_with_repeating_halves(&Pair {
                start: 1698522,
                end: 1698528
            }),
            vec![]
        );
        // 446443-446449 has one invalid ID, 446446.
        assert_eq!(
            find_ids_with_repeating_halves(&Pair {
                start: 446443,
                end: 446449
            }),
            vec![446446]
        );
        // 38593856-38593862 has one invalid ID, 38593859.
        assert_eq!(
            find_ids_with_repeating_halves(&Pair {
                start: 38593856,
                end: 38593862
            }),
            vec![38593859]
        );
    }

    #[test]
    fn test_find_ids_with_repeating_pattern() {
        // 11-22 still has two invalid IDs, 11 and 22.
        assert_eq!(
            find_ids_with_repeating_pattern(&Pair { start: 11, end: 22 }),
            vec![11, 22]
        );

        // 95-115 now has two invalid IDs, 99 and 111.
        // 998-1012 now has two invalid IDs, 999 and 1010.
        assert_eq!(
            find_ids_with_repeating_pattern(&Pair {
                start: 95,
                end: 115
            }),
            vec![99, 111]
        );
        assert_eq!(
            find_ids_with_repeating_pattern(&Pair {
                start: 998,
                end: 1012
            }),
            vec![999, 1010]
        );

        // 1188511880-1188511890 still has one invalid ID, 1188511885.
        // 222220-222224 still has one invalid ID, 222222.
        // 1698522-1698528 still contains no invalid IDs.
        // 446443-446449 still has one invalid ID, 446446.
        // 38593856-38593862 still has one invalid ID, 38593859.
        // 565653-565659 now has one invalid ID, 565656.
        // 824824821-824824827 now has one invalid ID, 824824824.
        // 2121212118-2121212124 now has one invalid ID, 2121212121.
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT).unwrap()), "1227775554");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT).unwrap()), "4174379265");
    }
}
