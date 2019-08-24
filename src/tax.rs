//! Defines structs and methods for working with tax.

use std::cmp::min;

use crate::gbp::{self, Gbp};

/// Struct representing a tax band.
///
/// A tax band has a name, an amount of affected income, and a rate. When applied to an amount of
/// income, only the income up to the affected amount will have tax computed.
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
/// Tax schedules are represented as a `Vec` of 'affected income' and 'applicable tax rate'. This
/// format makes calculating tax very simple (see [`apply`]). This is slightly naive compared to
/// 'real life', since the tax free allowance is gradually reduced for incomes above £100,000. This
/// has been ignored as it is not required for this project.
///
/// [`apply`]: #method.apply
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
    /// # use tax_calculator::tax;
    /// let tax_year_2017 = tax::Schedule::new(
    ///     11500,
    ///     ("Higher rate", 0.4),
    ///     vec![("Basic rate", 31500, 0.2)]
    /// );
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
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// # fn try_main() -> Result<(), tax_calculator::gbp::ParseError> {
    /// use tax_calculator::{tax, Gbp, SCHEDULES};
    /// let tax_year_2018 = SCHEDULES.get(&2018).unwrap();
    /// let income = Gbp::from_pounds(43500);
    /// assert_eq!(
    ///     tax_year_2018.apply(income).iter().map(|(b, a, t)| (b.name(), a, t)).collect::<Vec<_>>(),
    ///     vec![
    ///         ("Starter rate", &"£2000".parse()?, &"£380".parse()?),
    ///         ("Basic rate", &"£10,149".parse()?, &"2,029.80".parse()?),
    ///         ("Intermediate rate", &"£19,429".parse()?, &"£4,080.09".parse()?),
    ///         ("Higher rate", &"£72".parse()?, &"£28.80".parse()?),
    ///         ("Top rate", &"0".parse()?, &"0".parse()?),
    ///     ],
    /// );
    /// #     Ok(())
    /// # }
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
