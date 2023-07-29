use std::env;

const CURRENCIES: [&str; 2] = ["usd", "eur"];

fn print_as_word(number: String) {
    let mut leading_numbers = String::new();
    let mut output_str = String::new();
    let _temp_str = String::new();
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
                        output_str.push_str(format!("{} melyan o ", c).as_str())
                    } else if i != 0 && i != 4 {
                        output_str.push(c)
                    }
                }
            },

            num if num == 6 => {

            }

            _ => (),

        } // end of match

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
        print!("{} {} is {} dzd -> ", sum, unit, format_num(cstr.clone()));
        print_as_word(cstr)

    } else if unit == "dzd" {
        let output = sum/220f64;
        let cstr = format!("{}", output.round());
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
