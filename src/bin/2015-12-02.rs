//! Day 2: I Was Told There Would Be No Math

use std::str::FromStr;

use aoc_ornaments::{Part, Solution};

use nom::{
    character::complete::{char, digit1}, combinator::map_res, multi::separated_list1, sequence::tuple, IResult
};

// Parse a single number like "20"
fn number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn dimensions(input: &str) -> IResult<&str, (u32, u32, u32)> {
    let (input, nums) = separated_list1(char('x'), number)(input)?;
    match nums.as_slice() {
        [l, w, h] => Ok((input, (*l, *w, *h))),
        _ => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::LengthValue
        )))
    }
}

#[derive(Debug, derive_more::Deref)]
struct Day(Vec<(u32, u32, u32)>);

impl Day {
    fn dimensions(sides: (u32, u32, u32)) -> u32 {
        let (l, w, h) = sides;
        let a = l * w;
        let b = w * h;
        let c = h * l;

        (2 * a) + (2 * b) + (2 * c) + [a, b, c].iter().min().unwrap()
    }

    fn ribbon(sides: (u32, u32, u32)) -> u32 {
        let (x, y, z) = sort_dimensions(sides);

        (2 * x) + (2 * y) + (x * y * z)
    }

    fn compute(&self, l: fn((u32, u32, u32)) -> u32) -> u32 {
        self.iter().map(|sides| l(*sides)).sum()
    }
}

/// todo: sort_unstable():
/// 
/// ```rust
/// let mut dims = [a, b, c];
/// dims.sort_unstable();
/// (dims[0], dims[1], dims[2])
/// ```
fn sort_dimensions((a, b, c): (u32, u32, u32)) -> (u32, u32, u32) {
    if a >= b && a >= c {
        (b, c, a)
    } else if b >= a && b >= c {
        (a, c, b)
    } else {
        (a, b, c)
    }
}

impl Solution for Day {
    type Output = u32;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        Ok(self.compute(Self::dimensions))
    }

    fn part2(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        Ok(self.compute(Self::ribbon))
    }
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let parsed = input.lines().map(|line| {
            let (_, (l, w, h)) = dimensions(line).unwrap();
            (l, w, h)
        }).collect();

        Ok(Self(parsed))
    }
}

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
    fn test_cases_part1(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(Day::dimensions(dimensions(input).unwrap().1), expected);
    }

    #[rstest]
    #[case("2x3x4", 34)]
    #[case("1x1x10", 14)]
    fn test_cases_part2(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(Day::ribbon(dimensions(input).unwrap().1), expected);
    }
}