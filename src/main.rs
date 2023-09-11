use std::{env, process::exit};

#[derive(PartialEq)]
struct Currency<'a> {
    name: &'a str,
    value: f32,
}

impl<'a> Currency<'a> {
    const fn new(code: &'a str, price: f32) -> Self {
        Currency { name: code, value: price }
    }
}

const CURRENCIES: [Currency; 3] = [
    Currency::new("USD", 210f32),
    Currency::new("EUR", 220f32),
    Currency::new("GBP", 250f32)
];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: dzdc <SUM> <CURRENCY>");
    }

    let sum = match args[1].parse::<f32>() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("ERROR: Failed parsing the sum argument: {}", err);
            exit(1)
        }
    };

    let unit = &args[2].to_uppercase();
    let currency = match CURRENCIES.iter().find(|&c| c.name == unit) {
        Some(currency) => {
            currency
        }
        None => {
            eprintln!("ERROR: Unknown unit: {}", unit);
            eprintln!("INFO:  you can use USD, EUR or GBP as units");
            exit(1)
        }
    };

    as_word(calculate(sum, currency) as u32)
}


fn calculate(sum: f32, unit: &Currency<'_>) -> f32 {
    sum * unit.value
}

fn as_word(mut amount: u32) {
    let mut values = [0u32; 4];
    let units: [&str; 3] = ["mlyar", "million", "alf"];
    let divs: [u32; 3] = [10_000_000, 10_000, 10];

    for (i, &divisor) in divs.iter().enumerate() {
        values[i] = amount / divisor;
        amount %= divisor;
    }

    for (i, &value) in values.iter().enumerate() {
        if value != 0 {
            print!("{} {} ", value, units[i]);
        }
    }
    println!("dzd")
}