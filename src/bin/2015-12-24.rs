//! Day 24: It Hangs in the Balance

use std::str::FromStr;

use aoc_ornaments::{Part, ArgSolution};
use itertools::Itertools;

#[derive(Debug, derive_more::Deref)]
struct Day(Vec<usize>);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let packages = input.lines().map(|line| line.parse().unwrap()).collect();

        Ok(Self(packages))
    }
}

impl Day {
    fn quantum_entanglement(&self, compartments: usize) -> Result<usize, miette::Error> {
        let total_weight = self.iter().sum::<usize>();
        let group_weight = total_weight / compartments;
    
        // Start with smallest possible combinations and return early
        for n in 1..self.len() {
            if let Some(min_product) = self.iter()
                .combinations(n)
                .filter(|combo| combo.iter().copied().sum::<usize>() == group_weight)
                .map(|combo| combo.iter().copied().product::<usize>())
                .min()
            {
                return Ok(min_product);
            }
        }
    
        Err(miette::miette!("No valid combinations found"))
    }
}

impl ArgSolution<usize> for Day {
    type Output = usize;

    fn solve(&mut self, _part: Part, count: usize) -> aoc_ornaments::SolutionResult<String> {
        Ok(self.quantum_entanglement(count)?.to_string())
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("../inputs/2015-12-24.txt"))?;
    let part1 = day.solve(Part::One, 3)?;
    let part2 = day.solve(Part::Two, 4)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;

    // Fixture for the example packages
    #[fixture]
    fn example_packages() -> Day {
        Day(vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11])
    }

    #[fixture]
    fn group_weight(example_packages: Day) -> usize {
        example_packages.iter().sum::<usize>() / 3
    }

    #[rstest]
    #[case(vec![11, 9], 99)]
    #[case(vec![10, 9, 1], 90)]
    #[case(vec![10, 8, 2], 160)]
    #[case(vec![10, 7, 3], 210)]
    #[case(vec![10, 5, 4, 1], 200)]
    #[case(vec![10, 5, 3, 2], 300)]
    #[case(vec![10, 4, 3, 2, 1], 240)]
    #[case(vec![9, 8, 3], 216)]
    #[case(vec![9, 7, 4], 252)]
    #[case(vec![9, 5, 4, 2], 360)]
    #[case(vec![8, 7, 5], 280)]
    #[case(vec![8, 5, 4, 3], 480)]
    #[case(vec![7, 5, 4, 3, 1], 420)]
    fn test_quantum_entanglement_combinations(
        #[case] combination: Vec<usize>,
        #[case] expected_qe: usize,
        group_weight: usize
    ) {
        assert_eq!(combination.iter().sum::<usize>(), group_weight);
        assert_eq!(combination.iter().product::<usize>(), expected_qe);
    }

    #[rstest]
    fn test_optimal_solution(example_packages: Day) {
        assert_eq!(example_packages.quantum_entanglement(3).unwrap(), 99);
        assert_ne!(example_packages.quantum_entanglement(3).unwrap(), 90);
    }

    #[rstest]
    #[case(
        vec![11, 9],           // Group 1
        vec![10, 8, 2],        // Group 2
        vec![7, 5, 4, 3, 1]    // Group 3
    )]
    #[case(
        vec![10, 9, 1],
        vec![11, 7, 2],
        vec![8, 5, 4, 3]
    )]
    #[case(
        vec![10, 8, 2],
        vec![11, 9],
        vec![7, 5, 4, 3, 1]
    )]
    #[case(
        vec![10, 7, 3],
        vec![11, 9],
        vec![8, 5, 4, 2, 1]
    )]
    fn test_valid_group_combinations(
        #[case] group1: Vec<usize>,
        #[case] group2: Vec<usize>,
        #[case] group3: Vec<usize>,
        group_weight: usize
    ) {
        // Verify each group sums to the expected weight
        assert_eq!(group1.iter().sum::<usize>(), group_weight);
        assert_eq!(group2.iter().sum::<usize>(), group_weight);
        assert_eq!(group3.iter().sum::<usize>(), group_weight);

        // Verify all numbers are used exactly once
        let mut all_numbers = group1.clone();
        all_numbers.extend(group2);
        all_numbers.extend(group3);
        all_numbers.sort_unstable();
        
        assert_eq!(
            all_numbers,
            vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11]
        );
    }

    #[rstest]
    fn test_group_weight_properties(example_packages: Day, group_weight: usize) {
        let total_weight: usize = example_packages.iter().sum();
        assert_eq!(total_weight, 60);  // Sum of 1..5 and 7..11
        assert_eq!(total_weight % 3, 0);  // Must be divisible by 3
        assert_eq!(group_weight, 20);  // Each group must sum to 20
    }
}