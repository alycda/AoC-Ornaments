//! Day 22: Wizard Simulator 20XX

use std::str::FromStr;

use aoc_ornaments::{Part, Solution};
use nom::{branch::alt, bytes::complete::{tag, take_until}, character::complete::{not_line_ending, space0, u32}, combinator::{map, opt, recognize}, multi::separated_list1, sequence::{terminated, tuple}, IResult};
use rand::seq::SliceRandom;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
struct Player {
    hp: u32,
    mana: u32,
}

#[derive(Debug, Clone, Copy)]
struct Boss {
    hp: u32,
    damage: u32,
}

impl Boss {
    fn new(hp: &str, damage: &str) -> Self {
        Self {
            hp: hp.parse().unwrap(),
            damage: damage.parse().unwrap(),
        }
    }
}

/// cast
#[derive(Debug)]
struct Spell {
    name: String,
    cost: u32,
    /// instant
    damage: u32,
    effect: Option<Effect>,
}

impl Spell {
    fn cast() {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
enum Effect {
    /// duration, effect
    Armor(usize, u32),
    /// duration, effect. duration may be 0 for instant damage
    Damage(usize, u32),
    /// duration, effect
    Mana(usize, u32),
    Heal(u32),
}

#[derive(Debug)]
struct Day(Player, Vec<Spell>, Boss);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let (_, boss) = Self::parse_boss_stats(input)
            // .map(|(_, boss)| boss)
            .map_err(|e| miette::miette!(e.to_owned()))?;
        

        Ok(Self(Player { hp: 50, mana: 500 }, vec![], boss))
    }
}

impl Day {
    fn parse_boss_stats(input: &str) -> IResult<&str, Boss> {
        let (input, (_, hp)) = Self::parse_stat_line(input)?;
        let (_, (_, damage)) = Self::parse_stat_line(input)?;

        Ok((input, Boss { hp, damage }))
    }

    fn parse_stat_line(input: &str) -> IResult<&str, (String, u32)> {
        let (input, (key, _, _, value)) = tuple((
            // Parse the key (e.g. "Hit Points")
            terminated(take_until(":"), space0),
            tag(":"),
            space0,
            // Parse the number
            u32,
        ))(input)?;

        Ok((input, (key.to_string(), value)))
    }

    fn parse_spell(input: &str) -> IResult<&str, Spell> {
        let mut damage = 0;
        let mut effect = None;
    
        let (input, (name, _, cost, _, description)) = tuple((
            take_until(" costs"),    
            tag(" costs "),         
            u32, 
            tag(" mana. "),         
            not_line_ending                    
        ))(input)?;
    
        let (_, effects) = Self::parse_effect(description)?;
    
        for eff in effects {
            match eff {
                // Only set damage for instant damage effects
                Effect::Damage(0, d) => damage = d,
                // Otherwise store it as an effect
                any => effect = Some(any),
            }
        }
    
        Ok((input, Spell { name: name.to_string(), cost, damage, effect }))
    }

    fn parse_effect(input: &str) -> IResult<&str, Vec<Effect>> {
        alt((Self::instant_damage, Self::duration_effect))(input)
    }

    fn instant_damage(input: &str) -> IResult<&str, Vec<Effect>> {
        let (input, _) = tag("It instantly does ")(input)?;
        let (input, damage) = u32(input)?;
        let (input, _) = tag(" damage")(input)?;
        
        // Optional healing part
        let (input, heal) = opt(tuple((
            tag(" and heals you for "),
            u32,
            tag(" hit points")
        )))(input)?;

        let mut effects = vec![Effect::Damage(0, damage)];

        if let Some((_, heal, _)) = heal {
            effects.push(Effect::Heal(heal));
        }

        Ok((input, effects))
    }

    fn duration_effect(input: &str) -> IResult<&str, Vec<Effect>> {
        // dbg!(input);
        let (input, _) = tag("It starts an effect that lasts for ")(input)?;

        let (input, turns) = u32(input)?;
        let (input, _) = tag(" turns")(input)?;

        let (input, effect) = Self::flexible_duration_effect_parser(input, turns as usize)?;
        // dbg!(&effect);

        Ok((input, vec![effect]))
    }

