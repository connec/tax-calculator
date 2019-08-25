//! Contains a minimal implementation of a data type for GBP amounts.
//!
//! See the documentation for [`Gbp`] for more usage details.
//!
//! [`Gbp`]: struct.Gbp.html

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
/// This provides useful `impl`s for idiomatic usage in [formatting], [parsing], and [arithmetic]
/// \(though only the operators required for the task have been implemented).
///
/// [formatting]: #impl-Display
/// [parsing]: #impl-FromStr
/// [arithmetic]: #impl-Sub<Gbp>
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Gbp {
    pounds: u32,
    pence: u8,
}

impl Gbp {
    /// Construct a `Gbp` from pounds and pence.
    ///
    /// # Panics
    ///
    /// This will panic if `pence` is over 99 and adding the overflow to `pounds` causes an
    /// overflow.
    ///
    /// ```
    /// # use tax_calculator::Gbp;
    /// let amount = Gbp::new(1, 99);
    pub fn new(pounds: u32, pence: u8) -> Self {
        Gbp {
            pounds: pounds + u32::from(pence / 100),
            pence: pence % 100,
        }
    }

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
    /// let amount = Gbp::new(1_234_567, 89);
    /// assert_eq!(
    ///     format!("{}", amount),
    ///     "£1,234,567.89"
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
    /// assert_eq!(amount, Gbp::new(1_234_567, 89));
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
    /// let balance = Gbp::new(735, 62);
    /// let deposit = Gbp::new(437, 83);
    /// let updated_balance = balance + &deposit;
    /// assert_eq!(updated_balance, Gbp::new(1173, 45));
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
    /// let balance = Gbp::new(1000, 0);
    /// let cost = Gbp::new(437, 83);
    /// let remaining = balance - cost;
    /// assert_eq!(remaining, Gbp::new(562, 17));
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
    /// let mut balance = Gbp::new(1000, 0);
    /// let cost = Gbp::new(437, 83);
    /// balance -= cost;
    /// assert_eq!(balance, Gbp::new(562, 17));
    /// ```
    #[allow(clippy::suspicious_op_assign_impl)]
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
    /// let cost = Gbp::new(0, 99);
    /// let total = cost * 5.0;
    /// assert_eq!(total, Gbp::new(4, 95));
    /// ```
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, factor: f64) -> Gbp {
        let amount = ((f64::from(self.pounds) * 100.0 + f64::from(self.pence)) * factor).round();
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
    /// let amounts = vec![
    ///     Gbp::new(5, 0),
    ///     Gbp::new(0, 63),
    /// ];
    /// let total: Gbp = amounts.iter().sum();
    /// assert_eq!(total, Gbp::new(5, 63));
    /// ```
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Gbp>,
    {
        iter.fold(Gbp::from_pounds(0), |sum, item| sum + item)
    }
}
