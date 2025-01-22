//! Day 9: All in a Single Night

use std::str::FromStr;

use aoc_ornaments::{
    graph::{Distances, TravelingSales},
    Part, Solution,
};

#[derive(Debug, derive_more::Deref)]
struct Day(Distances<u32>);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(input.lines().fold(
            Distances::new(),
            |mut map, line| {
                let parts: Vec<&str> = line.split(' ').collect();

                match parts.as_slice() {
                    [a, "to", b, "=", d] => {
                        let distance = d.parse().unwrap();
                        map.insert_ordered(a.to_string(), b.to_string(), distance);
                        // Also insert the reverse direction with the same distance
                        // map.insert_ordered(b.to_string(), a.to_string(), distance);
                    }
                    _ => panic!("Invalid input"),
                }

                map
            },
        )))
    }
}

impl Solution for Day {
    type Output = u32;

    fn solve(&mut self, part: Part) -> aoc_ornaments::SolutionResult<String> {
        let strategy = match part {
            Part::One => Self::Output::min,
            Part::Two => Self::Output::max,
            // _ => u32::eq,
            _ => unimplemented!(),
        };

        Ok(TravelingSales::<Self::Output>::best(self, strategy)
            .unwrap()
            .to_string())
    }
}

fn main() -> miette::Result<()> {
    let input = include_str!("./inputs/2015-12-09.txt");
    let part1 = Day::from_str(input)?.solve(Part::One)?;
    let part2 = Day::from_str(input)?.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

        let mut day = Day::from_str(input).unwrap();
        let result = day.solve(Part::One).unwrap();

        assert_eq!(result, "605");
    }

    #[test]
    fn test_part2() {
        let input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

        let mut day = Day::from_str(input).unwrap();
        let result = day.solve(Part::Two).unwrap();

        assert_eq!(result, "982");
    }
}
