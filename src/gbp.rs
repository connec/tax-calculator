//! Defines a struct that wraps a GBP amount.

use num_format::{Locale, ToFormattedString};
use std::fmt;
use std::num::ParseIntError;

/// The maximum amount of GBP that can be represented.
pub static MAX: Gbp = Gbp {
    pounds: std::u32::MAX,
    pence: 99,
};

/// Represents errors that can occur when parsing GBP.
///
/// This is a superset of the errors that can occur when parsing an integer with one additional
/// case: the decimal value may be invalid (too long or short).
#[derive(Debug)]
pub enum ParseError {
    InvalidInt(ParseIntError),
    InvalidDecimal,
}

impl From<ParseIntError> for ParseError {
    fn from(error: ParseIntError) -> Self {
        ParseError::InvalidInt(error)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidInt(error) => error.fmt(f),
            ParseError::InvalidDecimal => write!(f, "Invalid decimal for GBP value"),
        }
    }
}

/// Represents an amount of Great British Pounds, stored as pounds and pence.
///
/// This provides useful formatting and parsing methods and implements necssary arithemtic operators
/// for convenience.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Gbp {
    pounds: u32,
    pence: u8,
}

impl Gbp {
    /// Construct a `Gbp` from a number of pounds.
    ///
    /// ```
    /// # use tax_calculator::Gbp;
    /// let amount = Gbp::from_pounds(100);
    /// ```
    pub fn from_pounds(pounds: u32) -> Self {
        Gbp { pounds, pence: 0 }
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
        write!(
            f,
            "£{}.{:02}",
            self.pounds.to_formatted_string(&Locale::en),
            self.pence
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
            .parse::<u32>()?;
        let pence = pence.parse::<u8>()?;
        Ok(Gbp { pounds, pence })
    }
}

impl std::ops::Add<&Gbp> for Gbp {
    type Output = Self;

    /// Add a GBP amount to this one.
    ///
    /// ```
    /// # use tax_calculator::Gbp;
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// # fn try_main() -> Result<(), tax_calculator::gbp::ParseError> {
    /// let balance = Gbp::from_pounds(1000);
    /// let deposit: Gbp = "£437.83".parse()?;
    /// let updated_balance = balance + &deposit;
    /// assert_eq!(
    ///     format!("{}", updated_balance),
    ///     "£1,437.83"
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    fn add(self, rhs: &Gbp) -> Self {
        let (pounds, pence) = if self.pence + rhs.pence > 100 {
            (self.pounds + rhs.pounds + 1, self.pence + rhs.pence - 100)
        } else {
            (self.pounds + rhs.pounds, self.pence + rhs.pence)
        };
        Gbp { pounds, pence }
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
        let (pounds, pence) = if rhs.pence > self.pence {
            (self.pounds - rhs.pounds - 1, 100 + self.pence - rhs.pence)
        } else {
            (self.pounds - rhs.pounds, self.pence - rhs.pence)
        };
        Gbp { pounds, pence }
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
        if rhs.pence > self.pence {
            self.pounds -= rhs.pounds + 1;
            self.pence = 100 - self.pence - rhs.pence;
        } else {
            self.pounds -= rhs.pounds;
            self.pence -= rhs.pence;
        }
    }
}

impl std::ops::Mul<f64> for Gbp {
    type Output = Gbp;

    /// Multiply a GBP amount by a float.
    ///
    /// **Note:** This will round fractional pence, and truncate pound values that exceed
    /// `std::u32::MAX`.
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
    fn mul(self, factor: f64) -> Gbp {
        let amount = ((self.pounds as f64 * 100.0 + self.pence as f64) * factor).round();
        println!("{}.{} * {} = {}", self.pounds, self.pence, factor, amount);
        Gbp {
            pounds: (amount / 100.0) as u32,
            pence: (amount % 100.0) as u8,
        }
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
        iter.fold(Gbp::from_pounds(0), |sum, item| sum + item)
    }
}
