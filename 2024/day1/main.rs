use std::fs;
use std::iter::zip;
use std::env;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {

    match env::var("PART") {
        Ok(val) => {
            match val.as_str().parse().expect("Could not parse environment variable as an integer") {
                1 => {
                    println!("Starting part 1...")
                },
                2 => {
                    println!("Starting part 2...");
                    // create hashmap for storing the index of right addresses
                    let mut right_addresses_index: HashMap<i32, i32> = HashMap::new();
                    let mut right_addresses_counter: Vec<i32> = Vec::new();
                },
                _ => {
                    println!("Got PATH of '{val}'. Expected '1' or '2'")
                }
            }
        },
        Err(_e) => {
            panic!("Could not find the environment variable PART: Please set it to either '1' or '2'")
        }
    }

    let file_contents = fs::read_to_string("assets/input.txt").expect(
        "Was not able to read the file"
    );

    let addresses = file_contents.lines();

    let mut left_addresses: Vec<i32> = Vec::new();
    let mut right_addresses: Vec<i32> = Vec::new();

    for (i, address) in addresses.enumerate() {
        let addresses_split: Vec<&str> = address.split("   ").collect();

        let left: &str = &addresses_split[0];
        let right: &str = &addresses_split[1];

        let left_parsed: i32 = left.parse().expect("Could not read {left}");
        let right_parsed: i32 = right.parse().expect("Could not read {right}");

        left_addresses.push(
            left_parsed
        );
        right_addresses.push(
            right_parsed
        )
    }

    left_addresses.sort();
    right_addresses.sort();
    
    let mut total_distance = 0;

    for (left, right) in zip(left_addresses, right_addresses) {
        let diff = (right - left).abs();
        total_distance = total_distance + diff;
    }

    println!("The total distance is: {total_distance}")

}
