use std::str::FromStr;

use aoc_ornaments::{spatial::{Direction, Position, UniquePositions}, Part, Solution};

#[derive(Debug)]
pub struct Day(UniquePositions);

impl std::ops::Deref for Day {
    type Target = UniquePositions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let mut visited = UniquePositions::new();
        let mut position = Position::ZERO;

        visited.insert(position);

        for c in input.chars() {
            position += Direction::parse(c)?.to_offset();
            visited.insert(position);
        }

        Ok(Self(visited))
    }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        Ok(self.len())
    }
}

fn main() -> miette::Result<()> {
    let input = include_str!("./inputs/2015-12-03.txt");
    let mut day_part1 = Day::from_str(input)?;
    // let mut day_part2 = Day::<Part2>::from_str(input)?;
    let part1 = day_part1.solve(Part::One)?;
    // let part2 = day_part2.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    // println!("Part 2: {}", part2); // > 2440
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
        assert_eq!(Day::from_str(input).unwrap().solve(Part::One).unwrap(), expected.to_string());
    }

    #[rstest]
    #[case("^v", 3)]
    #[case("^>v<", 3)]
    #[case("^v^v^v^v^v", 11)]
    fn test_cases_part2(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(Day::from_str(input).unwrap().solve(Part::Two).unwrap(), (expected - 1).to_string());
    }
}