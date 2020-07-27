extern crate clap;
extern crate serde_json;

use std::collections::HashMap;
use std::fs;

use clap::{App, Arg, ArgGroup, crate_authors, crate_description, crate_version};

use tax_calc::{DataScheme, TaxCalculator};

fn main() {
    let matches = App::new("Tax Calculator")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("config")
            .long("cfg")
            .short("c")
            .value_name("CFG")
            .help("The file that contains the tax bracket definitions")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("single")
            .short("s")
            .help("Indicates Tax Calculation as a Single Individuals"))
        .arg(Arg::with_name("married")
            .short("m")
            .help("Indicates Tax Calculation as a Married Couple"))
        .arg(Arg::with_name("head")
            .short("o")
            .help("Indicates Tax Calculation as the head of household"))
        .arg(Arg::with_name("income")
            .short("i")
            .long("income")
            .value_name("INCOME")
            .validator(income_validator)
            .takes_value(true)
            .required(true))
        .group(ArgGroup::with_name("type")
            .required(true)
            .arg("single")
            .arg("married")
            .arg("head"))
        .get_matches();
    let income: f64 = matches.value_of("income").unwrap().parse().unwrap();
    let cfg_path = matches.value_of("config").unwrap();
    let (single, married, head) = (
        matches.is_present("single"),
        matches.is_present("married"),
        matches.is_present("head"),
    );
    match (single, married, head) {
        (true, _, _) => process_file(cfg_path, "single", income),
        (_, true, _) => process_file(cfg_path, "married", income),
        (_, _, true) => process_file(cfg_path, "head", income),
        _ => unreachable!()
    };
}

fn income_validator(income: String) -> Result<(), String> {
    if let Ok(_) = income.parse::<f64>() {
        Ok(())
    } else {
        Err(String::from("Income is invalid. Should be valid float."))
    }
}

fn process_file(file_path: &str, category: &str, income: f64) {
    let data = fs::read_to_string(file_path);
    if let Ok(data) = data {
        // println!("{}", data);
        let parsed_data = serde_json::from_str::<HashMap<&str, DataScheme>>(data.as_str()).unwrap();
        let mut tax_calculator = TaxCalculator::new_from_data_scheme(&parsed_data[category], income);
        tax_calculator.calculate_brackets();
        tax_calculator.print_analysis();
    }
}

