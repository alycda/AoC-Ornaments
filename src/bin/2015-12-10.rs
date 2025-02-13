//! Day 10: Elves Look, Elves Say

use std::str::FromStr;

use aoc_ornaments::{Part, ArgSolution};
use itertools::Itertools;

/// char, count
#[derive(Debug, derive_more::Deref)]
struct Day(Vec<(u64, char)>);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let mut peekable = input.chars().peekable();
        let mut current_count = 1;
        let mut result = Vec::new();

        while let Some(c) = peekable.next() {

            if let Some(next) = peekable.peek() {
                match(c, next) {
                    (c, next) if c == *next => {
                        current_count += 1;
                    }
                    (c, _) => {
                        result.push((current_count, c));
                        current_count = 1
                    }
                }
            } else {
                result.push((current_count, c));
            }

        }

        Ok(Self(result))
    }
}

impl Day {
    fn next_sequence(&self) -> Self {
        // Convert our current state to a string to use group_by
        let string_rep = self.to_string();
        
        // Use group_by to collect runs of same digit
        let result = string_rep.chars()
            .chunk_by(|&x| x) // Group consecutive same chars
            .into_iter()
            .map(|(c, group)| {
                let count = group.count() as u64;
                (count, c)
            })
            .collect();

        Self(result)
    }

    fn compute(&self, count: usize) -> usize {
        let mut current = Day(self.to_vec());
        for _ in 0..count {
            current = current.next_sequence();
        }
        current.0.len() * 2
    }
}

impl ArgSolution<usize> for Day {
    type Output = usize;

    fn solve(&mut self, _part: Part, count: usize) -> aoc_ornaments::SolutionResult<String> {
        // off by one because the first iteration is handled in parse
        Ok(self.compute(count - 1).to_string())
    }
}

impl ToString for Day {
    fn to_string(&self) -> String {
        self.iter().map(|(count, digit)| format!("{}{}", count, digit)).collect()
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("../inputs/2015-12-10.txt"))?;
    let part1 = day.solve(Part::One, 40)?;
    let part2 = day.solve(Part::Two, 50)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("1", 11)]
    #[case("11", 21)]
    #[case("21", 1211)]
    #[case("1211", 111221)]
    #[case("111221", 312211)]
    fn test_cases_part1(#[case] input: &str, #[case] expected: i32) {
        let day = Day::from_str(input).unwrap();

        assert_eq!(day.to_string(), expected.to_string());
    }
}