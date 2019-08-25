use structopt::{clap, StructOpt};

use tax_calculator::{tax, Gbp, SCHEDULES};

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ArgRequiredElseHelp"))]
struct Opt {
    #[structopt(help = "Start of a tax year")]
    year: u32,

    #[structopt(help = "Gross income to calculate tax for")]
    gross_income: Gbp,
}

impl Opt {
    /// Retrieve the schedule associated with the chosen year. Returns a `clap::Error` if no
    /// schedule is defined for the year.
    fn schedule(&self) -> Result<&tax::Schedule, clap::Error> {
        if let Some(schedule) = SCHEDULES.get(&self.year) {
            return Ok(schedule);
        }

        let available_years = SCHEDULES
            .keys()
            .map(|y| y.to_string())
            .rev()
            .collect::<Vec<_>>()
            .join(", ");
        let description = &format!(
            "Tax schedule is not defined for year: {}. Available years: {}.",
            self.year, available_years
        );
        Err(clap::Error::with_description(
            description,
            clap::ErrorKind::InvalidValue,
        ))
    }
}

fn main() {
    let opt = Opt::from_args();

    let schedule = opt.schedule().unwrap_or_else(|error| error.exit());

    let tax = schedule.apply(opt.gross_income);
    println!(
        "
Tax Year: {}-{}
Gross Salary: {}

Personal Allowance: {}

Taxable Income: {}

{}
Total Tax Due: {}
",
        opt.year,
        opt.year + 1,
        opt.gross_income,
        schedule.tax_free_allowance(),
        opt.gross_income - schedule.tax_free_allowance(),
        format_tax(&tax),
        tax.iter().map(|(_, _, tax)| tax).sum::<Gbp>()
    );
}

fn format_tax(tax: &[(&tax::Band, Gbp, Gbp)]) -> String {
    let mut s = String::new();
    for (band, affected_income, tax) in tax {
        if affected_income == &Gbp::from_pounds(0) {
            continue;
        }
        s.push_str(&format!(
            "{}: {} @ {} = {}\n",
            band.name(),
            affected_income,
            band.rate(),
            tax
        ));
    }
    s
}
