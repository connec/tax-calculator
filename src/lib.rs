mod gbp;
pub mod tax;

use lazy_static::lazy_static;

use std::collections::BTreeMap;

pub use gbp::Gbp;

lazy_static! {
  /// Tax bands for each covered year.
  ///
  /// These are stored in a `BTreeMap` for convenient, ordered iteration of available years.
  pub static ref SCHEDULES: BTreeMap<u32, tax::Schedule> = {
    let mut m = BTreeMap::new();

    m.insert(2015, tax::Schedule::new(10600, ("Higher rate", 0.4), vec![
      ("Basic rate", 31785, 0.2)
    ]));
    m.insert(2016, tax::Schedule::new(11000, ("Higher rate", 0.4), vec![
      ("Basic rate", 32000, 0.2)
    ]));
    m.insert(2017, tax::Schedule::new(11500, ("Higher rate", 0.4), vec![
      ("Basic rate", 31500, 0.2)
    ]));
    m.insert(2018, tax::Schedule::new(11850, ("Top rate", 0.46), vec![
      ("Starter rate", 2000, 0.19),
      ("Basic rate", 12150, 0.2),
      ("Intermediate rate", 31580, 0.21),
      ("Higher rate", 150000, 0.4)
    ]));

    m
  };
}
