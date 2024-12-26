//! Day 6: Probably a Fire Hazard

use std::str::FromStr;

use aoc_ornaments::{spatial::Grid, Part, Solution};

#[derive(Debug)]
struct Day(Grid<bool>);

impl std::ops::Deref for Day {
    type Target = Grid<bool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let mut grid = Grid::initialize(1000, 1000, false);

        input.lines().for_each(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            match parts.as_slice() {
                ["turn", "on", start, "through", end] => {
                    let start = Grid::<bool>::position_from_str(start).unwrap();
                    let end = Grid::<bool>::position_from_str(end).unwrap();

                    grid.walk_region(start, end, |grid, pos| {
                        grid.set_at_unbounded(pos, true);
                    });
                }
                ["turn", "off", start, "through", end] => {
                    let start = Grid::<bool>::position_from_str(start).unwrap();
                    let end = Grid::<bool>::position_from_str(end).unwrap();

                    grid.walk_region(start, end, |grid, pos| {
                        grid.set_at_unbounded(pos, false);
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

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        // Ok(self.iter().filter(|&&b| b).count())

        let mut count = 0;

        self.walk(|pos| {
            if self.get_at_unbounded(pos) {
                count += 1;
            }
        });

        Ok(count)
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-06.txt"))?;
    let part1 = day.solve(Part::One)?;
    // let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    // println!("Part 2: {}", part2);

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
        assert_eq!(Day::from_str(input).unwrap().solve(Part::One).unwrap(), expected.to_string());
    }
}