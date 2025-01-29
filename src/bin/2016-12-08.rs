//! # Day 8: Two-Factor Authentication
//!

use std::str::FromStr;

use aoc_ornaments::{
    spatial::{Grid, PhantomGrid, Position, Spatial},
    ArgSolution, Part,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, i32, space0},
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
enum Dimension {
    X(i32),
    Y(i32),
}

#[derive(Debug)]
enum Operation {
    Rect(Position),
    Shift(Dimension, i32),
}

#[derive(Debug, derive_more::Deref)]
struct Day(Vec<Operation>);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(
            input
                .lines()
                .map(|line| Self::parse_line(line).unwrap().1)
                .collect(),
        ))
    }
}

impl Day {
    fn parse_line(input: &str) -> IResult<&str, Operation> {
        alt((Self::parse_rect, Self::parse_shift))(input)
    }

    fn parse_rect(input: &str) -> IResult<&str, Operation> {
        let (_, (_, x, _, y)) =
            preceded(tag("rect "), tuple((space0, i32, char('x'), i32)))(input)?;

        Ok(("", Operation::Rect(Position::new(x, y))))
    }

    fn parse_shift(input: &str) -> IResult<&str, Operation> {
        let (_, (_, dimension, _, _, _, offset)) = preceded(
            tag("rotate "),
            tuple((
                space0,
                alt((
                    map(tuple((tag("row y="), i32)), |(_, y)| Dimension::Y(y)),
                    map(tuple((tag("column x="), i32)), |(_, x)| Dimension::X(x)),
                )),
                space0,
                tag("by"),
                space0,
                i32,
            )),
        )(input)?;

        Ok(("", Operation::Shift(dimension, offset)))
    }
}

impl ArgSolution<Position> for Day {
    type Output = usize;

    fn solve(&mut self, _part: Part, args: Position) -> aoc_ornaments::SolutionResult<String> {
        let mut grid = PhantomGrid::new(args.x as u32, args.y as u32);

        self.iter().for_each(|instruction| {
            // println!("{grid}");
            // println!("{grid:?}\n");

            match instruction {
                Operation::Rect(size) => {
                    let tmp_grid = Grid::initialize(size.x as usize, size.y as usize, '.');

                    grid.flood_fill(Position::ZERO, |p: Position| tmp_grid.in_bounds(p))
                        .iter()
                        .for_each(|on| {
                            grid.0.insert(*on);
                        });
                }
                Operation::Shift(dimension, offset) => match dimension {
                    Dimension::X(x) => {
                        let g = grid
                            .iter()
                            .cloned()
                            .map(|mut p| {
                                if p.x == *x {
                                    // Wrap around the height
                                    p.y = (p.y + offset).rem_euclid(args.y);
                                }
                                p
                            })
                            .collect::<std::collections::HashSet<_>>();

                        grid = PhantomGrid(g, (Position::ZERO, args));
                    }
                    Dimension::Y(y) => {
                        let g = grid
                            .iter()
                            .cloned()
                            .map(|mut p| {
                                if p.y == *y {
                                    // Wrap around the width
                                    p.x = (p.x + offset).rem_euclid(args.x);
                                }
                                p
                            })
                            .collect::<std::collections::HashSet<_>>();

                        grid = PhantomGrid(g, (Position::ZERO, args));
                    }
                },
            }
        });

        // Part 2
        println!("{grid}");

        Ok(grid.0.len().to_string())
    }
}

fn main() -> miette::Result<()> {
    let mut puzzle = Day::from_str(include_str!("../inputs/2016-12-08.txt"))?;
    let part1 = puzzle.solve(Part::One, Position::new(50, 6))?;

    println!("Part 1: {part1}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() -> miette::Result<()> {
        let mut day = Day::from_str(
            "rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1",
        )?;

        assert_eq!(day.solve(Part::One, Position::new(7, 3))?, "6");

        Ok(())
    }
}
