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

/// Shortest TSP
struct Shortest;

// impl Default for Shortest {
//     fn default() -> Self {
//         std::cmp::min
//     }
// }

/// Longest TSP
struct Longest;

// impl Default for Shortest {
//     fn default() -> Self {
//         std::cmp::max
//     }
// }

impl Strategy for Shortest {
    const COMPARE: fn(u32, u32) -> u32 = u32::min;
}

impl Strategy for Longest {
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

    fn solve(&mut self, part: Part) -> miette::Result<String> {
        let cities = self.get_unique();
        let mut most_extreme = None;

        // let strategy = match part {
        //     Part::One => Part1,
        //     Part::Two => Part2,
        //     _ => unreachable!(),
        // };

        let strategy = P::COMPARE;

        // Try each city as a starting point
        for start in cities.iter() {
            let mut remaining: HashSet<_> = cities
                .iter()
                .filter(|&city| city != start)
                .copied()
                .collect();

            let mut path_extreme = None;

            // match strategy {
            //     Shortest(_) => {
            //         self.find_shortest_path(start, &mut remaining, 0, &mut path_extreme);
            //     },
            //     Longest(_) => {
            //         self.find_longest_path(start, &mut remaining, 0, &mut path_extreme);
            //     },
            // }

            // dbg!(strategy.0);

            // let tsp = |current: &str, remaining: &mut HashSet<&str>, total: u32, extreme: &mut Option<u32>| {
            //     // If no(thing)s remain, we've found a complete path
            //     if remaining.is_empty() {
            //         *extreme = match *extreme {
            //             None => Some(total),
            //             Some(s) => Some(strategy(s, total))
            //         };
            //         return;
            //     }


            //     // Try each remaining city as the next step
            //     let neighbors: Vec<_> = remaining.iter().copied().collect();
            //     for next in neighbors {
            //         // Get distance to this neighbor
            //         let key = if current < next {
            //             (current.to_string(), next.to_string())
            //         } else {
            //             (next.to_string(), current.to_string())
            //         };
            //         let distance = self.get(&key).unwrap();
                    
            //         // Visit this neighbor
            //         remaining.remove(next);
            //         tsp(
            //             next,
            //             remaining,
            //             total + *distance,
            //             extreme
            //         );
            //         remaining.insert(next);
            //     }

            // };

            self.tsp(start, &mut remaining, 0, &mut path_extreme);

            if let Some(path_len) = path_extreme {
                most_extreme = match most_extreme {
                    None => Some(path_len),
                    Some(current_extreme) => Some(strategy(current_extreme, path_len)),
                    // Some(current_extreme) => Some(current_extreme.min(path_len)),
                    // Some(current_extreme) => Some(strategy(path_len)),
                };
            }

        }

        Ok(most_extreme.ok_or_else(|| miette::miette!("a"))?.to_string())
    }
}

// impl Solution for Day {
//     type Output = u32;

//     fn part1(&mut self) -> miette::Result<Self::Output> {
//         let cities = self.get_unique();
//         let mut overall_shortest = None;
    
//         // Try each city as a starting point
//         for start in cities.iter() {
//             let mut remaining: HashSet<_> = cities
//                 .iter()
//                 .filter(|&city| city != start)
//                 .copied()
//                 .collect();
            
//             let mut path_shortest = None;
//             self.find_shortest_path(start, &mut remaining, 0, &mut path_shortest);
            
//             // Update overall shortest if this path is shorter
//             if let Some(path_len) = path_shortest {
//                 overall_shortest = match overall_shortest {
//                     None => Some(path_len),
//                     Some(current_shortest) => Some(current_shortest.min(path_len))
//                 };
//             }
//         }
        
//         Ok(overall_shortest.unwrap())
//     }

//     fn part2(&mut self) -> miette::Result<Self::Output> {
//         let cities = self.get_unique();
//         let mut overall_longest = None;
    
//         // Try each city as a starting point
//         for start in cities.iter() {
//             let mut remaining: HashSet<_> = cities
//                 .iter()
//                 .filter(|&city| city != start)
//                 .copied()
//                 .collect();
            
//             let mut path_longest = None;
//             self.find_longest_path(start, &mut remaining, 0, &mut path_longest);
            
//             // Update overall longest if this path is longer
//             if let Some(path_len) = path_longest {
//                 overall_longest = match overall_longest {
//                     None => Some(path_len),
//                     Some(current_shortest) => Some(current_shortest.max(path_len))
//                 };
//             }
//         }
        
//         Ok(overall_longest.unwrap())
//     }
// }

fn main() -> miette::Result<()> {
    let input = include_str!("./inputs/2015-12-09.txt");
    // let mut day = Day::from_str(include_str!("./inputs/2015-12-09.txt"))?;
    let part1 = Day::<Shortest>::from_str(input)?.solve(Part::One)?;
    let part2 = Day::<Longest>::from_str(input)?.solve(Part::Two)?;

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
