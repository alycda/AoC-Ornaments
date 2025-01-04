//! Day 21: RPG Simulator 20XX

use std::{str::FromStr, vec};

use aoc_ornaments::{Part, Solution, nom::split_newlines};
use nom::{bytes::complete::{tag, take_until, take_while1}, character::complete::{alpha1, char, digit1, multispace0, multispace1, not_line_ending, space0, space1}, combinator::opt, multi::separated_list1, sequence::{preceded, terminated, tuple}, IResult};

#[derive(Debug, Clone, Copy)]
struct Stats {
    hp: u32,
    damage: u32,
    armor: u32,
}

type Player = Stats;
type Boss = Stats;

#[derive(Debug)]
struct Item {
    name: String,
    cost: u32,
    damage: u32,
    armor: u32,
}

impl Item {
    fn new(name: String, cost: &str, damage: &str, armor: &str) -> Self {
        Self {
            name,
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

#[derive(Debug)]
struct Day(Player, Items, Boss);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        // let (_, stats) = separated_list0(newline::<&str, nom::error::Error<&str>>, not_line_ending)(input).expect("invalid input");
        // dbg!(&stats);

        // dbg!(stats).iter().for_each(|stat| {
        //     dbg!(Self::parse_stat(stat));
        // });

        let (_, stats) = split_newlines(input)?;
        let boss = Self::parse_stats(&stats)?;

        // let (a, b) = separated_list1::<_, _, _, nom::error::Error<_>, _, _>(tag("\n"), alphanumeric1)(input).expect("invalid input");
        // if let Ok((a, b)) = separated_list0(newline, not_line_ending)(input) {}

        // let (a, b) = terminated(tag("\n"), digit1)(input).expect("invalid input");

        // todo!();

        let player = Stats {
            hp: 100,
            damage: 0,
            armor: 0,
        };

        Ok(Self(player, Items::new(), boss))
    }
}

impl Day {
    fn parse_stat(input: &str) -> IResult<&str, (&str, u32)> {
        let (input, (key, _, _, number)) = tuple((
            // Parse the key (e.g. "Hit Points")
            // terminated(alpha1, space0),
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

    fn parse_weapons(input: &str) -> IResult<&str, Weapon> {
        Self::parse_item(input);

        todo!()
    }

    fn parse_armor(input: &str) -> IResult<&str, Armor> {
        Self::parse_item(input);

        todo!()
    }

    // fn parse_item(input: &str) -> IResult<&str, (&str, u32, u32, u32)> {
    //     todo!()
    // }

    fn parse_rings(input: &str) {
        todo!()
    }

    ///
    /// ```rust
    /// #[test]
    /// fn test() {
    /// let (_, damage_ring) = parse_ring("Damage +1    25     1       0");
    /// let (_, defense_ring) = parse_ring("Defense +3   80     0       3");
    /// 
    /// assert_eq!(damage_ring, Item { name: "Damage +1".to_string(), cost: 25, damage: 1, armor: 0 });
    /// assert_eq!(defense_ring, Item { name: "Defense +3".to_string(), cost: 80, damage: 0, armor: 3 });
    /// }
    /// ```
    fn parse_ring(input: &str) -> IResult<&str, Item> {
        let (input, (name, _, cost, _, damage, _, armor)) = tuple((
            // Name: take characters until we hit multiple spaces
            take_while1(|c| c != ' '),
            multispace1,
            // Numbers separated by spaces
            digit1,
            multispace1,
            digit1,
            multispace1,
            digit1
        ))(input)?;

        dbg!(&name, cost, damage, armor);

        todo!()
    }

    fn parse_items(input: &str/*, shop: &mut Items */) -> miette::Result<Items> {
        // // let (_, items) = separated_list0(newline::<&str, nom::error::Error<&str>>, not_line_ending)(input)
        // //     .map_err(|e| miette::miette!("Failed to parse items: {}", e))?;
        // let (_, items) = split_newlines(input)?;

        // dbg!(&items);

        // // items.iter().map(|item| {
        // //     let parts: Vec<&str> = item.split_whitespace().collect();
        // //     let cost = parts[0].parse().unwrap();
        // //     let damage = parts[1].parse().unwrap();
        // //     let armor = parts[2].parse().unwrap();
        // //     Ok(Item { cost, damage, armor })
        // // }).collect()

        let mut shop = Items::new();
        let (input, sections) = separated_list1(
            tuple((tag("\n"), multispace0)),
            Self::section
        )(input)
            .map_err(|e| miette::miette!("Failed to parse sections: {}", e))?;

        dbg!(&sections);

        for section in sections {
            let (name, items) = section;
            // dbg!(&name, &items);

            match name.as_ref() {
                "Weapons" => shop.weapons = items,
                "Armor" => shop.armor = items,
                "Rings" => shop.rings = items,
                _ => return Err(miette::miette!("Unknown section: {}", name))
            }
        }

        // Ok(Items { weapons, armor, rings });
        Ok(shop)
    }

    fn section(input: &str) -> IResult<&str, (String, Vec<Item>)> {
        // dbg!(input);

        let (input, (section_name, _)) = tuple((
            terminated(take_until(":"), tag(":")),
            multispace0,
        ))(input)?;
        
        let (input, _) = Self::header_line(input)?;

        // dbg!(section_name, input);

        // let (input, items) = separated_list1(tag("\n"), Self::item_line)(input)?;
        let (input, items) = separated_list1(tag("\n"), Self::parse_item)(input)?;
        // dbg!(input, &items);
        
        Ok((input, (section_name.to_string(), items)))
    }

    fn header_line(input: &str) -> IResult<&str, ()> {
        let (input, _) = tuple((
            not_line_ending,
            tag("\n")
        ))(input)?;
        Ok((input, ()))
    }

    // ///
    // /// ```rust
    // /// let (_, item) = item_line("");
    // /// ```
    // fn item_line(input: &str) -> IResult<&str, Item> {
    //     // let (input, (name, _, cost, _, damage, _, armor)) = tuple((
    //     //     // Name: take characters until we hit multiple spaces
    //     //     take_while1(|c| c != ' ' || c != '+'),
    //     //     multispace1,
    //     //     // Numbers separated by spaces
    //     //     digit1,
    //     //     multispace1,
    //     //     digit1,
    //     //     multispace1,
    //     //     digit1
    //     // ))(input)?;

    //     let (input, name) = take_while1(|c| c != ' ')(input.trim())?;
    //     let (input, numbers) = preceded(
    //         multispace1,
    //         separated_list1(
    //             multispace1,
    //             digit1
    //         )
    //     )(input.trim())?;

    //     assert_eq!(numbers.len(), 3);
    
    //     Ok((input, Item {
    //         name: name.trim().to_string(),
    //         cost: numbers[0].parse().unwrap(),
    //         damage: numbers[1].parse().unwrap(),
    //         armor: numbers[2].parse().unwrap(),
    //     }))
    // }

    // // fn parse_hp(input: &str) -> IResult<&str, usize> {
    // //     dbg!(input);
    // //     let (input, _) = tag("Hit Points: ")(input)?;
    // //     dbg!(input);

    // //     let (input, hp) = u32(input)?;

    // //     Ok((input, hp as usize))
    // // }

    /// `xyz  X  Y  Z` or `abc +X  Y  Z  A`
    fn parse_item(input: &str) -> IResult<&str, Item> {
        // let (input, (name, modifier)) = Self::parse_item_name(input)?;
        let (input, name) = Self::parse_item_name(input)?;
        let (input, (cost, damage, armor)) = Self::parse_nums_to_tuple(input)?;

        Ok((input, Item::new(name, cost, damage, armor)))
    }

    // fn parse_item_name(input: &str) -> IResult<&str, (&str, Option<&str>)> {
    //     tuple((alpha1, opt(preceded(space0, Self::parse_modifier))))(input)
    // }

    /// `abc +X` or `xyz`
    fn parse_item_name(input: &str) -> IResult<&str, String> {
        let (input, (_, name, modifier)) = tuple((space0, alpha1, opt(preceded(space0, Self::parse_modifier))))(input)?;

        if let Some(modifier) = modifier {
            Ok((input, format!("{} +{}", name, modifier)))
        } else {
            Ok((input, name.to_string()))
        }
    }

    #[deprecated]
    fn parse_nums_to_vec(input: &str) -> IResult<&str, Vec<&str>> {
        separated_list1(space1, digit1)(input)
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
        // dbg!(&self);

        let input = "Weapons:    Cost  Damage  Armor
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
Defense +3   80     0       3";

        // let (a, b) = Self::header_line("Weapons:    Cost  Damage  Armor").expect("invalid input");
        // dbg!(a, b);

        // let (a, b) = separated_list1(tuple((tag("\n"), multispace0::<_, nom::error::Error<_>>)), not_line_ending)(input).expect("invalid input");
        // dbg!(a, b);

        let shop = Day::parse_items(input/*, &mut self.1 */)?;

        self.1 = shop;
        dbg!(&self);

        // let (a, name) = alpha1::<_, nom::error::Error<_>>("Damage +1    25     1       0").expect("invalid input");
        // dbg!(a, &name);

        // // let (a, b) = preceded(char::<_, nom::error::Error<_>>('+'), digit1)(a).expect("invalid input");
        // let (a, modifier) = preceded(tuple((space0, char::<_, nom::error::Error<_>>('+'), space0)), digit1)(a).expect("invalid input");
        // dbg!(a, modifier);

        // // let (a, b) = tuple((space1, digit1, space1, digit1, space1, digit1))(a).expect("invalid input");
        // let (a, b) = preceded(space0, separated_list1(space1::<_, nom::error::Error<_>>, digit1))(a).expect("invalid input");
        // dbg!(a, b);

        // let (a, b) = alpha1::<_, nom::error::Error<_>>("Leather      13     0       1").expect("invalid input");
        // dbg!(a, b);
        // let (a, b) = alpha1::<_, nom::error::Error<_>>("Greataxe     74     8       0").expect("invalid input");
        // dbg!(a, b);



        // let (a, b) = tuple((alpha1::<_, nom::error::Error<_>>, opt(preceded(space0, preceded(char('+'), digit1)))))("Defense +3   80     0       3").expect("invalid input");
        // dbg!(a, b);
        // let (a, b) = tuple((alpha1::<_, nom::error::Error<_>>, opt(preceded(space0, Self::parse_modifier))))("Platemail   102     0       5").expect("invalid input");
        // dbg!(a, b);

        // let (a, b) = dbg!(Self::parse_item_name("Defense +2   40     0       2")).expect("ok");
        // // dbg!(Self::parse_nums_to_vec(a));
        // let (a, b) = dbg!(Self::parse_item_name("Warhammer    25     6       0")).expect("ok");
        // dbg!(Self::parse_nums_to_tuple(a));

        dbg!(Self::parse_item("Dagger        8     4       0").expect("ok"));
        dbg!(Self::parse_item("Leather      13     0       1").expect("ok"));
        dbg!(Self::parse_item("Damage +1    25     1       0").expect("ok"));

        todo!()
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-21.txt"))?;
    let part1 = day.solve(Part::One)?;
    let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}