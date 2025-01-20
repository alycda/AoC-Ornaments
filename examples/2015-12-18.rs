//! Day 18: Like a GIF For Your Yard

use std::str::FromStr;

use aoc_ornaments::{spatial::{Grid, Position}, Part, ArgSolution};

#[derive(Debug, derive_more::Deref, derive_more::DerefMut)]
struct Day(Grid<bool>);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(Grid::from_str(input)?))
    }
}

impl Day {
    fn step(&mut self, count: usize) {
        for _ in 0..count {
            let mut next = Grid(self.clone());

            self.walk(|pos| {
                let neighbors = self.get_all_neighbors(pos).iter().filter(|(_pos, state)| *state).count();

                let light = match (self.get_at_unbounded(pos), neighbors) {
                    (true, 2..=3) => true,
                    (false, 3) => true,
                    _ => false,
                };

                next.set_at(pos, light)
            });

            self.0 = next;
        }
    }

    fn always_on(&mut self) {
        let width = self.get_width() as i32;
        let height = self.get_height() as i32;

        self.set_at(Position::ZERO, true);
        self.set_at(Position::new(width - 1, 0), true);
        self.set_at(Position::new(0, height - 1), true);
        self.set_at(Position::new(width - 1, height - 1), true);
    }

    fn sum(&self) -> usize {
        self.iter().map(|row| row.iter().filter(|&&b| b).count()).sum()
    }

    /// used for debugging
    #[allow(dead_code)]
    fn print(&self) {
        for row in self.iter() {
            for &light in row {
                print!("{}", if light { '#' } else { '.' });
            }
            println!();
        }
    }
}

impl ToString for Day {
    /// used for debugging
    fn to_string(&self) -> String {
        self.iter().map(|row| row.iter().map(|&b| if b { '#' } else { '.' }).collect::<String>()).collect::<Vec<String>>().join("\n")
    }
}

impl ArgSolution<usize> for Day {
    type Output = usize;

    fn part1(&mut self, count: usize) -> aoc_ornaments::SolutionResult<Self::Output> {
        self.step(count);

        Ok(self.sum())
    }

    fn part2(&mut self, count: usize) -> aoc_ornaments::SolutionResult<Self::Output> {
        self.always_on();

        for _ in 0..count {
            let mut next = Grid(self.clone());

            self.walk(|pos| {
                let neighbors = self.get_all_neighbors(pos).iter().filter(|(_pos, state)| *state).count();

                let light = match (self.get_at_unbounded(pos), neighbors) {
                    (true, 2..=3) => true,
                    (false, 3) => true,
                    _ => false,
                };

                next.set_at(pos, light);
            });

            self.0 = next;
            self.always_on();
        }

        Ok(self.sum())
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-18.txt"))?;
    let part1 = day.solve(Part::One, 100)?;
    // state is dirty after part1, need to reset
    let mut day = Day::from_str(include_str!("./inputs/2015-12-18.txt"))?;
    let part2 = day.solve(Part::Two, 100)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(".#.#.#
...##.
#....#
..#...
#.#..#
####..", 0)]
    #[case("..##..
..##.#
...##.
......
#.....
#.##..", 1)]
    #[case("..###.
......
..###.
......
.#....
.#....", 2)]
    #[case("...#..
......
...#..
..##..
......
......", 3)]
    #[case("......
......
..##..
..##..
......
......", 4)]
    fn test_steps_part1(#[case] expected: &str, #[case] steps: usize) {
        let mut day = Day::from_str(".#.#.#
...##.
#....#
..#...
#.#..#
####..").unwrap();
        day.step(steps);
        // day.print();

        assert_eq!(expected, day.to_string());
    }

    #[test]
    fn test_part1() {
        let input = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";

        let mut day = Day::from_str(input).unwrap();
        let result = day.solve(Part::One, 4).unwrap();

        assert_eq!(result, "4");
    }

    #[rstest]
    #[case("##.#.#
...##.
#....#
..#...
#.#..#
####.#", 0)]
    #[case("#.##.#
####.#
...##.
......
#...#.
#.####", 1)]
    #[case("#..#.#
#....#
.#.##.
...##.
.#..##
##.###", 2)]
    #[case("#...##
####.#
..##.#
......
##....
####.#", 3)]
    #[case("#.####
#....#
...#..
.##...
#.....
#.#..#", 4)]
#[case("##.###
.##..#
.##...
.##...
#.#...
##...#", 5)]
    fn test_steps_part2(#[case] expected: &str, #[case] steps: usize) {
        let mut day = Day::from_str("##.#.#
...##.
#....#
..#...
#.#..#
####.#").unwrap();
        day.solve(Part::Two, steps).unwrap();

        assert_eq!(expected, day.to_string());
    }

    #[test]
    fn test_part2() {
        let input = "##.#.#
...##.
#....#
..#...
#.#..#
####.#";

        let mut day = Day::from_str(input).unwrap();
        let result = day.solve(Part::Two, 5).unwrap();

        assert_eq!(result, "17");
    }
}