use std::collections::HashMap;
use std::vec::Vec;

use std::io::{BufRead, BufReader};
use std::fs::File;

fn are_simmilar(lhs: &str, rhs: &str) -> bool {
    if lhs.len() == rhs.len() {
        let distance = lhs.chars().zip(rhs.chars()).fold(0, |acc, (lhs_char, rhs_char)| acc + (lhs_char != rhs_char) as i32);
        if distance == 1 {
            return true;
        }
    }
    return false;
}

fn print_common_characters(lhs: &str, rhs: &str) {
    println!("Common part between IDs");
    for pair in lhs.chars().zip(rhs.chars()).filter(|pair| pair.0 == pair.1) {
        print!("{}", pair.0);
    }
}

fn find_common_letters(ids: &Vec<String>) {
    for (index, a) in ids.iter().cloned().enumerate() {
        for b in ids[index + 1..].iter().cloned() {
            if are_simmilar(&a,&b) {
                print_common_characters(&a, &b);
            }
        }
    }
}

fn calculate_checksum(ids: &Vec<String>) -> i32 {
    let mut doubles = 0;
    let mut tripples = 0;
    let mut dict = HashMap::new();

    for line in ids {
        for c in line.chars() {
            let counter = dict.entry(c).or_insert(0);
            *counter += 1;
        }

        for (_letter, value) in dict.iter() {
            if *value == 2 {
                doubles += 1;
                break;
            }
        }

        for (_letter, value) in dict.iter() {
            if *value == 3 {
                tripples += 1;
                break;
            }
        }
    }

    doubles * tripples
}

fn main() {
    let file = File::open("src/input").expect("cannot open file");
    let file = BufReader::new(file);
    let ids: Vec<String> = file.lines().filter_map(|result| result.ok()).collect();

    println!("Checksum {}", calculate_checksum(&ids));

    find_common_letters(&ids);
}
