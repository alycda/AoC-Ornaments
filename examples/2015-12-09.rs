//! Day 9: All in a Single Night

use std::{collections::HashSet, marker::PhantomData, str::FromStr};

use aoc_ornaments::{linear::Distances, Part, Solution};

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
        Ok(Self(input.lines()
            .fold(Distances::new(), |mut map, line| {
                let parts: Vec<&str> = line.split(' ').collect();

                match parts.as_slice() {
                    [a, "to", b, "=", d] => {
                        let distance = d.parse().unwrap();
                        map.insert_ordered(a.to_string(), b.to_string(), distance);
                    },
                    _ => panic!("Invalid input"),
                }

                map
            }), PhantomData))        
    }
}

impl<P: Strategy> Day<P> {
    /// Travelling Salesman Problem (Brute Force)
    fn tsp(&self, current: &str, remaining: &mut HashSet<&str>, total: u32, extreme: &mut Option<u32>) {
        let strategy = P::COMPARE;

        // If no(thing)s remain, we've found a complete path
        if remaining.is_empty() {
            *extreme = match *extreme {
                None => Some(total),
                Some(s) => Some(strategy(s, total))
            };
            return;
        }

        // Try each remaining city as the next step
        let neighbors: Vec<_> = remaining.iter().copied().collect();
        for next in neighbors {
            // Get distance to this neighbor
            let key = if current < next {
                (current.to_string(), next.to_string())
            } else {
                (next.to_string(), current.to_string())
            };
            let distance = self.get(&key).unwrap();
            
            // Visit this neighbor
            remaining.remove(next);
            self.tsp(
                next,
                remaining,
                total + *distance,
                extreme
            );
            remaining.insert(next);
        }
    }
}

impl<P: Strategy> Solution for Day<P> {
    type Output = u32;

    fn solve(&mut self, _part: Part) -> miette::Result<String> {
        let cities = self.get_unique();
        let mut most_extreme = None;

        let strategy = P::COMPARE;

        // Try each city as a starting point
        for start in cities.iter() {
            let mut remaining: HashSet<_> = cities
                .iter()
                .filter(|&city| city != start)
                .copied()
                .collect();

            let mut path_extreme = None;

            self.tsp(start, &mut remaining, 0, &mut path_extreme);

            if let Some(path_len) = path_extreme {
                most_extreme = match most_extreme {
                    None => Some(path_len),
                    Some(current_extreme) => Some(strategy(current_extreme, path_len)),
                };
            }

        }

        Ok(most_extreme.ok_or_else(|| miette::miette!("Traveling Salesman Problem"))?.to_string())
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
London to Belfast = 518
Dublin to Belfast = 141";

        let mut day = Day::<ShortestPath>::from_str(input).unwrap();
        let result = day.solve(Part::One).unwrap();

        assert_eq!(result, "605");
    }

    #[test]
    fn test_part2() {
        let input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

        let mut day = Day::<LongestPath>::from_str(input).unwrap();
        let result = day.solve(Part::Two).unwrap();

        assert_eq!(result, "982");
    }
}
