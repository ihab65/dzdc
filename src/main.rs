use std::{env::{self, Args}, process::exit};

#[derive(PartialEq, Debug)]
struct Currency {
    name: String,
    value: f32,
}

impl Currency {
    const fn new(code: String, price: f32) -> Self {
        Currency {
            name: code,
            value: price,
        }
    }
}

fn entry(currencies: Vec<Currency>) -> Result<(), ()> {
    let mut args = env::args();
    let program = args.next().expect("path to program is provided");
    let subcommand = args.next().ok_or_else(|| {
        eprintln!("ERROR: no subcommand is provided");
        eprintln!("Usage: {program} convert <SUM> <CURRENCY>");
        exit(1)
    })?;

    match subcommand.as_str() {
        "convert" => {
            as_word(calculate(args, currencies) as u32);
            Ok(())
        }
        // "add" => {
        //     let unit = args
        //         .next()
        //         .expect("there is no arg to parse")
        //         .to_uppercase();
            
        //     let sum = args
        //         .next()
        //         .expect("there is no arg to parse")
        //         .parse::<f32>()
        //         .map_err(|err| {
        //             eprintln!("ERROR: Failed parsing the sum argument: {}", err);
        //         })?;


        //     let value = Currency::new(unit, sum);
        //     currencies.push(value);
        //     for elem in currencies.iter() {
        //         println!("{:?}", elem)
        //     }
        //     Ok(())
        // }
        _ => {
            // TODO: add a usage fn
            Err(())
        }
    }
}

fn main() {
    let currencies: Vec<Currency> = vec![
        Currency::new(String::from("USD"), 210f32),
        Currency::new(String::from("EUR"), 220f32),
        Currency::new(String::from("GBP"), 250f32),
    ];
    entry(currencies).unwrap();
}

fn calculate(mut args: Args, currencies: Vec<Currency>) -> f32 {
    let sum = match args.next().expect("there is no arg to parse").parse::<f32>() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("ERROR: Failed parsing the sum argument: {}", err);
            exit(1)
        }
    };

    let unit = args
        .next()
        .expect("there is no arg to parse")
        .to_uppercase();
    let currency = match currencies.iter().find(|&c| c.name == unit) {
        Some(currency) => currency,
        None => {
            eprintln!("ERROR: Unknown unit: {}", unit);
            eprintln!("INFO:  you can use USD, EUR or GBP as units");
            exit(1)
        }
    };
    sum * currency.value
}

fn format_num(cstr: String) -> String {
    cstr.as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",")
}

fn as_word(mut amount: u32) {
    let mut values = [0u32; 4];
    let units: [&str; 3] = ["mlyar", "million", "alf"];
    let divs: [u32; 3] = [10_000_000, 10_000, 10];

    let amount_str = format!("{}", amount);
    print!("{} dzd ", format_num(amount_str));

    for (i, &divisor) in divs.iter().enumerate() {
        values[i] = amount / divisor;
        amount %= divisor;
    }

    print!(">> ");
    for (i, &value) in values.iter().enumerate() {
        if value != 0 {
            print!("{} {} ", value, units[i]);
        }
    }
    println!()
}
