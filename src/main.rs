use structopt::{clap, StructOpt};

use tax_calculator::{Gbp, TaxBands, BANDS};

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ArgRequiredElseHelp"))]
struct Opt {
    #[structopt(help = "Start of a tax year")]
    year: u32,

    #[structopt(help = "Gross income to calculate tax for")]
    gross_income: Gbp,
}

impl Opt {
    fn bands(&self) -> Result<&TaxBands, clap::Error> {
        BANDS.get(&self.year).ok_or_else(|| {
            let available_years = BANDS
                .keys()
                .map(|y| y.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            let description = &format!(
                "Tax bands are not defined for year: {}. Available years: {}.",
                self.year, available_years
            );
            clap::Error::with_description(description, clap::ErrorKind::InvalidValue)
        })
    }
}

fn main() {
    let opt = Opt::from_args();

    let bands = opt.bands().unwrap_or_else(|error| error.exit());

    let tax = bands.apply(opt.gross_income);
    println!(
        "
Tax Year: {}-{}
Gross Salary: {}

Total Tax Due: {}
",
        opt.year,
        opt.year + 1,
        opt.gross_income,
        tax.iter().map(|(_, _, tax)| tax).sum::<Gbp>()
    );
}
