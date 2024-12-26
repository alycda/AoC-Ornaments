use std::{marker::PhantomData, str::FromStr};

use aoc_ornaments::{spatial::{Direction, Position, UniquePositions}, Part, Solution};

#[derive(Debug)]
pub struct Day<P>(UniquePositions, PhantomData<P>);

#[derive(Debug)]
pub struct Part1;

#[derive(Debug)]
pub struct Part2;

impl<P> std::ops::Deref for Day<P> {
    type Target = UniquePositions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Day<Part1> {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let mut visited = UniquePositions::new();
        let mut position = Position::ZERO;

        visited.insert(position);

        for c in input.chars() {
            position += Direction::parse(c)?.to_offset();
            visited.insert(position);
        }

        Ok(Self(visited, PhantomData))
    }
}

impl FromStr for Day<Part2> {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let mut santa_visited = UniquePositions::new();
        let mut robot_visited = UniquePositions::new();
        let mut santa_position = Position::ZERO;
        let mut robot_position = Position::ZERO;

        santa_visited.insert(santa_position);
        robot_visited.insert(robot_position);

        input.chars().enumerate().for_each(|(i, c)| {
            if i % 2 == 0 {
                santa_position += Direction::parse(c).expect("valid char").to_offset();
                santa_visited.insert(santa_position);
            } else {
                robot_position += Direction::parse(c).expect("valid char").to_offset();
                robot_visited.insert(robot_position);
            }
        });

        // for c in input.chars() {
        //     position += Direction::parse(c)?.to_offset();
        //     visited.insert(position);
        // }

        Ok(Self(santa_visited.union(&robot_visited).copied().collect(), PhantomData))
    }
}

impl<P> Solution for Day<P> where Day<P>: FromStr {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        Ok(self.len())
    }

    fn part2(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        Ok(self.len())
    }
}

fn main() -> miette::Result<()> {
    let input = include_str!("./inputs/2015-12-03.txt");
    let mut day_part1 = Day::<Part1>::from_str(input)?;
    let mut day_part2 = Day::<Part2>::from_str(input)?;
    let part1 = day_part1.solve(Part::One)?;
    let part2 = day_part2.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(">", 2)]
    #[case("^>v<", 4)]
    #[case("^v^v^v^v^v", 2)]
    fn test_cases_part1(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(Day::<Part1>::from_str(input).unwrap().solve(Part::One).unwrap(), expected.to_string());
    }

    #[rstest]
    #[case("^v", 3)]
    #[case("^>v<", 3)]
    #[case("^v^v^v^v^v", 11)]
    fn test_cases_part2(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(Day::<Part2>::from_str(input).unwrap().solve(Part::Two).unwrap(), expected.to_string());
    }
}