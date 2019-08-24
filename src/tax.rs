//! Defines structs and methods for working with tax.

use std::cmp::min;

use crate::gbp::{self, Gbp};

/// Struct representing a tax band.
#[derive(Debug)]
pub struct Band {
    name: String,
    affected_income: Gbp,
    rate: f64,
}

impl Band {
    /// Construct a new tax band from a name, affected income, and tax rate.
    pub fn new(name: String, affected_income: Gbp, rate: f64) -> Self {
        Band {
            name,
            affected_income,
            rate,
        }
    }

    /// Get the tax band's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Get the tax band's affected income.
    pub fn affected_income(&self) -> Gbp {
        self.affected_income
    }

    /// Get the tax band's tax rate.
    pub fn rate(&self) -> f64 {
        self.rate
    }

    pub fn apply(&self, amount: Gbp) -> (Gbp, Gbp) {
        let affected = min(self.affected_income, amount);
        (affected, affected * self.rate)
    }
}

/// Struct representing a year's tax schedule.
///
/// Tax schedules are represented as a `Vec` of 'affected income' and 'applicable tax rate'. This
/// format makes calculating tax very simple (see [`apply`]). This is slightly naive compared to
/// 'real life', since the personal allowance is gradually reduced for incomes above £100,000. This
/// has been ignored as it is not required for this project.
///
/// [`apply`]: #method.apply
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
