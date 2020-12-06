use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn has_all_fields(pass: &HashMap<&str,&str>) -> bool
{
    ["byr","iyr","eyr","hgt","hcl","ecl","pid"].iter()
        .all(|p| pass.contains_key(p))
}

fn is_in_range(s: &str, min:u32, max:u32) -> bool
{
    let val = s.parse::<u32>().unwrap();
    val >= min && val <= max
}

fn has_valid_hgt(s : &str) -> bool
{
    let (hgt,unit) = s.split_at(s.len()-2);

    match unit {
        "in" => is_in_range(hgt, 59,76),
        "cm" => is_in_range(hgt,150,193),
        _ => false
    }
}

fn has_valid_pid(s : &str) -> bool
{
    let re = Regex::new(r"^\d{9}$").unwrap();
    re.is_match(s)
}

fn has_valid_hcl(s : &str) -> bool
{
    let re = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
    re.is_match(s)
}

fn has_valid_ecl(s : &str) -> bool
{
    ["amb","blu","brn","gry","grn","hzl","oth"].contains(&s)
}

fn is_valid(pass: &HashMap<&str,&str>) -> bool
{
    pass.iter().all(|(&k, v)| match k {
        "byr" => is_in_range(v, 1920,2002),
        "iyr" => is_in_range(v, 2010,2020),
        "eyr" => is_in_range(v, 2020,2030),
        "hcl" => has_valid_hcl(v),
        "ecl" => has_valid_ecl(v),
        "pid" => has_valid_pid(v),
        "cid" => true,
        "hgt" => has_valid_hgt(v),
        _ => unreachable!()
    })
}

fn main() {
    if let Ok(lines) = read_lines("src/input") {

        let mut pre_checked = 0;
        let mut validated = 0;

        for line in lines {
            let line = line.unwrap();
            let passport = line
                .split_whitespace()
                .flat_map(|p| p.split(':'))
                .tuples()
                .collect::<HashMap<_,_>>();

            if has_all_fields(&passport)
            {
                pre_checked += 1;
                if is_valid(&passport)
                {
                    validated += 1;
                }
            }
        }
        println!("Number of passports: pre-checked={}, validated={}",pre_checked, validated);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}