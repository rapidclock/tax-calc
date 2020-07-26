extern crate serde;

use serde::Deserialize;

#[derive(Debug)]
struct Bracket {
    tax_rate: f64,
    cap: Option<f64>,
    pub quantity: f64,
    pub tax_amount: Option<f64>,
}

impl Bracket {
    pub fn new(rate: f64, cap: Option<f64>) -> Self {
        Bracket {
            tax_rate: rate,
            cap,
            quantity: 0f64,
            tax_amount: None,
        }
    }

    pub fn calculate_tax(&mut self) {
        self.tax_amount = Some(self.quantity * (self.tax_rate / 100f64));
    }

    pub fn take_chunk(&mut self, offered_value: f64) -> f64 {
        if let Some(limit) = self.cap {
            if offered_value > limit {
                self.quantity = limit;
                return offered_value - limit;
            }
        }
        self.quantity = offered_value;
        0f64
    }
}

#[derive(Debug)]
pub struct TaxCalculator {
    brackets: Vec<Bracket>,
    income: f64,
    total_tax: Option<f64>,
}


impl TaxCalculator {
    pub fn new_from_data_scheme(ds: &DataScheme, income: f64) -> Self {
        let mut brackets: Vec<Bracket> = vec![];
        let deduction_val = ds.deduction;
        let deduction_bracket = Bracket::new(0f64, Some(deduction_val as f64));
        brackets.push(deduction_bracket);
        for i in 0..ds.brackets.len() {
            let rb_op = ds.brackets.get(i);
            if let Some(rb) = rb_op {
                let rate = rb.rate as f64;
                let mut cap: Option<f64> = None;
                if let Some(rb_next) = ds.brackets.get(i + 1) {
                    cap = Some(rb_next.base as f64);
                }
                let bracket = Bracket::new(rate, cap);
                brackets.push(bracket);
            }
        }
        TaxCalculator {
            brackets,
            income,
            total_tax: None,
        }
    }

    pub fn calculate_brackets(&mut self) {
        let mut init = self.income;
        for bracket in &mut self.brackets {
            init = bracket.take_chunk(init);
            bracket.calculate_tax();
            if let Some(val) = bracket.tax_amount {
                if let Some(current_total) = self.total_tax {
                    self.total_tax = Some(current_total + val);
                } else {
                    self.total_tax = Some(val);
                }
            }
            if init == 0f64 {
                break;
            }
        }
    }

    pub fn print_analysis(&self) {
        println!("Tax Analysis for the income : {}", &self.income);
        for bracket in &self.brackets {
            if let Some(amount) = bracket.tax_amount {
                let cap_val = if let Some(val) = bracket.cap {
                    format!("{}", val)
                } else {
                    String::from("To infinity...")
                };
                println!("Rate: {rate}, Cap: {cap}, Bracket Fill: {limit}, Tax Amount: {tax}",
                         rate = bracket.tax_rate, limit = bracket.quantity,
                         tax = amount, cap = cap_val);
            }
        }
        let total_tax_paid = self.total_tax.unwrap();
        print!("Total Tax : {}, ", total_tax_paid);
        println!("Effective Tax Rate: {}", (total_tax_paid / self.income) * 100f64);
    }
}

#[derive(Deserialize, Debug)]
pub struct DataScheme {
    pub deduction: i64,
    pub brackets: Vec<RateBase>,
}

#[derive(Deserialize, Debug)]
pub struct RateBase {
    pub rate: i64,
    pub base: i64,
}