#![feature(nonzero_internals)]
//! Triangle
//! 
//! https://exercism.org/tracks/rust/exercises/triangle
//! The Ruby Koans triangle project, parts 1 & 2

use std::num::{NonZero, ZeroablePrimitive};
use std::ops::Add;

pub struct Triangle<T: ZeroablePrimitive>([NonZero<T>; 3]);

impl<T: ZeroablePrimitive + Ord + Add> Triangle<T> {
    pub fn build(mut sides: [NonZero<T>; 3]) -> Option<Triangle<T>> {
        sides.sort();

        let tryangle = Self(sides);

        match tryangle.is_real() {
            true => Some(tryangle),
            _ => None
        }
    }

    pub fn is_real(&self) -> bool {
        let [a, b, c] = self.0;

        (a + b >= c.into()) && 
        (b + c >= a.into()) && 
        (a + c >= b.into())
    }
    
    /// all values must be equal
    pub fn is_equilateral(&self) -> bool {        
        self.0.iter().all(|x| *x == self.0[0])
    }

    /// either sides 1 and 2 are equal or sides 2 and 3 are equal (pre-sorted)
    /// technically does not exclude equilateral
    pub fn is_scalene(&self) -> bool {
        self.is_equilateral() ||
        self.0[0..=1].iter().all(|x| *x == self.0[0]) ||
        self.0[1..=2].iter().all(|x| *x == self.0[1])
    }

    pub fn is_isosceles(&self) -> bool {
        !self.is_scalene() && !self.is_equilateral()
    }
}



fn main() -> miette::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([2, 2, 2], "equal int", true)]
    #[case([2, 3, 2], "2 sides equal", false)]
    #[case([2, 3, 5], "all sides inequal", false)]
    // #[case([0.5, 0.5, 0.5], "equal float", true)]
    #[trace] //This attribute enable tracing
    fn equilateral(#[case] input: u32, #[case] name: &str, #[case] expected: bool) {
        let triangle = Triangle::build(input);
        assert!(triangle.is_some());
        assert_eq!(triangle.is_equilateral, expected);
    }

}