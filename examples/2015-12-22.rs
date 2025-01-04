//! Day 22: Wizard Simulator 20XX

use std::str::FromStr;

use aoc_ornaments::{Part, Solution};
use nom::{branch::alt, bytes::complete::{tag, take_until}, character::complete::{not_line_ending, space0, u32}, combinator::{map, opt}, multi::separated_list1, sequence::{terminated, tuple}, IResult};

#[derive(Debug)]
struct Player {
    hp: u32,
    mana: u32,
}

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

#[derive(Debug)]
enum Effect {
    /// duration, effect
    Armor(usize, u32),
    /// duration, effect. duration may be 0 for instant damage
    Damage(usize, u32),
    /// duration, effect
    Mana(usize, u32),
    Heal(u32),
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
        

        Ok(Self(Player { hp: 50, mana: 500 }, vec![], boss))
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
        // dbg!(input);

        let mut damage = 0;
        let mut effect = None;

        let (input, (name, _, cost, _, description)) = tuple((
            take_until(" costs"),    // Spell name
            tag(" costs "),         // Literal "costs" with spaces
            u32, // Mana cost as number
            tag(" mana. "),         // Literal "mana."
            not_line_ending                    // Rest of the description
        ))(input)?;

        // dbg!(name, cost, description);

        let (_, effects) = Self::parse_effect(description)?;
        // dbg!(&effects);
        // dbg!(&effects[0]);
        // dbg!(&effects[1]);
        
        // let damage = match &effects[0] {
        //     Effect::Damage(_, damage) => *damage,
        //     _ => panic!("Expected damage effect"),
        // };

        for eff in effects {
            match eff {
                Effect::Damage(_, d) => damage = d,
                any => effect = Some(any),
                // Effect::Heal(d) => effect = Some(Effect)
                // _ => unimplemented!("this parser only handles instant damage and healing")
            }
        }

        // let (input, b) = tuple((not_line_ending, tag(" costs "), not_line_ending))(input)?;
        // dbg!(b);

        // // let (input, (_, cost, _, damage)) = tuple((
        // //     // Parse the key (e.g. "Hit Points")
        // //     terminated(take_until(" "), space0),
        // //     u32,
        // //     tag("mana."),
        // //     u32,
        // // ))(input)?;

        // // Ok((input, Attack { cost: cost as usize, damage: damage as usize, effect: None }))

        Ok((input, Spell { name: name.to_string(), cost, damage, effect }))
    }

    fn parse_effect(input: &str) -> IResult<&str, Vec<Effect>> {
        dbg!(input);
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

        // dbg!(damage, heal);

        let mut effects = vec![Effect::Damage(0, damage)];

        if let Some((_, heal, _)) = heal {
            effects.push(Effect::Heal(heal));
        }

        Ok((input, effects))
    }

    fn duration_effect(input: &str) -> IResult<&str, Vec<Effect>> {
        let (input, _) = tag("It starts an effect that lasts for ")(input)?;

        let (input, turns) = u32(input)?;
        let (input, _) = tag(" turns")(input)?;

        // dbg!(turns);
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
        move |input: &str| {
            // First find the keyword
            let (input, _) = take_until(keyword)(input)?;
            let (input, _) = tag(keyword)(input)?;
            // Now find the next number, skipping any text in between
            let (input, _) = Self::take_until_number(input)?;
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

        // let (input, name) = take_until(" costs")(input)?;
        // dbg!(&name);
        // let (input, _) = tag(" costs ")(input)?;
        // // let (input, cost) = map_res(digit1, str::parse)(input)?;
        // let (input, cost) = u32(input)?;
        // dbg!(&cost);
        // let (input, _) = tag(" mana. ")(input)?;
        // let (input, description) = rest(input)?;
        // dbg!(&description);

        // let (input, spells) = separated_list1(tag("\n"), Self::parse_spell)(input)?;

        // // let (input, spells) = Self::parse_spell(input)?;
        // dbg!(&spells);

        // todo!();

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