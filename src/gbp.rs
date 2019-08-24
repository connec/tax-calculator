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
    ///
    /// ```
    /// # use tax_calculator::Gbp;
    /// let amount = Gbp::from_pounds(100);
    /// ```
    pub fn from_pounds(pounds: u32) -> Self {
        Gbp(pounds * 100)
    }
}

impl fmt::Display for Gbp {
    /// Display a GBP amount, formatted like: £1,234,567.89
    ///
    /// ```
    /// # use tax_calculator::Gbp;
    /// assert_eq!(
    ///     format!("{}", Gbp::from_pounds(100)),
    ///     "£100.00"
    /// );
    /// ```
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
    ///
    /// ```
    /// # use tax_calculator::Gbp;
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// # fn try_main() -> Result<(), tax_calculator::gbp::ParseError> {
    /// let amount: Gbp = "£1,234,567.89".parse()?;
    /// assert_eq!(
    ///     format!("{}", amount),
    ///     "£1,234,567.89"
    /// );
    /// #     Ok(())
    /// # }
    /// ```
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

    /// Subtract a GBP amount from this one.
    ///
    /// ```
    /// # use tax_calculator::Gbp;
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// # fn try_main() -> Result<(), tax_calculator::gbp::ParseError> {
    /// let balance = Gbp::from_pounds(1000);
    /// let cost: Gbp = "£437.83".parse()?;
    /// let remaining = balance - cost;
    /// assert_eq!(
    ///     format!("{}", remaining),
    ///     "£562.17"
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    fn sub(self, rhs: Self) -> Self {
        Gbp(self.0 - rhs.0)
    }
}

impl std::ops::SubAssign<Gbp> for Gbp {
    /// Subtract a GBP amount from this one (mutating).
    ///
    /// ```
    /// # use tax_calculator::Gbp;
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// # fn try_main() -> Result<(), tax_calculator::gbp::ParseError> {
    /// let mut balance = Gbp::from_pounds(1000);
    /// let cost: Gbp = "£437.83".parse()?;
    /// balance -= cost;
    /// assert_eq!(
    ///     format!("{}", balance),
    ///     "£562.17"
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    fn sub_assign(&mut self, rhs: Gbp) {
        self.0 -= rhs.0
    }
}

impl std::ops::Mul<f64> for Gbp {
    type Output = Gbp;

    /// Multiply a GBP amount by a float.
    ///
    /// ```
    /// # use tax_calculator::Gbp;
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// # fn try_main() -> Result<(), tax_calculator::gbp::ParseError> {
    /// let cost: Gbp = "£0.99".parse()?;
    /// let total = cost * 5.0;
    /// assert_eq!(
    ///     format!("{}", total),
    ///     "£4.95"
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    fn mul(self, other: f64) -> Gbp {
        Gbp((self.0 as f64 * other).round() as u32)
    }
}

impl<'a> std::iter::Sum<&'a Gbp> for Gbp {
    /// Sum an iterator of GBP values.
    ///
    /// ```
    /// # use tax_calculator::Gbp;
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// # fn try_main() -> Result<(), tax_calculator::gbp::ParseError> {
    /// let amounts = vec![
    ///     Gbp::from_pounds(5),
    ///     "0.63".parse()?,
    /// ];
    /// let total: Gbp = amounts.iter().sum();
    /// assert_eq!(
    ///     format!("{}", total),
    ///     "£5.63"
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Gbp>,
    {
        Gbp(iter.map(|gbp| gbp.0).sum())
    }
}
