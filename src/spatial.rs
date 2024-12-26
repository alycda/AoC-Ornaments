use std::{collections::{HashMap, HashSet}, str::FromStr};
use miette::Diagnostic;
use thiserror::Error;

pub type Position = glam::IVec2;
pub type Velocity = glam::IVec2;

pub fn manhattan_distance(a: &Position, b: &Position) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

/// a Region or set of Positions
pub type UniquePositions = HashSet<Position>;

#[derive(Debug, Default)]
pub struct Visited<T>(HashMap<Position, T>);

impl<T> std::ops::Deref for Visited<T> {
    type Target = HashMap<Position, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Visited<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Visited<T> {
    pub fn new(k: Position, v: T) -> Self {
        let mut map = HashMap::new();
        map.insert(k, v).expect("unique key");

        Self(map)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Direction {
    /// A, North
    Up,
    /// X, South
    Down,
    /// #, West
    Left,
    /// O, East
    Right
}

#[derive(Error, Diagnostic, Debug)]
pub enum DirectionError {
    #[error("Invalid direction character: {0}")]
    #[diagnostic(code(direction::invalid_char))]
    InvalidChar(char),

    #[error("Invalid direction string: {0}")]
    #[diagnostic(code(direction::invalid_str))]
    InvalidStr(String),

    #[error("Invalid direction mapping: Expected 4 unique directions")]
    #[diagnostic(code(direction::invalid_mapping))]
    InvalidMapping,

    // #[error("Invalid symbol: {0}")]
    // #[diagnostic(code(direction::invalid_symbol))]
    // InvalidSymbol(T),
}

impl FromStr for Direction {
    type Err = DirectionError;

    fn from_str(s: &str) -> miette::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "^" | "up" | "a" | "north" | "n" => Ok(Direction::Up),
            "v" | "down" | "x" | "south" | "s" => Ok(Direction::Down),
            // CARFEUL here, I thought PlayStation buttons (`#`) were cute...
            "<" | "left" | "#" | "west" | "w" => Ok(Direction::Left),
            ">" | "right" | "o" | "east" | "e" => Ok(Direction::Right),
            _ => Err(DirectionError::InvalidStr(s.to_string()))
        }
    }
}

impl Direction {
    /// Creates a parser function that maps custom direction symbols to Direction variants
    /// Order is [Up, Down, Left, Right]
    pub fn with_mapping4<T>(mapping: [T; 4]) -> impl Fn(&T) -> Result<Direction, DirectionError> 
    where 
        T: std::fmt::Display + Eq + Clone + std::hash::Hash,
    {
        // // Validate no duplicates using HashSet
        // let unique: HashSet<_> = mapping.iter().collect();
        // if unique.len() != 4 {
        //     return Err(DirectionError::InvalidMapping);
        // }

        move |s| {
            if *s == mapping[0] { Ok(Direction::Up) }
            else if *s == mapping[1] { Ok(Direction::Down) }
            else if *s == mapping[2] { Ok(Direction::Left) }
            else if *s == mapping[3] { Ok(Direction::Right) }
            else { Err(DirectionError::InvalidStr(s.to_string())) }
        }
    }

    /// for parsing from a CHAR, otherwise use [FromStr] because we get [String.parse] for free
    pub fn parse(c: char) -> miette::Result<Self, DirectionError> {
        match c.to_ascii_lowercase() {
            '^' | 'a' | 'n' => Ok(Direction::Up),
            'v' | 'x' | 's' => Ok(Direction::Down),
            // CARFEUL here, I thought PlayStation buttons (`#`) were cute...
            '<' | '#' | 'e' => Ok(Direction::Left),
            // also here: `o`
            '>' | 'o' | 'w' => Ok(Direction::Right),
            _ => Err(DirectionError::InvalidChar(c))
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }

    pub fn to_offset(&self) -> Position {
        match self {
            Direction::Up => Position::NEG_Y,
            Direction::Down => Position::Y,
            Direction::Left => Position::NEG_X,
            Direction::Right => Position::X
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn turn_left(&self) -> Direction {
        self.turn_right().opposite()
    }
}