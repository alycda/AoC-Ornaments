//! Day 9: All in a Single Night

use std::{collections::HashSet, str::FromStr};

use aoc_ornaments::{graph::Distances, Part, Solution};

#[derive(Debug)]
struct TSP<T>(Option<T>, fn(T, T) -> T);

impl<T: std::ops::Add<Output = T> + Clone + Copy + Ord + Default> TSP<T> {
    fn new(compare: fn(T, T) -> T) -> Self {
        Self(None, compare)
    }

    fn best(map: &Distances<T>, strategy: fn(T, T) -> T) -> Option<T> {
        let set = map.get_unique();
        let mut best_path = None;

        for start in set.iter() {
            let mut remaining = set.clone();
            remaining.remove(start); // Remove starting city from remaining set

            let mut tsp = Self::new(strategy);
            tsp.call(map, start, &mut remaining, T::default());

            // Update overall best if this path is STRATEGY
            if let Some(path_length) = *tsp {
                best_path = match best_path {
                    None => Some(path_length),
                    Some(current_best) => Some(strategy(current_best, path_length)),
                };
            }
        }

        best_path
    }

    fn call(
        &mut self,
        locations: &Distances<T>,
        current: &str,
        remaining: &mut HashSet<&str>,
        running_total: T,
    ) {
        if remaining.is_empty() {
            self.0 = match self.0 {
                None => Some(running_total),
                Some(current_min) => Some((self.1)(current_min, running_total)),
            };
            return;
        }

        let neighbors: Vec<_> = remaining.iter().copied().collect();
        for next in neighbors {
            let key = if current < next {
                (current.to_string(), next.to_string())
            } else {
                (next.to_string(), current.to_string())
            };

            if let Some(distance) = locations.get(&key) {
                // dbg!(running_total, distance);

                remaining.remove(next);
                self.call(locations, next, remaining, running_total + *distance);
                remaining.insert(next);
            }
        }
    }
}

impl<T> std::ops::Deref for TSP<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

        Ok(TSP::<Self::Output>::best(self, strategy)
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
