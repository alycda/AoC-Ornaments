//! Day 22: Wizard Simulator 20XX

use std::str::FromStr;

use aoc_ornaments::{Part, Solution};
use nom::{bytes::complete::{tag, take_until}, character::complete::{u32, space0}, sequence::{terminated, tuple}, IResult};

#[derive(Debug)]
struct Player {}

#[derive(Debug)]
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
    cost: usize,
    damage: usize,
    effect: Option<Effect>,
}

impl Spell {
    fn cast() {
        todo!()
    }
}

#[derive(Debug)]
enum Effect {
    Armor(usize),
    Damage(usize),
    Mana(usize),
}

// #[derive(Debug)]
// struct Effect {
//     armor: Option<()>,
//     damage: Option<()>,
//     mana: Option<()>,
// }

#[derive(Debug)]
struct Day(Player, Vec<Spell>, Boss);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let (_, boss) = Self::parse_boss_stats(input)
            // .map(|(_, boss)| boss)
            .map_err(|e| miette::miette!(e.to_owned()))?;
        

        Ok(Self(Player {}, vec![], boss))
    }
}

impl Day {
    fn parse_boss_stats(input: &str) -> IResult<&str, Boss> {
        // let (input, (_, _, _, hp)) = tuple((
        //     // Parse the key (e.g. "Hit Points")
        //     terminated(take_until(":"), space0),
        //     tag(":"),
        //     space0,
        //     // Parse the number
        //     u32,
        // ))(input)?;

        let (input, (_, hp)) = Self::parse_stat_line(input)?;
        let (_, (_, damage)) = Self::parse_stat_line(input)?;

        // dbg!(hp, damage);

        // Ok((input, Boss::new(hp, damage)))
        Ok((input, Boss { hp, damage }))
    }

    fn parse_stat_line(input: &str) -> IResult<&str, (String, u32)> {
        // let (input, (key, _, value)) = tuple((
        //     take_until(":"),
        //     tag(":"),
        //     u32,
        // ))(input)?;

        // Ok((input, (key.to_string(), value)))

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
        dbg!(input);

        // let (input, (_, cost, _, damage)) = tuple((
        //     // Parse the key (e.g. "Hit Points")
        //     terminated(take_until(" "), space0),
        //     u32,
        //     tag("mana."),
        //     u32,
        // ))(input)?;

        // Ok((input, Attack { cost: cost as usize, damage: damage as usize, effect: None }))

        todo!()
    }

    fn parse_spells(input: &str) -> IResult<&str, Vec<Spell>> {
        let (input, spells) = Self::parse_spell(input)?;
        dbg!(input, spells);

        todo!();

        // Ok((input, spells))
    }

    fn init_spells(&mut self) -> miette::Result<()> {
        let input = "Magic Missile costs 53 mana. It instantly does 4 damage.
Drain costs 73 mana. It instantly does 2 damage and heals you for 2 hit points.
Shield costs 113 mana. It starts an effect that lasts for 6 turns. While it is active, your armor is increased by 7.
Poison costs 173 mana. It starts an effect that lasts for 6 turns. At the start of each turn while it is active, it deals the boss 3 damage.
Recharge costs 229 mana. It starts an effect that lasts for 5 turns. At the start of each turn while it is active, it gives you 101 new mana.";

        Self::parse_spells(input).map_err(|e| miette::miette!(e.to_owned()))?;

        todo!()
    }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        self.init_spells()?;
        dbg!(&self);

        todo!()
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-22.txt"))?;
    let part1 = day.solve(Part::One)?;
    // let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    // println!("Part 2: {}", part2);

    Ok(())
}