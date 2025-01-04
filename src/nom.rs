use nom::{character::complete::{newline, not_line_ending}, multi::separated_list0};

// type IResult<'a, O> = nom::IResult<&'a str, O, nom::error::Error<&'a str>>;

/// outputs a miette result for use in FromStr, otherwise you don't need the turbofish and .expect
pub fn split_newlines(input: &str) -> miette::Result<(&str, Vec<&str>)> {
    separated_list0(newline::<&str, nom::error::Error<&str>>, not_line_ending)(input)
        .map_err(|e| miette::miette!("Failed to parse input: {e}"))
}