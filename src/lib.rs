mod gbp;
mod tax_bands;

use lazy_static::lazy_static;

use std::collections::BTreeMap;

pub use gbp::Gbp;
pub use tax_bands::TaxBands;

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
