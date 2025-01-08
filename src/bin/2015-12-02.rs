//! Day 2: I Was Told There Would Be No Math

use std::str::FromStr;

use aoc_ornaments::{Part, Solution};

use nom::{
    character::complete::{char, u32}, multi::separated_list1, IResult
};

type Width = u32;
type Length = u32;
type Height = u32;

/// A collection of dimensions for a single present.
#[derive(Debug, derive_more::Deref)]
struct Day(Vec<(Length, Width, Height)>);

impl FromStr for Day {
    type Err = miette::Error;

    /// Parse the input into a collection of dimensions.
    /// 
    /// ## Example
    /// 
    /// - `2x3x4` has dimensions `2`, `3`, and `4`.
    /// 
    fn from_str(input: &str) -> miette::Result<Self> {
        let parsed = input.lines()
            .map(|line| {
                let (_, (l, w, h)) = Self::parse_dimensions(line).unwrap();

                (l, w, h)
            }).collect();

        Ok(Self(parsed))
    }
}

impl Day {
    fn parse_dimensions(input: &str) -> IResult<&str, (u32, u32, u32)> {
        let (input, nums) = separated_list1(char('x'), u32)(input)?;
        match nums.as_slice() {
            [l, w, h] => Ok((input, (*l, *w, *h))),
            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::LengthValue
            )))
        }
    }

    fn dimensions(sides: (u32, u32, u32)) -> u32 {
        let (l, w, h) = sides;
        let a = l * w;
        let b = w * h;
        let c = h * l;

        (2 * a) + (2 * b) + (2 * c) + [a, b, c].iter().min().unwrap()
    }

    fn ribbon((a, b, c): (u32, u32, u32)) -> u32 {
        let mut dimensions = [a, b, c];
        dimensions.sort_unstable();
        let [x, y, z] = dimensions;

        (2 * x) + (2 * y) + (x * y * z)
    }

    fn compute(&self, l: fn((u32, u32, u32)) -> u32) -> u32 {
        self.iter().map(|sides| l(*sides)).sum()
    }
}

impl Solution for Day {
    type Output = u32;

    /// Calculate the total square feet of wrapping paper needed.
    fn part1(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        Ok(self.compute(Self::dimensions))
    }

    /// Calculate the total feet of ribbon needed.
    fn part2(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        Ok(self.compute(Self::ribbon))
    }
}

/// Run Part 1 and Part 2.
fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("../inputs/2015-12-02.txt"))?;
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
    #[case("2x3x4", 58)]
    #[case("1x1x10", 43)]
    fn test_cases_part1(#[case] input: &str, #[case] expected: u32) -> miette::Result<()> {
        assert_eq!(Day::dimensions(Day::parse_dimensions(input).unwrap().1), expected);

        Ok(())
    }

    #[rstest]
    #[case("2x3x4", 34)]
    #[case("1x1x10", 14)]
    fn test_cases_part2(#[case] input: &str, #[case] expected: u32) -> miette::Result<()> {
        assert_eq!(Day::ribbon(Day::parse_dimensions(input).unwrap().1), expected);

        Ok(())
    }
}