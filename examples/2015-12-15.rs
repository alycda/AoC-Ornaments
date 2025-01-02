//! Day 15: Science for Hungry People

use std::{collections::HashMap, ops::Add, str::FromStr};

use aoc_ornaments::{Part, Solution};
use itertools::Itertools;
use nom::{bytes::complete::{tag, take_until}, character::complete::{i32, space0, space1}, multi::separated_list1, sequence::{delimited, preceded, terminated, tuple}, IResult};

type Ingredients = HashMap<String, Properties>;

#[derive(Debug, Clone, Copy)]
struct Properties {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl FromStr for Properties {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let (_, props) = Day::parse_properties(input).map_err(|e| miette::miette!("Failed to parse properties: {}", e))?;

        Ok(props)
    }
}

impl From<(i32, i32, i32, i32, i32)> for Properties {
    fn from((capacity, durability, flavor, texture, calories): (i32, i32, i32, i32, i32)) -> Self {
        Self {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

impl From<(&str, &str, &str, &str, &str)> for Properties {
    fn from((capacity, durability, flavor, texture, calories): (&str, &str, &str, &str, &str)) -> Self {
        Self {
            capacity: capacity.parse().expect("a number"),
            durability: durability.parse().expect("a number"),
            flavor: flavor.parse().expect("a number"),
            texture: texture.parse().expect("a number"),
            calories: calories.parse().expect("a number"),
        }
    }
}

impl From<[&str; 5]> for Properties {
    fn from([capacity, durability, flavor, texture, calories]: [&str; 5]) -> Self {
        Self {
            capacity: capacity.parse().expect("a number"),
            durability: durability.parse().expect("a number"),
            flavor: flavor.parse().expect("a number"),
            texture: texture.parse().expect("a number"),
            calories: calories.parse().expect("a number"),
        }
    }
}

impl Add for Properties {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            capacity: self.capacity + other.capacity,
            durability: self.durability + other.durability,
            flavor: self.flavor + other.flavor,
            texture: self.texture + other.texture,
            calories: 0
        }
    }
}

impl Properties {
    fn new(capacity: &str, durability: &str, flavor: &str, texture: &str, calories: &str) -> Self {
        Self {
            capacity: capacity.parse().expect("a number"),
            durability: durability.parse().expect("a number"),
            flavor: flavor.parse().expect("a number"),
            texture: texture.parse().expect("a number"),
            calories: calories.parse().expect("a number"),
        }
    }

    fn teaspoon(&self, count: usize) -> Self {
        // self.capacity * self.durability * self.flavor * self.texture

        Self {
            capacity: self.capacity * count as i32,
            durability: self.durability * count as i32,
            flavor: self.flavor * count as i32,
            texture: self.texture * count as i32,
            calories: 0,
        }
    }

    fn score(&self) -> usize {
        let capacity = self.capacity.max(0) as usize;
        let durability = self.durability.max(0) as usize;
        let flavor = self.flavor.max(0) as usize;
        let texture = self.texture.max(0) as usize;

        capacity * durability * flavor * texture
    }
}

impl From<Vec<(&str, i32)>> for Properties {
    fn from(props: Vec<(&str, i32)>) -> Self {
        // We can use try_into to convert the Vec to an array
        let arr: [(&str, i32); 5] = props.try_into()
            .expect("Expected exactly 5 properties");
            
        // Now we need to arrange the values in the correct order
        // Since the property names could come in any order, we should match on them
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;
        let mut calories = 0;

        for (name, value) in arr {
            match name {
                "capacity" => capacity = value,
                "durability" => durability = value,
                "flavor" => flavor = value,
                "texture" => texture = value,
                "calories" => calories = value,
                _ => panic!("Unknown property: {}", name),
            }
        }

        Self {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

#[derive(Debug)]
struct Day(Ingredients);

impl std::ops::Deref for Day {
    type Target = Ingredients;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(input.lines().fold(Ingredients::new(), |mut map, line| {
            let (_, (name, props)) = Self::parse_ingredient(line).expect("valid input");

            map.insert(name, props);

            map
        })))
    }
}

impl Day {
    fn parse_ingredient(input: &str) -> IResult<&str, (String, Properties)> {
        let (list, name) = terminated(
            // This will capture everything until the colon
            take_until(":"),
            // This consumes the colon and any following whitespace
            tuple((tag(":"), space0))
        )(input)?;

        // dbg!(name, list);

        let (remainder, props) = Self::parse_properties(list)?;
        // dbg!(&remainder);

        Ok((remainder, (name.to_string(), props)))
    }

    fn parse_properties(input: &str) -> IResult<&str, Properties> {
        let (remainder, props) = separated_list1(
            delimited(space0, tag(","), space0),  // This handles " , " as separator
            Self::parse_property
        )(input)?;

        Ok((remainder, Properties::from(props)))
    }

    // Parse a single property like "capacity 2"
    fn parse_property(input: &str) -> IResult<&str, (&str, i32)> {
        tuple((
            // Property name
            take_until(" "),
            // Space followed by number
            preceded(space1, i32)
        ))(input)
    }

    fn score_recipe(&self, amounts: &[(String, usize)]) -> usize {
        // Sum up the properties for each ingredient according to its amount
        let total_properties = amounts.iter().fold(
            Properties {
                capacity: 0,
                durability: 0,
                flavor: 0,
                texture: 0,
                calories: 0,
            },
            |acc, (name, amount)| {
                acc + self.0[name].teaspoon(*amount)
            }
        );
        
        total_properties.score()
    }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        // dbg!(&self);

        // self.iter().for_each(|(name, props)| {
        //     dbg!(name, props);
        // });

        // todo!()

        let ingredients: Vec<String> = self.0.keys().cloned().collect();
        
        // Generate combinations that sum to 100
        (0..=100)
            .combinations_with_replacement(ingredients.len() - 1)
            .filter_map(|amounts| {
                let sum: usize = amounts.iter().sum();
                if sum <= 100 {
                    let last_amount = 100 - sum;
                    let mut full_amounts = amounts.clone();
                    full_amounts.push(last_amount);
                    
                    // Pair ingredients with amounts
                    let recipe: Vec<(String, usize)> = ingredients.iter()
                        .cloned()
                        .zip(full_amounts)
                        .collect();
                        
                    Some(self.score_recipe(&recipe))
                } else {
                    None
                }
            })
            .max()
            .ok_or_else(|| miette::miette!("No valid combinations found"))
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-15.txt"))?;
    let part1 = day.solve(Part::One)?;
    // let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1); // > 11754288
    // println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case((44, 56), (68, 80, 152, 76))]
    fn test_cases_part1(#[case] tsp: (usize, usize), #[case] expected: (usize, usize, usize, usize)) {
//         let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
// Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

//         let mut day = Day::from_str(input).unwrap();
        // let b = day.get("Butterscotch").unwrap();
        // let c = day.get("Cinnamon").unwrap();

        let b = Properties::from_str("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8").expect("invalid input");
        let c = Properties::from_str("Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3").expect("invalid input");

        dbg!(&b, &c);
        dbg!(b.teaspoon(tsp.0), c.teaspoon(tsp.1));
        let z = dbg!(b.teaspoon(tsp.0) + c.teaspoon(tsp.1));

        assert_eq!(z.score(), expected.0 * expected.1 * expected.2 * expected.3);
    }

    #[test]
    fn test_part1() {
        let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

        let mut day = Day::from_str(input).unwrap();
        let result = day.solve(Part::One).unwrap();

        assert_eq!(result, "62842880");
    }
}