use std::{env, str};

const CURRENCIES: [&str; 2] = ["usd", "eur"];

fn format_num(cstr: String) -> String {
    cstr
    .as_bytes()
    .rchunks(3)
    .rev()
    .map(str::from_utf8)
    .collect::<Result<Vec<&str>, _>>()
    .unwrap()
    .join(",")
}

fn calculate(sum: f64 , unit: &str) {
    if CURRENCIES.contains(&unit) {
        let output = sum*220f64;
        let cstr = format!("{}", output.round());
        println!("{} {} is {} dzd", sum, unit, format_num(cstr))
    } else if unit == "dzd" {
        let output = sum/220f64;
        let cstr = format!("{}", output.round());
        println!("{} {} is {} usd or eur", sum, unit, format_num(cstr))
    }
}


fn main() {
    let mut args = env::args();
    args.next().unwrap();

    let sum = args
        .next()
        .unwrap()
        .parse::<f64>()
        .expect("ERROR: couldn't convert");

    let binding = args
        .next()
        .unwrap();
    let unit = binding.as_str();

    calculate(sum, unit)
}
