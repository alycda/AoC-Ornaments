//! Day 21: RPG Simulator 20XX

use std::{str::FromStr, vec};

use aoc_ornaments::{Part, Solution, nom::split_newlines};
use nom::{bytes::complete::{tag, take_until}, character::complete::{alpha1, char, digit1, multispace0, not_line_ending, space0, space1}, combinator::opt, multi::separated_list1, sequence::{preceded, terminated, tuple}, IResult};

#[derive(Debug, Clone, Copy)]
struct Stats {
    hp: u32,
    damage: u32,
    armor: u32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            hp: 100,
            damage: 0,
            armor: 0,
        }
    }
}

impl FromStr for Stats {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let (_, stats) = split_newlines(input)?;
        let boss = Self::parse_stats(&stats)?;

        Ok(boss)
    }
}

impl Stats {
    fn parse_stat(input: &str) -> IResult<&str, (&str, u32)> {
        let (input, (key, _, _, number)) = tuple((
            // Parse the key (e.g. "Hit Points")
            terminated(take_until(":"), space0),
            tag(":"),
            space0,
            // Parse the number
            digit1
        ))(input)?;
        
        // Convert the number string to u32
        let value = number.parse().unwrap();
        Ok((input, (key, value)))
    }

    fn parse_stats(input: &[&str]) -> miette::Result<Stats> {
        let mut stats = Stats {
            hp: 0,
            damage: 0,
            armor: 0,
        };
    
        for line in input {
            let (_, (key, value)) = Self::parse_stat(line)
                .map_err(|e| miette::miette!("Failed to parse line: {}", e))?;
            
            match key {
                "Hit Points" => stats.hp = value,
                "Damage" => stats.damage = value,
                "Armor" => stats.armor = value,
                _ => return Err(miette::miette!("Unknown stat: {}", key))
            }
        }
    
        Ok(stats)
    }
}

type Player = Stats;
type Boss = Stats;

#[derive(Debug)]
struct Item {
    _name: String,
    cost: u32,
    damage: u32,
    armor: u32,
}

impl Item {
    fn new(_name: String, cost: &str, damage: &str, armor: &str) -> Self {
        Self {
            _name,
            cost: cost.parse().expect("a number"),
            damage: damage.parse().expect("a number"),
            armor: armor.parse().expect("a number"),
        }
    }
}

type Weapon = Item;
type Armor = Item;
type Ring = Item;

/// Shop
#[derive(Debug)]
struct Items {
    weapons: Vec<Weapon>,
    armor: Vec<Armor>,
    rings: Vec<Ring>,
}

impl Items {
    fn new() -> Self {
        Self {
            weapons: vec![],
            armor: vec![],
            rings: vec![],
        }
    }
}

#[derive(Debug, derive_more::Deref)]
struct Day(Items);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let items = Day::parse_items(input)?;

        Ok(Day(items))
    }
}

impl Day {
    fn generate_loadouts(&self) -> Vec<Vec<&Item>> {
        let mut loadouts = Vec::new();
        
        // First, iterate through required weapons
        for weapon in &self.weapons {
            // Base loadout with just a weapon
            loadouts.push(vec![weapon]);
            
            // Optional armor combinations
            for armor in &self.armor {
                loadouts.push(vec![weapon, armor]);
                
                // Now handle ring combinations
                // Use combinations instead of permutations to avoid duplicates
                for ring in &self.rings {
                    loadouts.push(vec![weapon, armor, ring]);
                }
                
                // For two rings, we need to ensure we don't pick the same ring twice
                for (i, ring1) in self.rings.iter().enumerate() {
                    for ring2 in self.rings.iter().skip(i + 1) {  // skip(i + 1) ensures we don't reuse rings
                        loadouts.push(vec![weapon, armor, ring1, ring2]);
                    }
                }
            }
            
            // Handle rings without armor
            for ring in &self.rings {
                loadouts.push(vec![weapon, ring]);
            }
            
            // Two rings without armor
            for (i, ring1) in self.rings.iter().enumerate() {
                for ring2 in self.rings.iter().skip(i + 1) {
                    loadouts.push(vec![weapon, ring1, ring2]);
                }
            }
        }
        
        loadouts
    }

    fn calculate_loadout_stats(loadout: &[&Item]) -> Stats {
        let mut stats = Player::default();
        
        // Add up all damage and armor from equipment
        for item in loadout {
            stats.damage += item.damage;
            stats.armor += item.armor;
        }
        
        stats
    }

