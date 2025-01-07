//! Day 24: It Hangs in the Balance

use std::str::FromStr;

use aoc_ornaments::{Part, Solution};
use itertools::Itertools;

struct Day(Vec<usize>);

impl std::ops::Deref for Day {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let packages = input.lines().map(|line| line.parse().unwrap()).collect();

        Ok(Self(packages))
    }
}

impl Day {
    // fn find_combinations_with_sum(numbers: &[usize], target: usize, size: usize) -> Vec<Vec<usize>> {
    //     numbers
    //         .iter()
    //         .combinations(size)
    //         .filter(|combo| combo.iter().copied().sum::<usize>() == target)
    //         .map(|combo| combo.into_iter().copied().collect())
    //         .collect()
    // }

    fn find_all_combinations_with_sum(numbers: &[usize], target: usize) -> Vec<Vec<usize>> {
        (1..=numbers.len())
            .flat_map(|size| {
                numbers
                    .iter()
                    .combinations(size)
                    .filter(|combo| combo.iter().copied().sum::<usize>() == target)
                    .map(|combo| combo.into_iter().copied().collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            })
            .collect()
    }
    
    // If you need the product of each combination:
    fn get_products(combinations: &[Vec<usize>]) -> Vec<usize> {
        combinations
            .iter()
            .map(|combo| combo.iter().product())
            .collect()
    }

    fn find_best_combination(numbers: &[usize], target: usize) -> Option<(Vec<usize>, usize)> {
        let mut best: Option<(Vec<usize>, usize)> = None;
        
        // Start with smallest combinations first
        for size in 1..=numbers.len() {
            // If we already found a smaller combination, we can stop
            if let Some((best_combo, _)) = &best {
                if size > best_combo.len() {
                    break;
                }
            }
            
            for combo in numbers.iter().combinations(size) {
                let sum: usize = combo.iter().copied().sum();
                if sum == target {
                    let combo_vec: Vec<usize> = combo.into_iter().copied().collect();
                    let product: usize = combo_vec.iter().product();
                    
                    match &best {
                        None => best = Some((combo_vec, product)),
                        Some((best_combo, best_product)) => {
                            if combo_vec.len() < best_combo.len() || 
                               (combo_vec.len() == best_combo.len() && product > *best_product) {
                                best = Some((combo_vec, product));
                            }
                        }
                    }
                }
            }
        }
        
        best
    }
}

impl Solution for Day {
    type Output = usize;

    // for n in range(1,len(weights)):
    // good = [x for x in list(combinations(weights,n)) if sum(x) == sum(weights)/3]
    // if len(good) > 0:
    //     break

    // smallest = min(good, key=lambda x: reduce(mul, x))
    // print reduce(mul, smallest)

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        let total_weight = dbg!(self.iter().sum::<usize>());
        let group_weight = total_weight / 3;
        // let combinations = Day::find_combinations_with_sum(self, group_weight, 3);

        // let combinations = Self::find_all_combinations_with_sum(&self, group_weight);
        // // println!("Combinations that sum to {}: {:?}", group_weight, combinations);
        // println!("Number of combinations: {}", combinations.len());
        
        // let products = Self::get_products(&combinations);
        // println!("Products of combinations: {:?}", products);

        // let a = Self::find_best_combination(&self.iter().copied().collect::<Vec<_>>(), group_weight)
        //     .map(|(_, product)| product as usize);
        //     // .ok_or_else(|| "No valid combination found".into())

        // Ok(a.unwrap())

        // Find first group of valid combinations
        let mut good_combinations = Vec::new();
        for n in 1..self.len() {
            good_combinations = self.iter()
                .combinations(n)
                .filter(|combo| combo.iter().copied().sum::<usize>() == group_weight)
                .map(|combo| combo.iter().copied().cloned().collect::<Vec<_>>())
                .collect();
                
            if !good_combinations.is_empty() {
                break;
            }
        }

        // Find the combination with smallest product
        if let Some(smallest) = good_combinations.iter()
            .min_by_key(|combo| combo.iter().product::<usize>()) 
        {
            let product: usize = smallest.iter().product();
            println!("Smallest product: {}", product);

            return Ok(product)
        }

        todo!()
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-24.txt"))?;
    let part1 = day.solve(Part::One)?;
    // let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1); // < 344266178323
    // println!("Part 2: {}", part2);

    Ok(())
}