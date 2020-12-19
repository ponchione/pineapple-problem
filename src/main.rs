use std::fs::File;
use std::io::{BufReader, BufRead};
#[macro_use]
extern crate clap;
use clap::{App, ArgMatches};
use rusty_money::{money, Money};
use std::collections::HashMap;

fn main() {
    let yaml = load_yaml!("resources/cli.yml");
    let matches: ArgMatches = App::from_yaml(yaml).get_matches();
    let input_map = process_inputs(matches);

    // let data: Vec<Vec<&str>> = extract_data(
    //     input_map.get("data_file").unwrap()
    // );

    let data_file = File::open(
        input_map.get("data_file").unwrap()
    ).unwrap();

    let reader = BufReader::new(data_file);

    for line in reader.lines() {

        let _line = line.unwrap(); //to keep obj alive
        let data: Vec<&str> = _line.split(",").collect();

        let owed = money!(data[0], "USD");
        let paid = money!(data[1], "USD");

        let change = paid - owed;
        let cents = change.to_string()
            .replace("$", "")
            .replace(".", "")
            .parse::<i32>()
            .unwrap();

        println!("{}", cents);
    }
}//END

fn process_inputs(matches: ArgMatches) -> HashMap<&str, String> {
    let input = matches.value_of("INPUT").unwrap().to_string();
    let divisor = if matches.value_of("DIVISOR").is_none() {
        println!("No divisor supplied, defaulting to a divisor of 3");
        String::from("3")
    } else {
        matches.value_of("DIVISOR").unwrap().to_string()
    };

    //TODO Clean this up somehow
    let mut args_map = HashMap::new();
    args_map.insert(
        "data_file",
        input
    );
    args_map.insert(
        "divisor",
        divisor
    );

    return args_map;
}//END

//function for file I/O scoping
fn extract_data(data_file: String) -> Vec<(String, String)> {
    let data_file = File::open(data_file).unwrap();
    let reader = BufReader::new(data_file);
    let mut data: Vec<(String, String)> = Vec::new();

    for line in reader.lines() {
        let _line = line.unwrap(); //keep obj alive
        let raw_data: Vec<&str> = _line.split(",").collect();
        let owed = raw_data[0].to_string();
        let paid = raw_data[1].to_string();

        data.push((owed, paid));
    }

    return data;
}



fn process_line() {

}