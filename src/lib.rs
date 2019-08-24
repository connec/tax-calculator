use lazy_static::lazy_static;

use std::cmp::min;
use std::collections::BTreeMap;

lazy_static! {
  /// Tax bands for each covered year.
  ///
  /// These are stored in a `BTreeMap` for convenient, ordered iteration of available years.
  pub static ref BANDS: BTreeMap<u32, TaxBands> = {
    let mut m = BTreeMap::new();

    m.insert(2015, TaxBands::new(10600, 0.4, vec![
      (31785, 0.2)
    ]));
    m.insert(2016, TaxBands::new(11000, 0.4, vec![
      (32000, 0.2)
    ]));
    m.insert(2017, TaxBands::new(11500, 0.4, vec![
      (31500, 0.2)
    ]));
    m.insert(2018, TaxBands::new(11850, 0.46, vec![
      (2000, 0.19),
      (12150, 0.2),
      (31580, 0.21),
      (150000, 0.4)
    ]));

    m
  };
}

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
pub struct TaxBands(Vec<(u32, f64)>);

impl TaxBands {
    /// Construct a `TaxBands` from a tax free allowance, top rate, and the thresholds and rates in
    /// between.
    ///
    /// This is a convenient way of constructing a `TaxBands` from the format they are more commonly
    /// described in - a list of thresholds and tax rates.
    pub fn new(tax_free_allowance: u32, top_rate: f64, thresholds: Vec<(u32, f64)>) -> TaxBands {
        let mut bands = Vec::with_capacity(thresholds.len() + 2);
        bands.push((tax_free_allowance, 0.0));

        let mut prev_threshold = 0;
        for (threshold, rate) in thresholds.into_iter() {
            let modifier = min(prev_threshold, 1); // To match the specified output we need to fudge
                                                   // the bands after the first.
            bands.push((threshold - prev_threshold - modifier, rate));
            prev_threshold = threshold;
        }

        bands.push((std::u32::MAX, top_rate));
        TaxBands(bands)
    }

    /// Apply the tax bands to a given gross income and return the taxed amount, rate, and tax for
    /// each band.
    pub fn apply(&self, gross_income: u32) -> Vec<(u32, f64, f64)> {
        let mut untaxed_income = gross_income;
        let apply_band = |(affected_income, rate): &(u32, f64)| {
            let affected_income = min(*affected_income, untaxed_income);
            let tax = (affected_income as f64) * rate;
            untaxed_income -= affected_income;

            (affected_income, *rate, tax)
        };

        self.0.iter().map(apply_band).collect()
    }
}
