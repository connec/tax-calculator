//! Defines a struct that wraps a GBP amount.

use num_format::{Locale, ToFormattedString};
use std::fmt;
use std::num::ParseIntError;

/// The maximum amount of GBP that can be represented.
pub static MAX: Gbp = Gbp(std::u32::MAX);

/// Represents errors that can occur when parsing GBP.
///
/// This is a superset of the errors that can occur when parsing an integer with one additional
/// case: the decimal value may be invalid (too long or short).
#[derive(Debug)]
pub enum ParseError {
    InvalidFloat(ParseIntError),
    InvalidDecimal,
}

impl From<ParseIntError> for ParseError {
    fn from(error: ParseIntError) -> Self {
        ParseError::InvalidFloat(error)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidFloat(error) => error.fmt(f),
            ParseError::InvalidDecimal => write!(f, "Invalid decimal for GBP value"),
        }
    }
}

/// Represents an amount of Great British Pounds, stored as whole pence.
///
/// This provides useful formatting and parsing methods and implements necssary arithemtic operators
/// for convenience.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Gbp(u32);

impl Gbp {
    /// Construct a `Gbp` from a number of pounds.
    ///
    /// This accounts for the fact that we store values as pence.
    pub fn from_pounds(pounds: u32) -> Self {
        Gbp(pounds * 100)
    }
}

impl fmt::Display for Gbp {
    /// Display a GBP amount, formatted like: £1,234,567.89
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pounds = self.0 / 100;
        let pence = self.0 % 100;
        write!(
            f,
            "£{}.{:02}",
            pounds.to_formatted_string(&Locale::en),
            pence
        )
    }
}

impl std::str::FromStr for Gbp {
    type Err = ParseError;

    /// Parse a currency amount from a string.
    ///
    /// This is a slightly naive approach but it keeps things simple: we strip leading "£", split
    /// pounds and pence, remove any "," from the pounds, then combine and parse as a u32.
    fn from_str(mut string: &str) -> Result<Self, ParseError> {
        string = string.trim_start_matches('£');
        let (pounds, pence) = match string.find('.') {
            Some(index) => {
                let split = string.split_at(index);
                (split.0, split.1.trim_start_matches('.'))
            }
            None => (string, "00"),
        };
        if pence.len() != 2 {
            return Err(ParseError::InvalidDecimal);
        }
        let pounds = pounds
            .chars()
            .filter(|c| *c != ',')
            .collect::<String>()
            .parse::<u32>()?
            * 100;
        let pence = pence.parse::<u32>()?;
        Ok(Gbp(pounds + pence))
    }
}

impl std::ops::Sub for Gbp {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Gbp(self.0 - rhs.0)
    }
}

impl std::ops::SubAssign<Gbp> for Gbp {
    fn sub_assign(&mut self, rhs: Gbp) {
        self.0 -= rhs.0
    }
}

impl std::ops::Mul<f64> for Gbp {
    type Output = Gbp;

    fn mul(self, other: f64) -> Gbp {
        Gbp((self.0 as f64 * other).round() as u32)
    }
}

impl<'a> std::iter::Sum<&'a Gbp> for Gbp {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Gbp>,
    {
        Gbp(iter.map(|gbp| gbp.0).sum())
    }
}
