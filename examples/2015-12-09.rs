//! Day 9: All in a Single Night

use std::{collections::HashSet, str::FromStr};

use aoc_ornaments::{linear::Distances, Part, Solution};

#[derive(Debug, derive_more::Deref)]
struct Day(Distances<u32>);

impl FromStr for Day {
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
            })))        
    }
}

impl Day {
    fn compute(&self, strategy: PathStrategy) -> Option<u32> {
        let cities = self.get_unique();
        let mut overall_result = None;
    
        for start in cities.iter() {
            let mut remaining: HashSet<_> = cities
                .iter()
                .filter(|&city| city != start)
                .copied()
                .collect();
            
            // let mut path_result = None;

            todo!();
            // (strategy.finder)(self, start, &mut remaining, 0, &mut path_result);
            
            // if let Some(path_len) = path_result {
            //     overall_result = match overall_result {
            //         None => Some(path_len),
            //         Some(current) => Some((strategy.compare)(current, path_len))
            //     };
            // }
        }
        
        overall_result
    }
}

impl Solution for Day {
    type Output = u32;

    fn part1(&mut self) -> miette::Result<Self::Output> {
        // Ok(self.compute(PathStrategy {
        //     finder: Self::find_shortest_path,
        //     compare: u32::min,
        // }).expect("No path found"))

        let cities = self.get_unique();
        let mut overall_shortest = None;
    
        // Try each city as a starting point
        for start in cities.iter() {
            let mut remaining: HashSet<_> = cities
                .iter()
                .filter(|&city| city != start)
                .copied()
                .collect();
            
            let mut path_shortest = None;
            self.find_shortest_path(start, &mut remaining, 0, &mut path_shortest);
            
            // Update overall shortest if this path is shorter
            if let Some(path_len) = path_shortest {
                overall_shortest = match overall_shortest {
                    None => Some(path_len),
                    Some(current_shortest) => Some(current_shortest.min(path_len))
                };
            }
        }
        
        Ok(overall_shortest.unwrap())
    }

    fn part2(&mut self) -> miette::Result<Self::Output> {
        // Ok(self.compute(PathStrategy {
        //     finder: Self::find_longest_path,
        //     compare: u32::max,
        // }).expect("No path found"))

        let cities = self.get_unique();
        let mut overall_longest = None;
    
        // Try each city as a starting point
        for start in cities.iter() {
            let mut remaining: HashSet<_> = cities
                .iter()
                .filter(|&city| city != start)
                .copied()
                .collect();
            
            let mut path_longest = None;
            self.find_longest_path(start, &mut remaining, 0, &mut path_longest);
            
            // Update overall longest if this path is longer
            if let Some(path_len) = path_longest {
                overall_longest = match overall_longest {
                    None => Some(path_len),
                    Some(current_shortest) => Some(current_shortest.max(path_len))
                };
            }
        }
        
        Ok(overall_longest.unwrap())
    }
}

struct PathStrategy {
    // finder: fn(&Day, &City, &mut HashSet<City>, u32, &mut Option<u32>),
    compare: fn(u32, u32) -> u32,
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-09.txt"))?;
    let part1 = day.solve(Part::One)?;
    let part2 = day.solve(Part::Two)?;

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
