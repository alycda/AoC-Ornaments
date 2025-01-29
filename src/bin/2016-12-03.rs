//! # Day 3: Squares With Three Sides
//! 

use std::str::FromStr;

use aoc_ornaments::{Part, Solution};
use itertools::Itertools;

#[derive(Debug, derive_more::Deref)]
struct Day(Vec<(u32, u32, u32)>);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(input.lines()
            .map(|line| {
                line.split_whitespace().map(
                    |s| s.trim().parse::<u32>().unwrap()
                ).collect_tuple().unwrap()
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

    fn part2(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        let mut count = 0;

        for sides in self.iter().tuples::<(_, _, _)>() {
            for i in 0..3 {
                let ((a1, a2, a3), (b1, b2, b3), (c1, c2, c3)) = sides;
                let triangle = match i {
                    0 => (*a1, *b1, *c1),
                    1 => (*a2, *b2, *c2),
                    2 => (*a3, *b3, *c3),
                    _ => unreachable!(),
                };

                if Day::is_triangle(triangle) {
                    count += 1;
                }
            }
        }

        Ok(count)
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("../inputs/2016-12-03.txt"))?;
    let part1 = day.solve(Part::One)?;
    let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

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

    #[test]
    fn part2() {
        let input = "101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603";
        let mut day = Day::from_str(input).unwrap();
        assert_eq!(day.solve(Part::Two).unwrap(), "6");
    }
}