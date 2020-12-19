use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};
#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let input = matches.value_of("INPUT").unwrap();

    println!("Using input file: {}", input);
    match matches.value_of("divisor") {
        Some(divisor) => {
            println!("Using divisor: {}", divisor)
        },
        _ => {
            println!("No divisor specified, defaulting to 3.");
        }
    }

    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {

        let _line = line.unwrap();
        let data: Vec<&str> = _line.split(",").collect();
        let owed = data[0];
        let paid = data[1];

        println!("Owed: {}, Paid: {}", owed, paid);
    }



}

