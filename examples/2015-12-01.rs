//! Day 1: Not Quite Lisp

use std::str::FromStr;

use aoc_ornaments::{Part, Solution};

/// A collection of instructions to move between floors.
#[derive(Debug, derive_more::Deref)]
struct Day(Vec<i32>);

impl FromStr for Day {
    type Err = miette::Error;

    /// Parse the input into a collection of instructions.
    /// 
    /// ## Example
    /// 
    /// - `(` moves Santa up one floor.
    /// - `)` moves Santa down one floor.
    /// 
    fn from_str(input: &str) -> miette::Result<Self> {
        let parsed = input.chars()
            .map(|c| {
                match c {
                    '(' => 1,
                    ')' => -1,
                    _ => 0,
                }
            }).collect();

        Ok(Self(parsed))
    }
}

impl Solution for Day {
    type Output = i32;

    /// Find the floor Santa ends up on.
    fn part1(&mut self) -> miette::Result<Self::Output> {
        Ok(self.iter().sum())
    }

    /// Find the position of the first instruction that causes Santa to enter the basement.
    fn part2(&mut self) -> miette::Result<Self::Output> {
        let output = self.iter()
            // iterate over the instructions maintaining state (sum of floors)
            .scan(0, |floor, &x| {
                *floor += x;
                Some(*floor)
            })
            // find the first position where Santa enters the basement
            .position(|floor| floor < 0)
            // convert the position to a 1-based index
            .map(|pos| pos as i32 + 1)
            // convert to Result
            .ok_or_else(|| miette::miette!("Santa never enters the basement"))?;

        Ok(output)
    }
}

/// Run Part 1 and Part 2.
fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-01.txt"))?;
    let part1 = day.solve(Part::One)?;
    let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("(())", 0)]
    #[case("()()", 0)]
    #[case("(((", 3)]
    #[case("(()(()(", 3)]
    #[case("))(((((", 3)]
    #[case("())", -1)]
    #[case("))(", -1)]
    #[case(")))", -3)]
    #[case(")())())", -3)]
    fn test_cases_part1(#[case] input: &str, #[case] expected: i32) -> miette::Result<()> {
        let mut day = Day::from_str(input)?;
        assert_eq!(day.solve(Part::One)?, expected.to_string());

        Ok(())
    }

    #[rstest]
    #[case(")", 1)]
    #[case("()())", 5)]
    fn test_cases_part2(#[case] input: &str, #[case] expected: i32) -> miette::Result<()> {
        let mut day = Day::from_str(input)?;
        assert_eq!(day.solve(Part::Two)?, expected.to_string());

        Ok(())
    }
}