    // More flexible duration effect parser that looks for key patterns
    fn flexible_duration_effect_parser(input: &str, turns: usize) -> IResult<&str, Effect> {
        let (remaining, effect) = alt((
            // Shield: look for "armor" keyword
            map(
                Self::effect_value_parser("armor"),
                |value| Effect::Armor(turns, value)
            ),
            // Poison: look for "damage" with optional "deals" prefix
            map(
                Self::effect_value_parser("damage"),
                |value| Effect::Damage(turns, value)
            ),
            // Recharge: look for "mana" with optional "gives" prefix
            map(
                Self::effect_value_parser("mana"),
                |value| Effect::Mana(turns, value)
            )
        ))(input)?;

        // Consume the rest of the line without caring about exact wording
        let (input, _) = not_line_ending(remaining)?;
        Ok((input, effect))
    }

    // More accurately: finds a keyword and then extracts a nearby number
    fn effect_value_parser(keyword: &'static str) -> impl Fn(&str) -> IResult<&str, u32> {
        move |original_input: &str| {
            // dbg!(original_input);

            // First find the keyword
            let (input, _) = take_until(keyword)(original_input)?;
            // dbg!(input);

            let (input, _) = tag(keyword)(input)?;

            // dbg!(input);

            // Now find the next number, skipping any text in between
            let (input, _) = Self::take_until_number(original_input)?;
            let (input, value) = u32(input)?;
            
            Ok((input, value))
        }
    }

    // Helper to skip until we find a digit
    fn take_until_number(input: &str) -> IResult<&str, &str> {
        Self::take_until_matching(input, |c| c.is_ascii_digit())
    }

    // Generic helper to take until a condition is met
    fn take_until_matching<F>(input: &str, condition: F) -> IResult<&str, &str>
    where 
        F: Fn(char) -> bool,
    {
        let pos = input.find(|c| condition(c))
            .ok_or_else(|| nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::TakeUntil)))?;
        Ok((&input[pos..], &input[..pos]))
    }

    fn parse_spells(input: &str) -> IResult<&str, Vec<Spell>> {
        let (input, spells) = separated_list1(tag("\n"), Self::parse_spell)(input)?;

        Ok((input, spells))
    }

    fn init_spells(&mut self) -> miette::Result<()> {
        let input = "Magic Missile costs 53 mana. It instantly does 4 damage.
Drain costs 73 mana. It instantly does 2 damage and heals you for 2 hit points.
Shield costs 113 mana. It starts an effect that lasts for 6 turns. While it is active, your armor is increased by 7.
Poison costs 173 mana. It starts an effect that lasts for 6 turns. At the start of each turn while it is active, it deals the boss 3 damage.
Recharge costs 229 mana. It starts an effect that lasts for 5 turns. At the start of each turn while it is active, it gives you 101 new mana.";

        let (_, spells) = Self::parse_spells(input).map_err(|e| miette::miette!(e.to_owned()))?;
        self.1 = spells;

        Ok(())
    }
}

#[derive(Clone, Eq, Hash)]
struct GameState {
    player_hp: i32,
    player_mana: i32,
    boss_hp: i32,
    mana_spent: i32,
    shield_timer: i32,
    poison_timer: i32,
    recharge_timer: i32,
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.mana_spent == other.mana_spent
    }
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.mana_spent.cmp(&self.mana_spent)
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl GameState {
    fn new(player_hp: i32, player_mana: i32, boss_hp: i32) -> Self {
        Self {
            player_hp,
            player_mana,
            boss_hp,
            mana_spent: 0,
            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,
        }
    }

    fn apply_effects(&mut self) {
        if self.shield_timer > 0 { self.shield_timer -= 1; }
        if self.poison_timer > 0 { 
            self.boss_hp -= 3;
            self.poison_timer -= 1; 
        }
        if self.recharge_timer > 0 { 
            self.player_mana += 101;
            self.recharge_timer -= 1;
        }
    }

    fn cast_spell(&mut self, spell: &Spell) -> bool {
        if self.player_mana < spell.cost as i32 { return false; }

        self.player_mana -= spell.cost as i32;
        self.mana_spent += spell.cost as i32;

        match spell.name.as_str() {
            "Magic Missile" => self.boss_hp -= 4,
            "Drain" => {
                self.boss_hp -= 2;
                self.player_hp += 2;
            },
            "Shield" => {
                if self.shield_timer > 0 { return false; }
                self.shield_timer = 6;
            },
            "Poison" => {
                if self.poison_timer > 0 { return false; }
                self.poison_timer = 6;
            },
            "Recharge" => {
                if self.recharge_timer > 0 { return false; }
                self.recharge_timer = 5;
            },
            _ => unreachable!()
        }
        true
    }

    fn process_boss_turn(&mut self) -> bool {
        let armor = if self.shield_timer > 0 { 7 } else { 0 };
        self.player_hp -= std::cmp::max(1, 10 - armor);

        true
    }

    fn is_game_over(&self) -> bool {
        self.player_hp <= 0 || self.boss_hp <= 0
    }

    fn has_armor(&self) -> bool {
        self.shield_timer > 0
    }

    fn has_poison(&self) -> bool {
        self.poison_timer > 0
    }

    // fn get_valid_spells(&self, spells: &[Spell]) -> Vec<&Spell> {
    //     spells.iter().filter(|s| {
    //         s.cost <= self.player_mana as u32 && match s.effect {
    //             Some(Effect::Armor(..)) => !self.has_armor(),
    //             Some(Effect::Damage(..)) => !self.has_poison(),
    //             Some(Effect::Mana(..)) => self.recharge_timer == 0,
    //             None => true,
    //             _ => false,
    //         }
    //     }).collect()
    // }
}

