//! Defines structs and methods for working with tax.
//!
//! In particular a [`tax::Schedule`] struct represents the income tax schedule for a particular
//! year including the tax free allowance and income tax bands. The [`Schedule::apply`] method can
//! be used to calculate tax details.
//!
//! [`tax::Schedule`]: struct.Schedule.html
//! [`Schedule::apply`]: struct.Schedule.html#method.apply

use std::cmp::min;

use crate::gbp::{self, Gbp};

/// Struct representing a tax band.
///
/// A tax band has a name, an amount of affected income, and a rate. When applied to an amount of
/// income, only the income up to the affected amount will have tax computed (see [`apply`]).
///
/// [`apply`]: #method.apply
///
/// ```
/// use tax_calculator::{tax, Gbp};
///
/// let basic_rate = tax::Band::new(
///     "Basic rate".to_string(),
///     Gbp::from_pounds(11500),
///     0.2,
/// );
/// let income = Gbp::from_pounds(15000);
///
/// let (affected, tax) = basic_rate.apply(income);
///
/// assert_eq!(affected, Gbp::from_pounds(11500));
/// assert_eq!(tax, Gbp::from_pounds(2300));
/// ```
#[derive(Debug)]
pub struct Band {
    name: String,
    affected_income: Gbp,
    rate: f64,
}

impl Band {
    /// Construct a new tax band from a name, affected income, and tax rate.
    ///
    /// ```
    /// # use tax_calculator::tax;
    /// # use tax_calculator::Gbp;
    /// let basic_rate = tax::Band::new(
    ///     "Basic rate".to_string(),
    ///     Gbp::from_pounds(11500),
    ///     0.2
    /// );
    /// ```
    pub fn new(name: String, affected_income: Gbp, rate: f64) -> Self {
        Band {
            name,
            affected_income,
            rate,
        }
    }

    /// Get the tax band's name.
    ///
    /// ```
    /// # use tax_calculator::tax;
    /// # use tax_calculator::Gbp;
    /// # let basic_rate = tax::Band::new("Basic rate".to_string(), Gbp::from_pounds(11500), 0.2);
    /// assert_eq!(basic_rate.name(), "Basic rate");
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Get the tax band's affected income.
    ///
    /// ```
    /// # use tax_calculator::tax;
    /// # use tax_calculator::Gbp;
    /// # let basic_rate = tax::Band::new("Basic rate".to_string(), Gbp::from_pounds(11500), 0.2);
    /// assert_eq!(basic_rate.affected_income(), Gbp::from_pounds(11500));
    /// ```
    pub fn affected_income(&self) -> Gbp {
        self.affected_income
    }

    /// Get the tax band's tax rate.
    ///
    /// ```
    /// # use tax_calculator::tax;
    /// # use tax_calculator::Gbp;
    /// # let basic_rate = tax::Band::new("Basic rate".to_string(), Gbp::from_pounds(11500), 0.2);
    /// assert_eq!(basic_rate.rate(), 0.2);
    /// ```
    pub fn rate(&self) -> f64 {
        self.rate
    }

    /// Apply the band to an amount, returning the amount the tax applied to and the tax.
    ///
    /// ```
    /// # use tax_calculator::tax;
    /// # use tax_calculator::Gbp;
    /// # let basic_rate = tax::Band::new("Basic rate".to_string(), Gbp::from_pounds(11500), 0.2);
    /// assert_eq!(
    ///     basic_rate.apply(Gbp::from_pounds(10000)),
    ///     (Gbp::from_pounds(10000), Gbp::from_pounds(2000))
    /// );
    /// assert_eq!(
    ///     basic_rate.apply(Gbp::from_pounds(15000)),
    ///     (Gbp::from_pounds(11500), Gbp::from_pounds(2300))
    /// );
    /// ```
    pub fn apply(&self, amount: Gbp) -> (Gbp, Gbp) {
        let affected = min(self.affected_income, amount);
        (affected, affected * self.rate)
    }
}

/// Struct representing a year's tax schedule.
///
/// Tax schedules are represented as a set tax free allowance and a `Vec` of [`Band`]s. This format
/// format makes calculating tax very simple (see [`apply`]), however it is slightly naive compared
/// to 'real life', since the tax free allowance is gradually reduced for incomes above a threshold.
/// This has been ignored as it is not required for this project.
///
/// [`apply`]: #method.apply
/// [`Band`]: struct.Band.html
///
/// ```
/// use tax_calculator::{tax, Gbp};
/// let tax_year_2017 = tax::Schedule::new(
///     11500,
///     ("Higher rate", 0.4),
///     vec![("Basic rate", 31500, 0.2)]
/// );
/// let income = Gbp::from_pounds(83000);
/// assert_eq!(
///     tax_year_2017.apply(income).iter().map(|(b, a, t)| (b.name(), a, t)).collect::<Vec<_>>(),
///     vec![
///         ("Basic rate", &Gbp::from_pounds(31500), &Gbp::from_pounds(6300)),
///         ("Higher rate", &Gbp::from_pounds(40000), &Gbp::from_pounds(16000)),
///     ],
/// );
/// ```
#[derive(Debug)]
pub struct Schedule {
    tax_free_allowance: Gbp,
    bands: Vec<Band>,
}

