use std::fs::File;
use std::io::{Write, BufReader, BufRead};
#[macro_use]
extern crate clap;
use clap::{App, ArgMatches};
use rusty_money::{money, Money};
use std::collections::HashMap;
use std::path::Path;

fn main() {
    let yaml = load_yaml!("resources/cli.yml");
    let matches: ArgMatches = App::from_yaml(yaml).get_matches();
    //better way of doing this..
    process_inputs(matches);
    println!("Done!");
}//END


//ArgMatches doesn't have an iterator so this is kind of messy...
fn process_inputs(matches: ArgMatches) { //-> HashMap<&str, String>
    let input = matches.value_of("INPUT").unwrap().to_string();

    //Code for determining divisor
    // let divisor = if matches.value_of("DIVISOR").is_none() {
    //     println!("No divisor supplied, defaulting to a divisor of 3");
    //     String::from("3")
    // } else {
    //     matches.value_of("DIVISOR").unwrap().to_string()
    // };

    let mut args_map = HashMap::new();
    args_map.insert(
        "data_file",
        input
    );

    // args_map.insert(
    //     "divisor",
    //     divisor
    // );

    extract_data(args_map)
    // return args_map;
}//END


//function for file I/O scoping
fn extract_data(args_map: HashMap<&str, String>) { //-> Vec<(String, String)>
    let data_file = File::open(
        args_map.get("data_file").unwrap()
    ).unwrap();
    let reader = BufReader::new(data_file);
    let mut data: Vec<(String, String)> = Vec::new();

    for line in reader.lines() {
        let _line = line.unwrap(); //keep obj alive
        let raw_data: Vec<&str> = _line.split(",").collect();
        let owed = raw_data[0].to_string();
        let paid = raw_data[1].to_string();

        data.push((owed, paid));
    }

    // determine_change(args_map.get("divisor").unwrap(), data)
    determine_change(data)
}//END

#[allow(unused_variables)] //suppress compiler warning
//divisor: &String,
fn determine_change(data: Vec<(String, String)>) { //-> Vec<u32>
    let mut cents_vec: Vec<u32> = Vec::new();
    for item in data {
        //paid - owed
        let change = money!(item.1, "USD") - money!(item.0, "USD");
        let cents = change.to_string()
            .replace("$", "")
            .replace(".", "")
            .parse::<u32>()
            .unwrap();

        cents_vec.push(cents);
    }
    // Logic for checking divisor would have probably
    // implemented here on a match {}

    minimal_change(cents_vec)
}//END


const COINS: [u32; 4] = [25, 10, 5, 1];
fn minimal_change(cents: Vec<u32>) { //-> String
    let mut change_results = Vec::new();
    for amount in cents {
        let mut q = 0;
        let mut d = 0;
        let mut n = 0;
        let mut p = 0;
        let mut remainder = amount as usize;
        for &coin in &COINS {
            let coin = coin as usize;
            while remainder >= coin {
                match coin {
                    25 => q += 1,
                    10 => d += 1,
                    5 => n += 1,
                    1 => p += 1,
                    _ => {}
                }

                remainder = remainder - coin;
            }
        }
        change_results.push(
            format!("{} quarters, {} dimes, {} nickles, {} pennies", q, d, n, p)
        );
    }

    write_output_file(change_results)
}//END


fn write_output_file(output_strings: Vec<String>) {
    let path = Path::new("output.txt");
    let display = path.display();

    let mut output_file = match File::create(&path) {
        Err(why) => panic!("Can't create {}: {}", display, why),
        Ok(file) => file
    };

    for item in &output_strings {
        write!(output_file, "{}\n", item)
            .expect("Can't write to file!");
    }
}//END


//--------------------------------------------------------
// My attempt at finding all possible change combinations
// for the given sum (in cents). The idea was to find all
// combinations, and then select one at random and supply
// it for the "twist" divisor case.
//
// Unfortunately, I wasn't able to get my attempt at an
// algorithm to compile, so I decided to comment it out,
// and remove the "connecting" code, but still include
// it for viewing. The code for my attempt is below.
//--------------------------------------------------------
//
// fn unique_sums(coins: Vec<u32>, target: u32) {
//     let mut result: Vec<Vec<u32>> = vec![vec![]];
//
//     back_track(
//         result,
//         Vec::new(),
//         target,
//         coins,
//         0
//     );
//
//     for i in result.iter().flatten() {
//         // let i = &i as usize;
//         println!("{}", i )
//     }
//     How to return something here...
// }
//
// fn back_track (
//     mut results: Vec<Vec<u32>>,
//     mut temp: Vec<u32>,
//     remains: u32,
//     nums: Vec<u32>,
//     start: u32
// ) {
//     if remains == 0 {
//         results.push(temp.clone()); //.clone() ? maybe
//         return
//     }
//
//     for i in start..=3 - 1 {
//         let i = i as usize;
//         temp.push(nums[i]);
//         back_track(results.clone(),
//                    temp.clone(),
//                    remains - nums[i].clone(),
//                    nums.clone(),
//                    start + 1);
//
//         temp.remove(temp.len() - 1);
//     }
// }