//! Day 3: Perfectly Spherical Houses in a Vacuum

use std::{collections::HashSet, str::FromStr};

use aoc_ornaments::{spatial::{Direction, Position, Visited}, Part, Solution};

#[derive(Debug)]
pub struct Part1(Visited<usize>);

impl std::ops::Deref for Part1 {
    type Target = Visited<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Deref for Day<Part1> {
    type Target = Visited<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct Part2 {
    santa: Visited<usize>,
    robot: Visited<usize>,
}

#[derive(Debug)]
struct Day<P>(P);
// struct Day(Hashset<Position>, PhantomData<P>);

impl Solution for Day<Part1> {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        Ok(self.keys().count())
    }

    fn part2(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        unimplemented!("Part 2")
    }
}

impl Solution for Day<Part2> {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        unimplemented!("Part 1")
    }

    fn part2(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        let mut common = HashSet::new();

        dbg!(&self.0.santa);
        dbg!(&self.0.robot);

        // panic!("halt");

        self.0.santa.keys().for_each(|k| {
            if self.0.robot.contains_key(k) {
                common.insert(k);
            }
        });

        // Ok(uncommon.len())

        Ok(self.0.santa.keys().len() + self.0.robot.keys().len() - common.len() * 2)
    }
}

impl FromStr for Day<Part1> {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let mut visited = Visited::default(); //Visited::with_start(Position::ZERO);
        let mut position = Position::ZERO;

        visited.insert(position, 1);

        for c in input.chars() {
            position += Direction::parse(c)?.to_offset();
            *visited.entry(position).or_insert(0) += 1;
        }

        Ok(Self(Part1(visited)))
    }
}

impl FromStr for Day<Part2> {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let mut santa = Visited::default(); //Visited::with_start(Position::ZERO);
        let mut robot = Visited::default(); //Visited::with_start(Position::ZERO);
        let mut santa_position = Position::ZERO;
        let mut robot_position = Position::ZERO;

        santa.insert(santa_position, 1);
        robot.insert(robot_position, 1);
        
        input.chars().enumerate().for_each(|(i, c)| {
            if i % 2 == 0 {
                santa_position += Direction::parse(c).unwrap().to_offset();
                *santa.entry(santa_position).or_insert(0) += 1;
            } else {
                robot_position += Direction::parse(c).unwrap().to_offset();
                *robot.entry(robot_position).or_insert(0) += 1;
            }
        });

        Ok(Self(Part2{ santa, robot }))
    }
}

fn main() -> miette::Result<()> {
    let input = include_str!("./inputs/2015-12-03.txt");
    let mut day_part1 = Day::<Part1>::from_str(input)?;
    let mut day_part2 = Day::<Part2>::from_str(input)?;
    let part1 = day_part1.solve(Part::One)?;
    let part2 = day_part2.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2); // > 2440

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
        assert_eq!(Day::<Part2>::from_str(input).unwrap().solve(Part::Two).unwrap(), (expected - 1).to_string());
    }
}