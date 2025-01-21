//! # Day 3: Squares With Three Sides
//! 

use std::str::FromStr;

use aoc_ornaments::{nom::parse_dimensions, Part, Solution};
use itertools::Itertools;
use nom::{bytes::complete::take_until, character::complete::digit1};

#[derive(Debug, derive_more::Deref)]
struct Day(Vec<(u32, u32, u32)>);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(input.lines()
            .map(|line| {
                // let (line, _) = take_until(digit1)(line).unwrap();
                // let (_, (l, w, h)) = tuple(preceded(space0, parse_dimensions))(line).unwrap();
                // let (_, (l, w, h)) = parse_dimensions(line.trim()).unwrap();

                // let digits: Option<(usize, usize, usize)> = 
                line.split_whitespace().map(
                    |s| s.trim().parse::<u32>().unwrap()
                ).collect_tuple().unwrap()

                // (l, w, h)
                // digits.unwrap()
            }).collect()))
    }
}

impl Day {
    fn is_triangle((a, b, c): (u32, u32, u32)) -> bool {
        let mut sides = [a, b, c];
        sides.sort_unstable();

        sides[0] + sides[1] > sides[2]
    }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> miette::Result<Self::Output> {
        Ok(self.iter().filter(|sides| Day::is_triangle(**sides)).count())
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("../inputs/2016-12-03.txt"))?;
    let part1 = day.solve(Part::One)?;
    // let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    // println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "5 10 25";
        let mut day = Day::from_str(input).unwrap();
        assert_eq!(day.solve(Part::One).unwrap(), "0");
    }
}