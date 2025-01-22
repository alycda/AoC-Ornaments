//! Day 13: Knights of the Dinner Table

use std::{collections::HashSet, str::FromStr};

use aoc_ornaments::{graph::Distances, Part, Solution};

type Happiness = Distances<i64>;

/// values are NOT reflexive. (A, B) is not the same as (B, A)
#[derive(Debug, derive_more::Deref)]
struct Day(Happiness);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let mut map = Happiness::new();

        input.lines().for_each(|line| {
            // len() - 1 to remove the period
            let parts: Vec<&str> = line[..line.len() - 1].split_whitespace().collect();

            match parts.as_slice() {
                [a, "would", "gain", n, "happiness", "units", "by", "sitting", "next", "to", b,] => {
                    map.insert((a.to_string(), b.to_string()), n.parse().unwrap());
                },
                [a, "would", "lose", n, "happiness", "units", "by", "sitting", "next", "to", b,] => {
                    map.insert((a.to_string(), b.to_string()), -n.parse::<i64>().unwrap());
                },
                // _ => return Err(miette::MietteError::from("invalid input")),
                _ => todo!(),
            }
        });

        Ok(Self(map))
    }
}

impl Day {
    pub fn get_pair(&self, a: &str, b: &str) -> i64 {
        let a_to_b = self.0.get(&(a.to_string(), b.to_string())).unwrap_or(&0);
        let b_to_a = self.0.get(&(b.to_string(), a.to_string())).unwrap_or(&0);
        a_to_b + b_to_a        
    }

    /// TODO: TravelingSales::best_circular
    // Modified find_longest_path to handle circular seating
    pub fn find_happiest_arrangement(&self, start: &str, current: &str, remaining: &mut HashSet<&str>, total: i64, happiest: &mut Option<i64>) {
        if remaining.is_empty() {
            // Don't forget to add happiness between last and first person!
            let final_happiness = total + self.get_pair(current, start);
            *happiest = match *happiest {
                None => Some(final_happiness),
                Some(h) => Some(h.max(final_happiness))
            };
            return;
        }

        // Try each remaining person as the next in the arrangement
        let candidates: Vec<_> = remaining.iter().copied().collect();
        for next in candidates {
            let pair_happiness = self.get_pair(current, next);
            
            remaining.remove(next);
            self.find_happiest_arrangement(
                start,
                next,
                remaining,
                total + pair_happiness,
                happiest
            );
            remaining.insert(next);
        }
    }
}

impl Solution for Day {
    type Output = i64;

    /// Traveling Salesman Circular
    fn solve(&mut self, part: Part) -> aoc_ornaments::SolutionResult<String> {
        let mut people = self.get_unique();
        let mut max_happiness = None;

        let start = match part {
            Part::One => {
                let s = people.iter().next().unwrap().to_owned();
                people.remove(&s);
                s
            },
            Part::Two => {
                people.insert("Me");
                "Me"
            },
            _ => unimplemented!("Part 3")
        };

        self.find_happiest_arrangement(start, start, &mut people, 0, &mut max_happiness);

        Ok(max_happiness.ok_or_else(|| miette::miette!("No happiness found"))?.to_string())
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("../inputs/2015-12-13.txt"))?;
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
    fn test_cases_part1() -> miette::Result<()> {
        let input = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";
        let mut day = Day::from_str(input)?;

        assert_eq!(day.solve(Part::One)?, "330");

        Ok(())
    }

//     #[test]
//     fn test_display() -> miette::Result<()> {
//         let expected = "
//      +41 +46
// +55   David    -2
// Carol       Alice
// +60    Bob    +54
//      -7  +83";

//         Ok(())
//     }

}
