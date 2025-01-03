//! Day 18: Like a GIF For Your Yard

use std::str::FromStr;

use aoc_ornaments::{spatial::{Grid, Position}, Part, Solution};

struct Day(Grid<bool>);

impl std::ops::Deref for Day {
    type Target = Grid<bool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Day {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

    fn toggle() {
        todo!()
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
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        self.step(100);

        Ok(self.sum())
    }

    // fn part2(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        // println!("Grid dimensions: {}x{}", self.get_width(), self.get_height());

        // // Print neighbors for each corner
        // let corners = [
        //     Position::ZERO,  // Top-left
        //     Position::new(self.get_width() as i32 - 1, 0), // Top-right
        //     Position::new(0, self.get_height() as i32 - 1), // Bottom-left
        //     Position::new(self.get_width() as i32 - 1, self.get_height() as i32 - 1), // Bottom-right
        // ];
        
        // for (i, corner) in corners.iter().enumerate() {
        //     let neighbors = self.get_all_neighbors(*corner);
        //     dbg!(&neighbors);
        //     println!("Corner {} has {} neighbors: {:?}", i, neighbors.len(), 
        //         neighbors.iter().filter(|(_pos, state)| *state).count());
        // }


        // self.always_on();  // Set initial corners
        
        // for (i, corner) in corners.iter().enumerate() {
        //     let neighbors = self.get_all_neighbors(*corner);
        //     println!("Corner {} has {} neighbors: {:?}", i, neighbors.len(), 
        //         neighbors.iter().filter(|(_pos, state)| *state).count());
        // }
    
        // for _ in 0..100 {
        //     let mut next = Grid(self.clone());  // Start with a copy of current state
    
        //     self.walk(|pos| {
        //         let neighbors = self.get_all_neighbors(pos)
        //             .iter()
        //             .filter(|(_pos, state)| *state)
        //             .count();
    
        //         match (self.get_at_unbounded(pos), neighbors) {
        //             (false, 3) => next.set_at(pos, true),   // Dead -> Alive
        //             (true, n) if n != 2 && n != 3 => next.set_at(pos, false),  // Alive -> Dead
        //             _ => None  // Otherwise state stays the same
        //         }
        //     });
    
        //     self.0 = next;
        //     self.always_on();  // Force corners on after grid swap
        // }
    
        // Ok(self.sum())
    // }

    fn part2(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        self.always_on();
        dbg!(self.sum());

        let debug_pos = [Position::ZERO, Position::new(99, 0), Position::new(0, 99), Position::new(99, 99)];

        for _ in 0..100 {
            // self.always_on();
            let mut next = Grid(self.clone());

            self.walk(|pos| {

                if debug_pos.contains(&pos) {
                    // dbg!(pos);
                    // dbg!(self.get_at_unbounded(pos));

                    assert!(self.get_at_unbounded(pos));
                }


                let neighbors = self.get_all_neighbors(pos).iter().filter(|(_pos, state)| *state).count();

                let light = match (self.get_at_unbounded(pos), neighbors) {
                    (true, 2..=3) => true,
                    (false, 3) => true,
                    _ => false,
                };

                next.set_at(pos, light);
                // next.always_on();
            });

            self.0 = next;
            self.always_on();
            dbg!(self.sum());
        }

        Ok(self.sum())
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-18.txt"))?;
    let part1 = day.solve(Part::One)?;
    // state is dirty after part1, need to reset
    let mut day = Day::from_str(include_str!("./inputs/2015-12-18.txt"))?;
    let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2); // > 721

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
    fn test_steps_part1(#[case] input: &str, #[case] steps: usize) {
        // Day::step(4);

        todo!();
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
        // todo: 4 steps
        let result = day.solve(Part::One).unwrap();

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
    fn test_steps_part2(#[case] input: &str, #[case] steps: usize) {
        // Day::step(5);

        todo!();
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
        // todo: 5 steps
        let result = day.solve(Part::One).unwrap();

        assert_eq!(result, "17");
    }
}