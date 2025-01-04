//!
//! 
//! ## working with [miette]
//! 
//! since nom is usually zero-copy and relies on borrowed data, you may get errors like:
//! 
//! > borrowed data escapes outside of associated function
//! 
//! when using:
//! 
//! ```rust
//! .map_err(|e| miette::miette!(e))?;
//! ```
//! 
//! The problem is that `e` is a borrowed reference to the input data, and it's being returned from the function.
//! 
//! the fix:
//! 
//! ```rust
//! .map_err(|e| miette::miette!(e.to_owned()))?;
//! ```
//! 
//! if a new string is allocated (or owership is taken) then the borrowed reference is no longer needed and the error goes away.
//! 
//! you can also:
//! 
//! ```rust
//! .map_err(|e| miette::miette!("Failed to parse input: {e}"))
//! ```
//! 
//! which allocates a new string, similar to format! macro.

use nom::{character::complete::{newline, not_line_ending}, multi::separated_list0};

// type IResult<'a, O> = nom::IResult<&'a str, O, nom::error::Error<&'a str>>;

/// outputs a miette result for use in FromStr, otherwise you don't need the turbofish and .expect
pub fn split_newlines(input: &str) -> miette::Result<(&str, Vec<&str>)> {
    separated_list0(newline::<&str, nom::error::Error<&str>>, not_line_ending)(input)
        .map_err(|e| miette::miette!("Failed to parse input: {e}"))
}