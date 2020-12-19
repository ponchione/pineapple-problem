use std::fs::File;
use std::io::{BufReader, BufRead};
#[macro_use]
extern crate clap;
use clap::App;
use rusty_money::{money, Money};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let input = matches.value_of("INPUT").unwrap();

    println!("INFO! Using input file: {}", input);
    match matches.value_of("divisor") {
        Some(divisor) => {
            println!("INFO! Using divisor: {}", divisor)
        },
        _ => {
            println!("INFO! No divisor specified, defaulting to 3.");
        }
    }

    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {

        let _line = line.unwrap(); //to keep obj alive
        let data: Vec<&str> = _line.split(",").collect();
        // let owed = data[0];
        // let paid = data[1];

        let owed = money!(data[0], "USD");
        let paid = money!(data[1], "USD");
        // println!("Owed: {}, Paid: {}", owed, paid);
        let change = paid - owed;
        let cents = change.to_string()
            .replace("$", "")
            .replace(".", "")
            .parse::<i32>()
            .unwrap();

        println!("{}", cents);

    }

}

// fn divisible_3(change: String) -> bool {
//     let whole_change =
//     let digit_vec: Vec<>
// }