impl Day {

    // fn simulate_battle(&self, rng: &mut impl rand::Rng) -> Option<u32> {
    //     let mut state = GameState::new(self.0.clone(), self.2.clone());
        
    //     while !state.is_game_over() {
    //         state.apply_effects();
    //         if state.boss.hp == 0 {
    //             return Some(state.mana_spent);
    //         }
            
    //         let valid_spells = state.get_valid_spells(&self.1);
    //         if valid_spells.is_empty() || state.player.mana < 53 {
    //             return None;
    //         }
    
    //         // More robust spell selection
    //         let spell = if !state.has_armor() && rng.gen_bool(0.66) {
    //             valid_spells.iter().find(|s| s.name == "Shield")
    //                 .or_else(|| valid_spells.first()) // Fallback if Shield isn't available
    //         } else if state.player.mana < 400 && state.boss.hp > 10 && rng.gen_bool(0.66) {
    //             valid_spells.iter().find(|s| s.name == "Recharge")
    //                 .or_else(|| valid_spells.first())
    //         } else if !state.has_poison() && rng.gen_bool(0.66) {
    //             valid_spells.iter().find(|s| s.name == "Poison")
    //                 .or_else(|| valid_spells.first())
    //         } else {
    //             let idx = rng.gen_range(0..valid_spells.len());
    //             Some(&valid_spells[idx])
    //         }.unwrap(); // This unwrap is now safe because we always have a fallback
    
    //         if state.cast_spell(spell).is_none() {
    //             return None;
    //         }
    
    //         state.apply_effects();
    //         if state.boss.hp == 0 {
    //             return Some(state.mana_spent);
    //         }
            
    //         if !state.process_boss_turn() {
    //             return None;
    //         }
    //     }
        
    //     if state.boss.hp == 0 {
    //         Some(state.mana_spent)
    //     } else {
    //         None
    //     }
    // }

    fn find_least_mana(&self) -> Option<i32> {
        let mut heap = BinaryHeap::new();
        let mut seen = HashSet::new();
        
        let initial = GameState::new(50, 500, self.2.hp as i32);
        heap.push(initial.clone());
        seen.insert(initial);

        while let Some(mut state) = heap.pop() {
            // Boss is dead?
            if state.boss_hp <= 0 {
                return Some(state.mana_spent);
            }

            // Player's turn
            state.apply_effects();
            if state.boss_hp <= 0 {
                return Some(state.mana_spent);
            }

            // Try each spell
            for spell in &self.1 {
                let mut new_state = state.clone();
                if !new_state.cast_spell(spell) { continue; }

                // Boss turn
                new_state.apply_effects();
                if new_state.boss_hp <= 0 {
                    return Some(new_state.mana_spent);
                }

                // Boss attacks
                let armor = if new_state.shield_timer > 0 { 7 } else { 0 };
                new_state.player_hp -= std::cmp::max(1, self.2.damage as i32 - armor);

                if new_state.player_hp > 0 && !seen.contains(&new_state) {
                    seen.insert(new_state.clone());
                    heap.push(new_state);
                }
            }
        }

        None
    }
}

impl Solution for Day {
    type Output = i32;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        self.init_spells()?;
        dbg!(&self);

        // Self::find_least_mana_monte_carlo(&self).ok_or_else(|| miette::miette!("No solution found"))
        Self::find_least_mana(&self).ok_or_else(|| miette::miette!("No solution found"))
    }
}

fn effect_duration(effect: &Effect) -> usize {
    match effect {
        Effect::Armor(duration, _) => *duration,
        Effect::Damage(duration, _) => *duration,
        Effect::Mana(duration, _) => *duration,
        Effect::Heal(_) => 0,  // Instant effect
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-22.txt"))?;
    let part1 = day.solve(Part::One)?;
    // let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1); // < 1309
    // println!("Part 2: {}", part2);

    Ok(())
}