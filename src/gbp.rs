use itertools::Itertools;
use std::fmt;
use std::num::ParseIntError;

pub static MAX: Gbp = Gbp(std::u32::MAX);

/// Represents an amount of Great British Pounds.
///
/// This provides useful formatting and parsing methods and implements arithemtic operators for
/// convenience.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Gbp(u32);

impl Gbp {
    pub fn from_pounds(pounds: u32) -> Self {
        Gbp(pounds * 100)
    }
}

impl fmt::Display for Gbp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = self.0.to_string();
        let (pounds, pence) = if string.len() > 2 {
            string.split_at(string.len() - 2)
        } else {
            ("", string.as_str())
        };
        let pounds = pounds
            .as_bytes()
            .rchunks(3)
            .rev()
            .intersperse(&[b','])
            .flatten()
            .map(|c| char::from(*c))
            .collect::<String>();
        write!(f, "£{}.{}", pounds, pence)
    }
}

impl std::str::FromStr for Gbp {
    type Err = ParseIntError;

    /// This is a slightly naive approach but it keeps things simple: we strip leading "£", and any
    /// ",", then parse it as a u32.
    fn from_str(mut string: &str) -> Result<Self, ParseIntError> {
        if string.starts_with("£") {
            string = &string["£".len()..];
        }
        let string = string.chars().filter(|c| *c != ',').collect::<String>();
        Ok(Gbp(100 * string.parse::<u32>()?))
    }
}

impl std::ops::Sub for Gbp {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Gbp(self.0 - rhs.0)
    }
}

impl std::ops::SubAssign<&Gbp> for Gbp {
    fn sub_assign(&mut self, rhs: &Gbp) {
        self.0 -= rhs.0
    }
}

impl std::ops::Mul<&f64> for Gbp {
    type Output = Gbp;

    fn mul(self, other: &f64) -> Gbp {
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
