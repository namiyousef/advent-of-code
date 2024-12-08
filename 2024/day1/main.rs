use std::fs;
use std::iter::zip;
use std::env;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {

    const PART: i32 = 2;

    // create hashmap for storing the index of right addresses
    let mut right_addresses_index: HashMap<i32, i32> = HashMap::new();
    let mut right_addresses_counter: Vec<i32> = Vec::new();

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
        );

        if PART == 2 {
            let addresses_size = right_addresses_index.len() as i32;
            if !right_addresses_index.contains_key(&right_parsed) {
                right_addresses_index.insert(
                    right_parsed, addresses_size
                );
                right_addresses_counter.push(1);
            } else {
                let index = *right_addresses_index.get(&right_parsed).expect(
                    "Expected to get a value from the hashmap"
                ) as usize;
                println!("Incrementing index={index}");
                right_addresses_counter[index] += 1;
            }
        }
    }

    left_addresses.sort();
    right_addresses.sort();
    
    let mut total_distance = 0;

    for (left, right) in zip(left_addresses, right_addresses) {
        if PART == 1 {
            let diff = (right - left).abs();
            total_distance = total_distance + diff;
        } else {
            println!("Looking to see if {left} in right addresses..");
            let left_number_index = *right_addresses_index.get(
                &left
            ).unwrap_or(&-1);
            let mut num_occurances = 0;
            if left_number_index != -1 {
                num_occurances = right_addresses_counter[
                    left_number_index as usize
                ];
            }
            let diff = left * num_occurances;
            total_distance += diff;
            println!("{left} appears {num_occurances} times in right");
        }

    }

    println!("The total distance is: {total_distance}")

}
