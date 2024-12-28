use std::{collections::{BTreeMap, BTreeSet, HashMap, HashSet}, str::FromStr};
use miette::Diagnostic;
use thiserror::Error;

pub type Position = glam::IVec2;
pub type Velocity = glam::IVec2;

pub fn manhattan_distance(a: &Position, b: &Position) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

// Canonical ordering: always store with lesser point first
// pub type Distances<T> = BTreeMap<(String, String), T>;

#[derive(Debug)]
pub struct Distances<T>(BTreeMap<(String, String), T>);

impl<T> std::ops::Deref for Distances<T> {
    type Target = BTreeMap<(String, String), T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Distances<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: std::ops::Add<Output = T> + Clone + Copy + Ord> Distances<T> {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert_ordered(&mut self, a: String, b: String, distance: T) {
        let key = if a < b {
            (a, b)
        } else {
            (b, a)
        };
        self.0.insert(key, distance);
    }

    pub fn get_distance(&self, a: &str, b: &str) -> Option<&T> {
        let key = if a < b {
            (a.to_string(), b.to_string())
        } else {
            (b.to_string(), a.to_string())
        };
        self.get(&key)
    }

    /// not sure if btree vs hashset is better here
    pub fn to_unique(&self) -> UniqueLocations {
        let mut set = BTreeSet::new();
        for (a, b) in self.keys() {
            set.insert(a.clone());
            set.insert(b.clone());
        }

        set
    }

    /// recursive
    pub fn find_shortest_path(&self, current: &str, remaining: &mut HashSet<&str>, total: T, shortest: &mut Option<T>) {
        // If no(thing)s remain, we've found a complete path
        if remaining.is_empty() {
            *shortest = match *shortest {
                None => Some(total),
                Some(s) => Some(s.min(total))
            };
            return;
        }

        // Try each remaining city as the next step
        let neighbors: Vec<_> = remaining.iter().copied().collect();
        for next in neighbors {
            // Get distance to this neighbor
            let key = if current < next {
                (current.to_string(), next.to_string())
            } else {
                (next.to_string(), current.to_string())
            };
            let distance = self.get(&key).unwrap();
            
            // Visit this neighbor
            remaining.remove(next);
            self.find_shortest_path(
                next,
                remaining,
                total + *distance,
                shortest
            );
            remaining.insert(next);
        }
    }

    /// recursive
    pub fn find_longest_path(&self, current: &str, remaining: &mut HashSet<&str>, total: T, shortest: &mut Option<T>) {
        // If no(thing)s remain, we've found a complete path
        if remaining.is_empty() {
            *shortest = match *shortest {
                None => Some(total),
                Some(s) => Some(s.max(total))
            };
            return;
        }

        // Try each remaining city as the next step
        let neighbors: Vec<_> = remaining.iter().copied().collect();
        for next in neighbors {
            // Get distance to this neighbor
            let key = if current < next {
                (current.to_string(), next.to_string())
            } else {
                (next.to_string(), current.to_string())
            };
            let distance = self.get(&key).unwrap();
            
            // Visit this neighbor
            remaining.remove(next);
            self.find_longest_path(
                next,
                remaining,
                total + *distance,
                shortest
            );
            remaining.insert(next);
        }
    }

    pub fn get_unique_cities(&self) -> HashSet<&str> {
        let mut cities = HashSet::new();
        for ((city1, city2), _) in &self.0 {
            cities.insert(city1.as_str());
            cities.insert(city2.as_str());
        }
        cities
    }
}

pub fn distance() {}

pub type UniqueLocations = BTreeSet<String>;

/// a Region or set of Positions
pub type UniquePositions = HashSet<Position>;

#[derive(Debug)]
pub struct Grid<T>(Vec<Vec<T>>);

impl<T> std::ops::Deref for Grid<T> {
    type Target = Vec<Vec<T>>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: std::fmt::Debug + Copy + PartialEq> Grid<T> {
    pub fn initialize(width: usize, height: usize, value: T) -> Self {
        let mut grid = Vec::with_capacity(height);
        for _ in 0..height {
            let row = vec![value; width];
            grid.push(row);
        }

        Self(grid)
    }

    pub fn get_width(&self) -> usize {
        self[0].len()
    }

    pub fn get_height(&self) -> usize {
        self.len()
    }

    pub fn get_at_unbounded(&self, pos: Position) -> T {
        self[pos.y as usize][pos.x as usize]
    }

    /// Bounded by the grid's dimensions
    pub fn get_at(&self, pos: Position) -> Option<T> {
        // if pos.x < 0 || pos.y < 0 || pos.x >= self.get_width() as i32 || pos.y >= self.get_height() as i32 {
        if !self.in_bounds(pos) {
            return None;
        }

        Some(self[pos.y as usize][pos.x as usize])
        // Some(self.get_at_unbounded(pos))
    }

    pub fn in_bounds(&self, pos: Position) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.get_width() as i32 && pos.y < self.get_height() as i32
    }

    pub fn set_at(&mut self, pos: Position, value: T) -> Option<()> {
        if !self.in_bounds(pos) {
            return None;
        }
        self[pos.y as usize][pos.x as usize] = value;
        Some(())
    }

    // Unbounded version if you need it
    pub fn set_at_unbounded(&mut self, pos: Position, value: T) {
        self[pos.y as usize][pos.x as usize] = value;
    }

    /// Walks the grid from top-left to bottom-right
    pub fn walk<F: FnMut(Position) -> O, O>(&self, mut see: F) {
        for row in 0..self.get_height() {
            for col in 0..self.get_width() {
                let pos = Position::new(col as i32, row as i32);

                see(pos);
            }
        }
    }

    pub fn walk_region<F>(&mut self, start: Position, end: Position, mut see: F) 
    where 
        F: FnMut(&mut Self, Position)
    {
        for row in start.y..=end.y {
            for col in start.x..=end.x {
                let pos = Position::new(col, row);
                see(self, pos);
            }
        }
    }

    // pub fn walk_region<F: FnMut(Position) -> O, O>(&self, start: Position, end: Position, mut see: F) {
    //     for row in start.y..=end.y {
    //         for col in start.x..=end.x {
    //             let pos = Position::new(col, row);
    //             see(pos);
    //         }
    //     }
    // }

    /// because Position is a type and not a NewType, we can't impl FromStr for it
    pub fn position_from_str(s: &str) -> miette::Result<Position> {
        let parts: Vec<i32> = s.split(',').map(|s| s.parse().unwrap()).collect();
        Ok(Position::new(parts[0], parts[1]))
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    

}