//! Day 14: Reindeer Olympics

use std::{collections::HashMap, str::FromStr};

use aoc_ornaments::{Part, ArgSolution};

#[derive(Debug)]
struct ReindeerStats {
    /// km/s
    speed: u64,
    /// seconds
    time: u64,
    /// seconds
    rest: u64,
}

impl ReindeerStats {
    pub fn new(speed: &str, time: &str, rest: &str) -> Self {
        Self {
            speed: speed.parse().unwrap(),
            time: time.parse().unwrap(),
            rest: rest.parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Day(HashMap<String, ReindeerStats>);

impl std::ops::Deref for Day {
    type Target = HashMap<String, ReindeerStats>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();

        input.lines().for_each(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            match parts.as_slice() {
                [name, "can", "fly", speed, "km/s", "for", time, "seconds,", "but", "then", "must", "rest", "for", rest, "seconds."] => {
                    map.insert(name.to_string(), ReindeerStats::new(speed, time, rest));
                },
                _ => panic!("Invalid input"),
            }
        });
        
        Ok(Self(map))
    }
}

impl Day {
    fn seconds(count: u64, deer: &ReindeerStats) -> u64 {
        let cycle = deer.time + deer.rest;
        let cycles = count / cycle;
        let remainder = count % cycle;

        let distance = cycles * deer.time * deer.speed;
        let remainder_distance = std::cmp::min(remainder, deer.time) * deer.speed;

        distance + remainder_distance
    }

    fn points(count: u64, deer: &[&ReindeerStats]) -> Vec<u64> {
        let mut points = vec![0; deer.len()];
        
        for i in 1..=count {            
            // First pass: find max distance
            let distances: Vec<u64> = deer.iter()
                .map(|d| Day::seconds(i, d))
                .collect();
                
            let max_distance = *distances.iter().max().unwrap();
            
            // Second pass: award points to all deer tied for the lead
            for (idx, &distance) in distances.iter().enumerate() {
                if distance == max_distance {
                    points[idx] += 1;
                }
            }
        }
        
        points
    }
}

impl ArgSolution<u64> for Day {
    type Output = u64;

    fn part1(&mut self, count: Self::Output) -> aoc_ornaments::SolutionResult<Self::Output> {
        Ok(self.iter()
            .map(|(_name, stats)| {
                let distance = Day::seconds(count, stats);
                distance
            }).max().unwrap())
    }

    fn part2(&mut self, count: Self::Output) -> aoc_ornaments::SolutionResult<Self::Output> {
        let stats: Vec<&ReindeerStats> = self.values().collect();
        let all_points = Day::points(count, &stats);
        
        Ok(*all_points.iter().max().unwrap())
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("../inputs/2015-12-14.txt"))?;
    let part1 = day.solve(Part::One, 2503)?;
    let part2 = day.solve(Part::Two, 2503)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(1, (14, 16))]
    #[case(10, (140, 160))]
    #[case(11, (140, 176))]
    #[case(12, (140, 176))]
    #[case(138, (154, 176))]
    #[case(174, (280, 192))]
    fn test_cases_part1(#[case] seconds: u64, #[case] expected: (u64, u64)) {
//         let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
// Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

        let a = Day::seconds(seconds, &ReindeerStats { speed: 14, time: 10, rest: 127 });
        let b = Day::seconds(seconds, &ReindeerStats { speed: 16, time: 11, rest: 162 });

        assert_eq!(expected, (a, b));
    }

    #[rstest]
    #[case(1000, (312, 689))]
    fn test_cases_part2(#[case] seconds: u64, #[case] expected: (u64, u64)) {
//         let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
// Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

        let all = dbg!(Day::points(seconds, &[
            &ReindeerStats { speed: 14, time: 10, rest: 127 },
            &ReindeerStats { speed: 16, time: 11, rest: 162 },
        ]));

        assert_eq!(expected, (all[0], all[1]));
    }
}