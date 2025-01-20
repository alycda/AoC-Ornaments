//! # No Time for a Taxicab
//! 
//! Manhattan Distance
//! Cycle Detection
//! 

use std::str::FromStr;

use aoc_ornaments::{spatial::{manhattan_distance, Direction, Position}, Part, Solution};
use nom::{character::complete::{digit1, one_of}, sequence::tuple, IResult};

#[derive(Debug, derive_more::Deref)]
struct Day(Vec<Position>);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let mut direction = Direction::Up;

        Ok(Day(input.split(", ")
            .map(|turn| {
                let (_, (turn, distance)) = Self::parse_direction(turn)
                    .expect("parse error");

                match turn {
                    'L' => direction = direction.turn_left(),
                    'R' => direction = direction.turn_right(),
                    _ => unreachable!()
                }

                (direction, distance)
            })
            .map(|(direction, distance)| {
                match direction {
                    Direction::Up | Direction::Down => {
                        Position::new(0, distance) * direction.to_offset()
                    },
                    Direction::Left | Direction::Right => {
                        Position::new(distance, 0) * direction.to_offset()
                    },
                }
            })
            .collect::<Vec<_>>()))
    }
}

impl Day {
    fn parse_direction(input: &str) -> IResult<&str, (char, i32)> {
        tuple((one_of("LR"), digit1))(input)
            .map(|(input, (turn, distance))| (input, (turn, distance.parse().unwrap())))
    }
}

impl Solution for Day {
    type Output = i32;

    fn part1(&mut self) -> miette::Result<Self::Output> {
        Ok(manhattan_distance(&Position::ZERO, &self.iter().sum()))
    }

    fn part2(&mut self) -> miette::Result<Self::Output> {
        let mut visited = std::collections::HashSet::new();
        let current = Position::ZERO;
        visited.insert(Position::ZERO);

        Ok(self.iter()
            .flat_map(|straight_line| {
                // Break down movement into unit steps
                let unit = match (straight_line.x, straight_line.y) {
                    (x, 0) => Position::new(x.signum(), 0),
                    (0, y) => Position::new(0, y.signum()),
                    _ => unreachable!(),
                };
                let steps = straight_line.x.abs().max(straight_line.y.abs());
                (0..steps).map(move |_| unit)
            })
            .scan(current, |pos, step| {
                *pos += step;
                Some(*pos)
            })
            .find(|pos| !visited.insert(*pos))
            .map(|pos| manhattan_distance(&Position::ZERO, &pos))
            // can't find the Easter Bunny HQ
            .expect("no position visited twice"))

    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("../inputs/2016-12-01.txt"))?;
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
    #[case("R2, L3", 5)]
    #[case("R2, R2, R2", 2)]
    #[case("R5, L5, R5, R3", 12)]
    fn part_1(#[case] input: &str, #[case] expected: i32) {
        let mut day = Day::from_str(input).unwrap();
        assert_eq!(day.solve(Part::One).unwrap(), expected.to_string());
    }

    #[test]
    fn part_2() {
        let mut day = Day::from_str("R8, R4, R4, R8").unwrap();
        assert_eq!(day.solve(Part::Two).unwrap(), "4");
    }
}