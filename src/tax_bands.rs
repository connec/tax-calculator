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
pub struct TaxBands(Vec<(Gbp, f64)>);

impl TaxBands {
    /// Construct a `TaxBands` from a tax free allowance, top rate, and the thresholds and rates in
    /// between.
    ///
    /// This is a convenient way of constructing a `TaxBands` from the format they are more commonly
    /// described in - a list of thresholds and tax rates.
    pub fn new(tax_free_allowance: u32, top_rate: f64, thresholds: Vec<(u32, f64)>) -> TaxBands {
        let mut bands = Vec::with_capacity(thresholds.len() + 2);
        bands.push((Gbp::from_pounds(tax_free_allowance), 0.0));

        let mut prev_threshold = 0;
        for (threshold, rate) in thresholds.into_iter() {
            let modifier = min(prev_threshold, 1); // To match the specified output we need to fudge
                                                   // the bands after the first.
            bands.push((
                Gbp::from_pounds(threshold - prev_threshold - modifier),
                rate,
            ));
            prev_threshold = threshold;
        }

        bands.push((gbp::MAX, top_rate));
        TaxBands(bands)
    }

    /// Apply the tax bands to a given gross income and return the taxed amount, rate, and tax for
    /// each band.
    pub fn apply(&self, gross_income: Gbp) -> Vec<(Gbp, f64, Gbp)> {
        let mut untaxed_income = gross_income;
        let apply_band = |(affected_income, rate): &(Gbp, f64)| {
            let affected_income = min(*affected_income, untaxed_income);
            let tax = affected_income * rate;
            untaxed_income = untaxed_income - affected_income;

            (affected_income, *rate, tax)
        };

        self.0.iter().map(apply_band).collect()
    }
}
