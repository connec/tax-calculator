use std::env::{self, Args};
use std::process;

use tax_calculator::BANDS;

fn main() {
    let mut args = env::args();
    args.next(); // Skip over the first arg, which is the executbale path

    let year = arg_u32(&mut args, "year");
    let gross_income = arg_u32(&mut args, "gross income");

    let bands = BANDS
        .get(&year)
        .unwrap_or_else(|| error(&format!("No tax bands defined for year: {}", year)));

    let tax = bands.apply(gross_income);
    println!(
        "
Tax Year: {}-{}
Gross Salary: {}

Total Tax Due: {}
",
        year,
        year + 1,
        gross_income,
        tax.iter().map(|(_, _, tax)| tax).sum::<f64>()
    );
}

fn arg_u32(args: &mut Args, name: &str) -> u32 {
    match args.next() {
        Some(arg) => arg.parse().unwrap_or_else(|_| {
            error(&format!("Argument {} is not a valid number: {}", name, arg))
        }),
        None => error(&format!("Missing required positional argument: {}", name)),
    }
}

fn error(message: &str) -> ! {
    eprintln!("{}", message);
    process::exit(1);
}