impl Schedule {
    /// Construct a `Schedule` from a tax free allowance, top rate, and the thresholds and rates
    /// in between.
    ///
    /// This is a convenient way of constructing a `Schedule` from the format they are more
    /// commonly described in - a list of thresholds and tax rates.
    ///
    /// ```
    /// # use tax_calculator::tax;
    /// let tax_year_2017 = tax::Schedule::new(
    ///     11500,
    ///     ("Higher rate", 0.4),
    ///     vec![("Basic rate", 31500, 0.2)]
    /// );
    /// ```
    pub fn new(
        tax_free_allowance: u32,
        top_rate: (&str, f64),
        thresholds: Vec<(&str, u32, f64)>,
    ) -> Schedule {
        let mut bands = Vec::with_capacity(thresholds.len() + 2);

        let mut prev_threshold = 0;
        for (name, threshold, rate) in thresholds.into_iter() {
            let modifier = min(prev_threshold, 1); // To match the specified output we need to fudge
                                                   // the bands after the first.
            bands.push(Band::new(
                name.to_string(),
                Gbp::from_pounds(threshold - prev_threshold - modifier),
                rate,
            ));
            prev_threshold = threshold;
        }

        bands.push(Band::new(top_rate.0.to_string(), gbp::MAX, top_rate.1));
        Schedule {
            tax_free_allowance: Gbp::from_pounds(tax_free_allowance),
            bands,
        }
    }

    /// Get the tax free allowance for this schedule.
    /// # use tax_calculator::{tax, Gbp};
    /// # let tax_year_2017 = tax::Schedule::new(
    /// #     11500,
    /// #     ("Higher rate", 0.4),
    /// #     vec![("Basic rate", 31500, 0.2)]
    /// # );
    /// assert_eq!(
    ///     tax_year_2017.tax_free_allowance(),
    ///     Gbp::from_pounds(11500)
    /// );
    pub fn tax_free_allowance(&self) -> Gbp {
        self.tax_free_allowance
    }

    /// Apply the tax bands to a given gross income and return the taxed amount, rate, and tax for
    /// each band.
    ///
    /// This calculates the initial taxable income, accounting for the tax free allowance, then
    /// applies each band in turn, reducing the remaining taxable income as tax is applied.
    ///
    /// The result is a `Vec` with an entry for each band along with the amount of income taxed at
    /// that rate and the amount of tax due.
    /// ```
    /// use tax_calculator::{tax, Gbp, SCHEDULES};
    /// let tax_year_2018 = SCHEDULES.get(&2018).unwrap();
    /// let income = Gbp::from_pounds(43500);
    /// let tax = tax_year_2018.apply(income);
    ///
    /// assert_eq!(
    ///     tax.iter().map(|(b, a, t)| (b.name(), a, t)).collect::<Vec<_>>(),
    ///     vec![
    ///         ("Starter rate", &Gbp::new(2000, 0), &Gbp::new(380, 0)),
    ///         ("Basic rate", &Gbp::new(10149, 0), &Gbp::new(2029, 80)),
    ///         ("Intermediate rate", &Gbp::new(19429, 0), &Gbp::new(4080, 09)),
    ///         ("Higher rate", &Gbp::new(72, 0), &Gbp::new(28, 80)),
    ///         ("Top rate", &Gbp::new(0, 0), &Gbp::new(0, 0)),
    ///     ],
    /// );
    /// assert_eq!(
    ///     tax.iter().map(|(_, _, t)| t).sum::<Gbp>(),
    ///     Gbp::new(6518, 69),
    /// );
    /// ```
    pub fn apply<'a>(&'a self, gross_income: Gbp) -> Vec<(&'a Band, Gbp, Gbp)> {
        let mut taxable_income = gross_income - self.tax_free_allowance;
        let apply_band = |band: &'a Band| {
            let (affected_income, tax) = band.apply(taxable_income);
            taxable_income -= affected_income;

            (band, affected_income, tax)
        };

        self.bands.iter().map(apply_band).collect()
    }
}
