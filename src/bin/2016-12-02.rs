//! # Day 2: Bathroom Security
//! 

use std::{collections::HashMap, str::FromStr};

use aoc_ornaments::{keypad::{ButtonPad, NumberPad3x3Telephone}, spatial::Direction, Part, Solution};
use nom::{character::complete::one_of, multi::many0, IResult};

#[derive(Debug, derive_more::Deref)]
struct Day(Vec<Vec<Direction>>);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(input.lines()
            .map(|line| {
                Day::to_directions(line).unwrap().1
            })
            .collect()))
    }
}

impl Day {
    fn to_directions(input: &str) -> IResult<&str, Vec<Direction>> {
        let (_, chars) = many0(one_of("ULDR"))(input)?;

        Ok(("", chars.iter().map(| c | {
                Direction::parse(c).unwrap()
            }).collect::<Vec<_>>()))
    }

    fn to_digits(&self) -> Vec<usize> {
        todo!()
    }
}

impl Solution for Day {
    type Output = String;

    fn part1(&mut self) -> miette::Result<Self::Output> {
        // dbg!(&self);

        let pad = ButtonPad::<NumberPad3x3Telephone>::default();

        let mut start = *pad.get(&'5').expect("Invalid start position (should be (1,1)");

        Ok(self.iter().map(|directions| {
            for direction in directions {
                let next = start + direction.to_offset();
                // if pad.get(next).is_some() {
                //     start = &next;
                // }

                if pad.values().any(|&pos| pos == next) {
                    start = next;
                }
            }

            pad.iter().find(|(_, pos)| **pos == start).unwrap().0

        }).collect::<String>())
    }

    fn part2(&mut self) -> miette::Result<Self::Output> {
        todo!();
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("../inputs/2016-12-02.txt"))?;
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
    fn part_1() {
        let input = "ULL\nRRDDD\nLURDL\nUUUUD";
        let mut day = Day::from_str(input).unwrap();

        assert_eq!(day.solve(Part::One).unwrap(), "1985");
    }
}