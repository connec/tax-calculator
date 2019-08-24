# tax-calculator

A little CLI to calculate income tax.

**Note:** This is a work-in-progress. Likely next steps:

- Better serialization and deserialization of currency (e.g. formatting and argument parsing).
- Name bands and print them.

[clap]: https://clap.rs/

## Setup

To use the CLI you need to build it from source. To do so you need a [Rust] toolchain. The easiest
way to get one is to use [`rustup`].

Once rust is installed you can run the CLI with [Cargo]:

```
$ cargo run
   ...
tax-calculator 0.1.0
Chris Connelly <chris@connec.co.uk>

USAGE:
    tax-calculator <year> <gross_income>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <year>
    <gross_income>
```

[Rust]: https://www.rust-lang.org/
[`rustup`]: https://rustup.rs
[Cargo]: https://doc.rust-lang.org/cargo/

## Usage

The CLI requires two positional arguments:

1. A tax year, specified as the starting year (e.g. `2018` would represent the 2018-2019 tax year).
   Tax bands are only defined for years 2018, 2017, 2016, and 2015.
2. A gross income, specified as a plain unformatted number (e.g. `43500` would represent
   £43,500.00).

Positional arguments can be given to `cargo run` after a `--`, for example:

```
$ cargo run -- 2018 43500
    ...

Tax Year: 2018-2019
Gross Salary: 43500

Total Tax Due: 6518.69
```
