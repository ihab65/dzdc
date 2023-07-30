use std::env;

const CURRENCIES: [&str; 2] = ["usd", "eur"];

fn print_as_word(number: String) {
    let mut leading_numbers = String::new();
    let mut output_str = String::new();
    match number.len() {

            num if num == 4 => {
                for (i, c) in number.chars().enumerate() {
                    if i != 3 {    
                        leading_numbers.push(c);
                    }
                    output_str = format!("{leading_numbers} alf")
                }
            },

            num if num == 5 => {
                for (i, c) in number.chars().enumerate() {
                    if i == 0 && c == '1'{
                        output_str.push_str("million o ")
                    } else if i == 0 {
                        output_str.push_str(format!("{} millions o ", c).as_str())
                    } else if i != 0 && i != 4 {
                        output_str.push(c)
                    }
                }
            },

            num if num == 6 => {
                
                for (i, c) in number.chars().enumerate() {
                    if i == 0 {
                        output_str.push(c)
                    } else if i == 1 {
                        output_str.push(c);
                        output_str.push_str(" millions o ")
                    } else if i!= 0 && i != 1 && i != 5 {
                        output_str.push(c)
                    }
                }
            },

            num if num == 7 => {
                
                for (i, c) in number.chars().enumerate() {
                    if i == 0 {
                        output_str.push(c)
                    } else if i == 1 {
                        output_str.push(c)
                    } else if i == 2 {
                        output_str.push(c);
                        output_str.push_str(" millions o ")
                    } else if i!= 0 && i != 1 && i != 2 && i != 6 {
                        output_str.push(c)
                    }
                }
            },

            num if num == 8 => {
                for (i, c) in number.chars().enumerate() {
                    if i == 0 && c == '1' {
                        output_str.push_str(" milliard o ")
                    } else if i == 0 && c != '1' {
                        output_str.push(c);
                        output_str.push_str(" milliards o ")
                    } else if i == 1 {
                        output_str.push(c)
                    } else if i == 2 {
                        output_str.push(c)
                    } else if i == 3 {
                        output_str.push(c);
                        output_str.push_str(" millions")
                    }
                }
            }

            _ => (),

        } // end of match
        if output_str.ends_with("000") {
            output_str = output_str.trim_end_matches("o 000").to_string();
        }
        println!("{}", output_str)

    }
    

fn format_num(cstr: String) -> String {
    cstr
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",")
}

fn calculate(sum: f64 , unit: &str) {

    if CURRENCIES.contains(&unit) {
        let output = sum*220f64;
        let cstr = format!("{}", output.round());
        println!("");
        print!("{} {} is {} dzd -> ", sum, unit, format_num(cstr.clone()));
        print_as_word(cstr)

    } else if unit == "dzd" {
        let output = sum/220f64;
        let cstr = format!("{}", output.round());
        println!("");
        print!("{} {} is {} usd or eur => ", sum, unit, format_num(cstr))
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
