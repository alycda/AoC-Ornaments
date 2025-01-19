//! Day 6: Probably a Fire Hazard

use std::{ops::Not, str::FromStr};

use aoc_ornaments::{spatial::Grid, Part, Solution};

#[derive(Debug, derive_more::Deref, Clone, Copy, PartialEq)]
pub struct Part1(bool);

impl From<bool> for Part1 {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl Not for Part1 {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

#[derive(Debug, derive_more::Deref, Clone, Copy, PartialEq)]
pub struct Part2(u32);

#[derive(Debug, derive_more::Deref)]
struct Day<P>(Grid<P>);

impl FromStr for Day<Part1> {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let mut grid = Grid::initialize(1000, 1000, Part1(false));

        input.lines().for_each(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            match parts.as_slice() {
                ["turn", "on", start, "through", end] => {
                    let start = Grid::<bool>::position_from_str(start).unwrap();
                    let end = Grid::<bool>::position_from_str(end).unwrap();

                    grid.walk_region(start, end, |grid, pos| {
                        grid.set_at_unbounded(pos, true.into());
                    });
                }
                ["turn", "off", start, "through", end] => {
                    let start = Grid::<bool>::position_from_str(start).unwrap();
                    let end = Grid::<bool>::position_from_str(end).unwrap();

                    grid.walk_region(start, end, |grid, pos| {
                        grid.set_at_unbounded(pos, false.into());
                    });
                }
                ["toggle", start, "through", end] => {
                    let start = Grid::<bool>::position_from_str(start).unwrap();
                    let end = Grid::<bool>::position_from_str(end).unwrap();

                    grid.walk_region(start, end, |grid, pos| {
                        // grid.get_at_unbounded(pos) = !grid.get_at_unbounded(pos);
                        grid.set_at_unbounded(pos, !grid.get_at_unbounded(pos));
                    });
                }
                _ => {}
            }
        });

        Ok(Self(grid))
    }
}

impl FromStr for Day<Part2> {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let mut grid = Grid::initialize(1000, 1000, Part2(0));

        input.lines().for_each(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            match parts.as_slice() {
                ["turn", "on", start, "through", end] => {
                    let start = Grid::<bool>::position_from_str(start).unwrap();
                    let end = Grid::<bool>::position_from_str(end).unwrap();

                    grid.walk_region(start, end, |grid, pos| {
                        let current = grid.get_at_unbounded(pos).0;
                        grid.set_at_unbounded(pos, Part2(current + 1));
                    });
                }
                ["turn", "off", start, "through", end] => {
                    let start = Grid::<bool>::position_from_str(start).unwrap();
                    let end = Grid::<bool>::position_from_str(end).unwrap();

                    grid.walk_region(start, end, |grid, pos| {
                        let current = grid.get_at_unbounded(pos).0;
                        grid.set_at_unbounded(pos, Part2(current.saturating_sub(1)));
                    });
                }
                ["toggle", start, "through", end] => {
                    let start = Grid::<bool>::position_from_str(start).unwrap();
                    let end = Grid::<bool>::position_from_str(end).unwrap();

                    grid.walk_region(start, end, |grid, pos| {
                        let current = grid.get_at_unbounded(pos).0;
                        grid.set_at_unbounded(pos, Part2(current + 2));
                    });
                }
                _ => {}
            }
        });

        Ok(Self(grid))
    }
}

impl Solution for Day<Part1> {
    type Output = usize;

    /// After following the instructions, how many lights are lit?
    ///
    /// Option 1: Explicitly deref using *
    /// `Ok(self.iter().filter(|&&b| *b).count())`

    /// Option 2: Map to bool first
    /// `Ok(self.iter().map(|&b| *b).filter(|&b| b).count())`

    /// Option 3: Use as_ref() to get a reference to the inner bool
    /// `Ok(self.iter().filter(|&b| *b.as_ref()).count())`
    fn part1(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        let mut count = 0;

        self.walk(|pos| {
            if *self.get_at_unbounded(pos) {
                count += 1;
            }
        });

        Ok(count)
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

    /// What is the total brightness of all lights combined after following Santa's instructions?
    fn part2(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        let mut total = 0;

        self.walk(|pos| {
            total += self.get_at_unbounded(pos).0 as usize;
        });

        Ok(total)
    }
}

/// Run Part 1 and Part 2.
fn main() -> miette::Result<()> {
    let input = include_str!("./inputs/2015-12-06.txt");
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
    #[case("turn on 0,0 through 999,999", 1000 * 1000)]
    #[case("toggle 0,0 through 999,0", 1000)]
    #[case("toggle 0,0 through 999,0
toggle 0,0 through 999,0", 0)]
    #[case("turn off 499,499 through 500,500", 0)]
    fn test_cases_part1(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(Day::<Part1>::from_str(input).unwrap().solve(Part::One).unwrap(), expected.to_string());
    }

    #[rstest]
    #[case("turn on 0,0 through 0,0", 1)]
    #[case("toggle 0,0 through 999,999", 2000000)]
    #[case("turn on 0,0 through 0,0
toggle 0,0 through 999,999", 2000001)]
    fn test_cases_part2(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(Day::<Part2>::from_str(input).unwrap().solve(Part::Two).unwrap(), expected.to_string());
    }
}