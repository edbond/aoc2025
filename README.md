# Advent of Code 2025

My solutions for [Advent of Code 2025](https://adventofcode.com/2025) written in Rust.

## About

Advent of Code is an annual set of Christmas-themed programming challenges that follow an Advent calendar. Each day from December 1st to December 25th, a new two-part puzzle is released.

## Structure

- `src/` - Solution implementations
  - `day1.rs` - Day 1: Secret Entrance
  - `day2.rs` - Day 2: Invalid ID Detection
  - `lib.rs` - Library setup with aoc-runner
- `input/` - Puzzle inputs (not committed to git)

## Running Solutions

```bash
# Run all solutions
cargo aoc

# Run a specific day
cargo aoc -d 1

# Run benchmarks
cargo aoc bench
```

## Testing

```bash
# Run all tests
cargo test

# Run tests for a specific day
cargo test day1
cargo test day2
```

## Progress

- [x] Day 1: Secret Entrance ⭐⭐
- [x] Day 2: Invalid ID Detection ⭐⭐

## Tools

- **Language**: Rust
- **Framework**: [cargo-aoc](https://github.com/gobanos/cargo-aoc)
- **Dependencies**:
  - `aoc-runner` - Advent of Code runner framework
  - `aoc-runner-derive` - Macros for automatic solution discovery
  - `anyhow` - Error handling

## License

This is personal learning code for Advent of Code challenges.
