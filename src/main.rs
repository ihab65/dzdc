use std::env;

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

    let sum = args[1]
        .parse::<f32>()
        .map_err(|err| eprintln!("ERROR: failed parsing the sum argument: {err}"))
        .unwrap();

    let unit = &args[2];
    let currency = CURRENCIES.iter().find(|&c| c.name == unit.to_uppercase()).unwrap();

    as_word(calculate(sum, currency) as u32)
}

fn calculate(sum: f32, unit: &Currency<'_>) -> f32 {
    sum * unit.value
}

fn as_word(mut amount: u32) {
    let b = amount / 10_000_000;
    amount = amount % 10_000_000;
    let m = amount / 10_000;
    amount = amount % 10_000;
    let k = (amount / 1_000) * 100;
    let c = amount % 1_000;

    if b != 0 {
        print!(" {}mlyar", b);
    }
    if m != 0 {
        print!(" {}mlyoun", m);
    }
    if k != 0 {
        print!(" {}alf", k);
    }
    if c != 0 {
        print!(" {}", c);
    }
    println!(" dzd");
}