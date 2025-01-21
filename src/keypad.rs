//!

use std::{collections::HashMap, marker::PhantomData};

use crate::spatial::Position;

#[derive(Debug)]
pub struct ButtonPad<T> {
    map: HashMap<char, Position>,
    _kind: PhantomData<T>,
}

impl<T> ButtonPad<T> {
    pub fn new(map: HashMap<char, Position>) -> Self {
        Self {
            map,
            _kind: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for ButtonPad<T> {
    type Target = HashMap<char, Position>;
    
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

/// Calculator Layout
/// 
/// +---+---+---+
/// | 7 | 8 | 9 |
/// +---+---+---+
/// | 4 | 5 | 6 |
/// +---+---+---+
/// | 1 | 2 | 3 |
/// +---+---+---+
pub struct NumberPad3x3Calculator;

/// Telephone Layout
/// 
/// +---+---+---+
/// | 1 | 2 | 3 |
/// +---+---+---+
/// | 4 | 5 | 6 |
/// +---+---+---+
/// | 7 | 8 | 9 |
/// +---+---+---+
pub struct NumberPad3x3Telephone;

impl Default for ButtonPad<NumberPad3x3Calculator> {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert('7', Position::ZERO);
        map.insert('8', Position::new(1, 0));
        map.insert('9', Position::new(2, 0));
        map.insert('4', Position::new(0, 1));
        map.insert('5', Position::ONE);
        map.insert('6', Position::new(2, 1));
        map.insert('1', Position::new(0, 2));
        map.insert('2', Position::new(1, 2));
        map.insert('3', Position::new(2, 2));

        Self {
            map,
            _kind: PhantomData,
        }
    }
}

impl Default for ButtonPad<NumberPad3x3Telephone> {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert('1', Position::ZERO);
        map.insert('2', Position::new(1, 0));
        map.insert('3', Position::new(2, 0));
        map.insert('4', Position::new(0, 1));
        map.insert('5', Position::ONE);
        map.insert('6', Position::new(2, 1));
        map.insert('7', Position::new(0, 2));
        map.insert('8', Position::new(1, 2));
        map.insert('9', Position::new(2, 2));

        Self {
            map,
            _kind: PhantomData,
        }
    }
}