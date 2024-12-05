use std::fs;
use std::iter::zip;

fn main() {

    let file_contents = fs::read_to_string("input.txt").expect(
        "Was not able to read the file"
    );

    let addresses = file_contents.lines();

    let mut left_addresses: Vec<i32> = Vec::new();
    let mut right_addresses: Vec<i32> = Vec::new();

    for address in addresses {
        let addresses_split: Vec<&str> = address.split("   ").collect();

        let left: &str = &addresses_split[0];
        let right: &str = &addresses_split[1];

        left_addresses.push(
            left.parse().expect("Could not read {left}")
        );
        right_addresses.push(
            right.parse().expect("Could not read {right}")
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
