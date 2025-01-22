//! Day 9: All in a Single Night

use std::{collections::HashSet, marker::PhantomData, str::FromStr};

use aoc_ornaments::{linear::Distances, Part, Solution};

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

            // Update overall minimum if this path is shorter
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

#[derive(Debug)]
struct Day<P>(Distances<u32>, PhantomData<P>);

trait Strategy {
    const COMPARE: fn(u32, u32) -> u32;
}

impl<P> std::ops::Deref for Day<P> {
    type Target = Distances<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Travelling Salesman Problem
struct ShortestPath;

/// Travelling Salesman Problem
struct LongestPath;

impl Strategy for ShortestPath {
    const COMPARE: fn(u32, u32) -> u32 = u32::min;
}

impl Strategy for LongestPath {
    const COMPARE: fn(u32, u32) -> u32 = u32::max;
}

impl<P> FromStr for Day<P> {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(
            input.lines().fold(Distances::new(), |mut map, line| {
                let parts: Vec<&str> = line.split(' ').collect();

                match parts.as_slice() {
                    [a, "to", b, "=", d] => {
                        let distance = d.parse().unwrap();
                        map.insert_ordered(a.to_string(), b.to_string(), distance);
                    }
                    _ => panic!("Invalid input"),
                }

                map
            }),
            PhantomData,
        ))
    }
}

impl<P: Strategy> Solution for Day<P> {
    type Output = u32;

    fn solve(&mut self, _part: Part) -> aoc_ornaments::SolutionResult<String> {
        Ok(TSP::<u32>::best(self, P::COMPARE).unwrap().to_string())
    }
}

fn main() -> miette::Result<()> {
    let input = include_str!("./inputs/2015-12-09.txt");
    let part1 = Day::<ShortestPath>::from_str(input)?.solve(Part::One)?;
    let part2 = Day::<LongestPath>::from_str(input)?.solve(Part::Two)?;

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
Dublin to London = 464
London to Belfast = 518
Belfast to London = 518
Dublin to Belfast = 141
Belfast to Dublin = 141";

        let mut day = Day::<ShortestPath>::from_str(input).unwrap();
        let result = day.solve(Part::One).unwrap();

        assert_eq!(result, "605");
    }

    #[test]
    fn test_part2() {
        let input = "London to Dublin = 464
Dublin to London = 464
London to Belfast = 518
Belfast to London = 518
Dublin to Belfast = 141
Belfast to Dublin = 141";

        let mut day = Day::<LongestPath>::from_str(input).unwrap();
        let result = day.solve(Part::Two).unwrap();

        assert_eq!(result, "982");
    }
}