    fn simulate_battle(&self, player_stats: Stats, boss_stats: Stats) -> bool {
        // BUGFIX: Use saturating_sub to prevent underflow
        let player_damage = (player_stats.damage.saturating_sub(boss_stats.armor)).max(1);
        let boss_damage = (boss_stats.damage.saturating_sub(player_stats.armor)).max(1);
                
        // Calculate turns needed to win using ceiling division
        let turns_to_kill_boss = (boss_stats.hp + player_damage - 1) / player_damage;
        let turns_to_kill_player = (player_stats.hp + boss_damage - 1) / boss_damage;
                
        turns_to_kill_boss <= turns_to_kill_player
    }


    fn parse_items(input: &str) -> miette::Result<Items> {
        let mut shop = Items::new();
        let (_, sections) = separated_list1(
            tuple((tag("\n"), multispace0)),
            Self::section
        )(input)
            .map_err(|e| miette::miette!("Failed to parse sections: {}", e))?;


        for section in sections {
            let (name, items) = section;

            match name.as_ref() {
                "Weapons" => shop.weapons = items,
                "Armor" => shop.armor = items,
                "Rings" => shop.rings = items,
                _ => return Err(miette::miette!("Unknown section: {}", name))
            }
        }
        Ok(shop)
    }

    fn section(input: &str) -> IResult<&str, (String, Vec<Item>)> {
        let (input, (section_name, _)) = tuple((
            terminated(take_until(":"), tag(":")),
            multispace0,
        ))(input)?;
        
        let (input, _) = Self::header_line(input)?;

        let (input, items) = separated_list1(tag("\n"), Self::parse_item)(input)?;
        
        Ok((input, (section_name.to_string(), items)))
    }

    fn header_line(input: &str) -> IResult<&str, ()> {
        let (input, _) = tuple((
            not_line_ending,
            tag("\n")
        ))(input)?;
        Ok((input, ()))
    }

    /// `xyz  X  Y  Z` or `abc +X  Y  Z  A`
    fn parse_item(input: &str) -> IResult<&str, Item> {
        // let (input, (name, modifier)) = Self::parse_item_name(input)?;
        let (input, name) = Self::parse_item_name(input)?;
        let (input, (cost, damage, armor)) = Self::parse_nums_to_tuple(input)?;

        Ok((input, Item::new(name, cost, damage, armor)))
    }

    /// `abc +X` or `xyz`
    fn parse_item_name(input: &str) -> IResult<&str, String> {
        let (input, (_, name, modifier)) = tuple((space0, alpha1, opt(preceded(space0, Self::parse_modifier))))(input)?;

        if let Some(modifier) = modifier {
            Ok((input, format!("{} +{}", name, modifier)))
        } else {
            Ok((input, name.to_string()))
        }
    }

    /// ` X  Y  Z`
    fn parse_nums_to_tuple(input: &str) -> IResult<&str, (&str, &str, &str)> {
        let (remainder, (_, cost, _, damage, _, armor)) = tuple((space0, digit1, space1, digit1, space1, digit1))(input)?;

        Ok((remainder, (cost, damage, armor)))
    }

    /// `+X`
    fn parse_modifier(input: &str) -> IResult<&str, &str> {
        preceded(char('+'), digit1)(input)
    }

}

impl Solution for Day {
    type Output = u32;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        // let player = Player::default();
        let boss = Boss::from_str(include_str!("../inputs/2015-12-21.txt")).expect("Failed to parse boss stats");
        let combos = self.generate_loadouts();

        let winning_costs = combos.iter()
            .filter_map(|combo| {
                let stats = Self::calculate_loadout_stats(combo);
                if self.simulate_battle(stats, boss) {
                    Some(combo.iter().map(|item| item.cost).sum())
                } else {
                    None
                }
            })
            .min() // Get the lowest cost among winning combinations
            .ok_or_else(|| miette::miette!("No winning combinations found"))?;

        Ok(winning_costs)
    }

    fn part2(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        // let player = Player::default();
        let boss = Boss::from_str(include_str!("../inputs/2015-12-21.txt")).expect("Failed to parse boss stats");

        let losing_costs = self.generate_loadouts().iter()
            .filter_map(|combo| {
                let stats = Self::calculate_loadout_stats(combo);
                if !self.simulate_battle(stats, boss) {
                    Some(combo.iter().map(|item| item.cost).sum())
                } else {
                    None
                }
            })
            .max() // Get the highest cost among losing combinations
            .ok_or_else(|| miette::miette!("No losing combinations found"))?;

        Ok(losing_costs)
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str("Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0

Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5

Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3")?;
    let part1 = day.solve(Part::One)?;
    let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}