use miette::Diagnostic;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};
use thiserror::Error;

pub type Position = glam::IVec2;
pub type Velocity = glam::IVec2;

pub fn manhattan_distance(a: &Position, b: &Position) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

/// a Region or set of Positions
pub type UniquePositions = HashSet<Position>;
/// Up, Right, Down, Left
pub const DIRECTIONS: [Position; 4] = [Position::NEG_Y, Position::X, Position::Y, Position::NEG_X];

/// NW, SW, NE, SE
pub const DIAGONALS: [Position; 4] = [
    Position::NEG_ONE,
    Position::new(-1, 1),
    Position::new(1, -1),
    Position::ONE,
];

/// Up, SE, Right, NE, Down, NW, Left, SW
pub const ALL_DIRECTIONS: [Position; 8] = [
    Position::NEG_Y,
    Position::ONE,
    Position::X,
    Position::new(1, -1),
    Position::Y,
    Position::NEG_ONE,
    Position::NEG_X,
    Position::new(-1, 1),
];

pub trait Spatial: Display {
    fn new(width: u32, height: u32) -> Self;

    fn in_bounds(&self, pos: Position) -> bool {
        pos.x >= 0
            && pos.y >= 0
            && pos.x < self.get_width() as i32
            && pos.y < self.get_height() as i32
    }
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;

    fn get_orthogonal_neighbors(&self, at: Position) -> Vec<Position> {
        let mut neighbors = Vec::new();

        for delta in DIRECTIONS.iter() {
            let neighbor = at + *delta;

            // boundary check
            if self.in_bounds(neighbor) {
                neighbors.push(neighbor);
            }
        }

        neighbors
    }

    /// Depth-First Search
    fn flood_fill<F: Fn(Position) -> bool>(&self, start: Position, test: F) -> UniquePositions {
        let mut region = UniquePositions::new();
        let mut stack = vec![start];

        while let Some(pos) = stack.pop() {
            if !region.insert(pos) {
                continue;
            }

            for neighbor in self.get_orthogonal_neighbors(pos) {
                if test(neighbor) {
                    stack.push(neighbor)
                }
            }
        }

        region
    }
}

#[derive(Debug, Clone, derive_more::Deref)]
/// only stores the interesting positions and minmax bounds
pub struct PhantomGrid(#[deref] pub UniquePositions, pub (Position, Position));

impl Spatial for PhantomGrid {
    fn new(width: u32, height: u32) -> Self {
        Self(
            UniquePositions::new(),
            (Position::ZERO, Position::new(width as i32, height as i32)),
        )
    }

    fn get_width(&self) -> u32 {
        self.1 .1.x as u32
    }

    fn get_height(&self) -> u32 {
        self.1 .1.y as u32
    }
}

impl Display for PhantomGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let interesting = &self.0;
        let bounds = self.1;
        let (min, max) = bounds;

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let pos = Position::new(x, y);
                if interesting.contains(&pos) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, derive_more::Deref, derive_more::DerefMut)]
pub struct Grid<T>(pub Vec<Vec<T>>);

impl FromStr for Grid<char> {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(
            input.lines().map(|line| line.chars().collect()).collect(),
        ))
    }
}

impl FromStr for Grid<bool> {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(
            input
                .lines()
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect(),
        ))
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
    /// ORTHOGONAL neighbors. use [Self::get_all_neighbors] for all 8
    pub fn get_neighbors(&self, pos: Position) -> Vec<(Position, T)> {
        let mut neighbors = Vec::with_capacity(4);

        for delta in DIRECTIONS.iter() {
            let new_pos = pos + *delta;

            // Boundary check matching the working version
            if new_pos.x >= 0
                && new_pos.x < self.get_width() as i32
                && new_pos.y >= 0
                && new_pos.y < self.get_height() as i32
            {
                neighbors.push((new_pos, self.get_at_unbounded(new_pos)));
            }
        }

        neighbors
    }

    pub fn get_all_neighbors(&self, pos: Position) -> Vec<(Position, T)> {
        let mut neighbors = Vec::with_capacity(8);

        for delta in ALL_DIRECTIONS.iter() {
            let new_pos = pos + *delta;

            // Boundary check matching the working version
            if new_pos.x >= 0
                && new_pos.x < self.get_width() as i32
                && new_pos.y >= 0
                && new_pos.y < self.get_height() as i32
            {
                neighbors.push((new_pos, self.get_at_unbounded(new_pos)));
            }
        }

        neighbors
    }

    pub fn in_bounds(&self, pos: Position) -> bool {
        pos.x >= 0
            && pos.y >= 0
            && pos.x < self.get_width() as i32
            && pos.y < self.get_height() as i32
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
        F: FnMut(&mut Self, Position),
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

#[derive(Debug, Default, derive_more::Deref, derive_more::DerefMut)]
pub struct Visited<T>(HashMap<Position, T>);

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
    Right,
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
            _ => Err(DirectionError::InvalidStr(s.to_string())),
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
            if *s == mapping[0] {
                Ok(Direction::Up)
            } else if *s == mapping[1] {
                Ok(Direction::Down)
            } else if *s == mapping[2] {
                Ok(Direction::Left)
            } else if *s == mapping[3] {
                Ok(Direction::Right)
            } else {
                Err(DirectionError::InvalidStr(s.to_string()))
            }
        }
    }

    /// for parsing from a CHAR, otherwise use [FromStr] because we get [String.parse] for free
    pub fn parse(c: &char) -> miette::Result<Self, DirectionError> {
        match c.to_ascii_lowercase() {
            '^' | 'a' | 'n' | 'u' => Ok(Direction::Up),
            'v' | 'x' | 's' | 'd' => Ok(Direction::Down),
            // CARFEUL here, I thought PlayStation buttons (`#`) were cute...
            '<' | '#' | 'e' | 'l' => Ok(Direction::Left),
            // also here: `o`
            '>' | 'o' | 'w' | 'r' => Ok(Direction::Right),
            _ => Err(DirectionError::InvalidChar(*c)),
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn to_offset(&self) -> Position {
        match self {
            Direction::Up => Position::NEG_Y,
            Direction::Down => Position::Y,
            Direction::Left => Position::NEG_X,
            Direction::Right => Position::X,
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
