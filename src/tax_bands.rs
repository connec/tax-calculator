use std::cmp::min;

use crate::gbp::{self, Gbp};

/// Struct representing a year's tax bands.
///
/// Tax bands are represented as a `Vec` of 'affected income' and 'applicable tax rate'. This format
/// makes calculating tax very simple (see [`apply`]). This is slightly naive compared to
/// 'real life', since the personal allowance is gradually reduced for incomes above £100,000. This
/// has been ignored as it is not required for this project.
///
/// `f64` is used for the tax bands. This isn't perfect for precision, so we might want to swap this
/// out for a precise decimal type.
///
/// [`apply`]: #method.apply
#[derive(Debug)]
pub struct TaxBands {
    tax_free_allowance: Gbp,
    bands: Vec<(String, Gbp, f64)>,
}

impl TaxBands {
    /// Construct a `TaxBands` from a tax free allowance, top rate, and the thresholds and rates in
    /// between.
    ///
    /// This is a convenient way of constructing a `TaxBands` from the format they are more commonly
    /// described in - a list of thresholds and tax rates.
    pub fn new(
        tax_free_allowance: u32,
        top_rate: (&str, f64),
        thresholds: Vec<(&str, u32, f64)>,
    ) -> TaxBands {
        let mut bands = Vec::with_capacity(thresholds.len() + 2);

        let mut prev_threshold = 0;
        for (name, threshold, rate) in thresholds.into_iter() {
            let modifier = min(prev_threshold, 1); // To match the specified output we need to fudge
                                                   // the bands after the first.
            bands.push((
                name.to_string(),
                Gbp::from_pounds(threshold - prev_threshold - modifier),
                rate,
            ));
            prev_threshold = threshold;
        }

        bands.push((top_rate.0.to_string(), gbp::MAX, top_rate.1));
        TaxBands {
            tax_free_allowance: Gbp::from_pounds(tax_free_allowance),
            bands,
        }
    }

    pub fn tax_free_allowance(&self) -> Gbp {
        self.tax_free_allowance
    }

    /// Apply the tax bands to a given gross income and return the taxed amount, rate, and tax for
    /// each band.
    pub fn apply<'a>(&'a self, gross_income: Gbp) -> Vec<(&'a str, Gbp, f64, Gbp)> {
        let mut untaxed_income = gross_income - self.tax_free_allowance;
        let apply_band = |(name, affected_income, rate): &'a (String, Gbp, f64)| {
            let affected_income = min(*affected_income, untaxed_income);
            let tax = affected_income * rate;
            untaxed_income = untaxed_income - affected_income;

            (name.as_str(), affected_income, *rate, tax)
        };

        self.bands.iter().map(apply_band).collect()
    }
}
