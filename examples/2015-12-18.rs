//! Day 18: Like a GIF For Your Yard

use std::str::FromStr;

use aoc_ornaments::{spatial::Grid, Part, Solution};

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
        Ok(Self(Grid::from_str(input)?))
    }
}

impl Day {
    fn step(&mut self, count: usize) {
        for _ in 0..count {
            let mut next = Grid(self.clone());

        //     for y in 0..self.get_height() {
        //         for x in 0..self.get_width() {
        //             let neighbors = self.get_neighbors(x, y);
        //             let on = self[y][x];

        //             next[y][x] = match (on, neighbors) {
        //                 (true, 2..=3) => true,
        //                 (false, 3) => true,
        //                 _ => false,
        //             };
        //         }
        //     }

            self.walk(|pos| {
                let light = self.get_at_unbounded(pos);
                let neighbors = self.get_all_neighbors(pos);

                let neighbors_on = neighbors.iter().filter(|(_pos, state)| *state).count();

                // dbg!(neighbors_on);

                let next_light = match (light, neighbors_on) {
                    (true, 2..=3) => true,
                    (false, 3) => true,
                    _ => false,
                };

                next.set_at(pos, next_light)
            });

            self.0 = next;
        }
    }

    fn toggle() {
        todo!()
    }

    fn sum(&self) -> usize {
        self.iter().map(|row| row.iter().filter(|&&b| b).count()).sum()
    }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        self.step(100);

        Ok(self.sum())
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-18.txt"))?;
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
    fn test_steps(#[case] input: &str, #[case] steps: usize) {
        // Day::step(4);

        todo!();
    }

    #[test]
    fn test_part1() {
        let input = "1B5...
    234...
    ......
    ..123.
    ..8A4.
    ..765.";

        let mut day = Day::from_str(input).unwrap();
        let result = day.solve(Part::One).unwrap();

        assert_eq!(result, "4");
    }
}