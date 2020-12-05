use std::collections::HashMap;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn has_all_fields(s: &HashMap<String,String>, v: &Vec<&str>) -> bool
{
    let mut valid= true;
    for val in v.iter()
    {
        if !s.contains_key(*val)
        {
            valid = false;
            break;
        }
    }
    valid
}

fn has_valid_eyr(s: &HashMap<String,String>) -> bool
{
    let s = s.get("eyr").unwrap().parse::<u32>().unwrap();
    s >= 2020 && s <= 2030
}

fn has_valid_iyr(s: &HashMap<String,String>) -> bool
{
    let s = s.get("iyr").unwrap().parse::<u32>().unwrap();
    s >= 2010 && s <= 2020
}

fn has_valid_byr(s : &HashMap<String,String>) -> bool
{
    let s = s.get("byr").unwrap().parse::<u32>().unwrap();
    s >= 1920 && s <= 2002
}

fn has_valid_hgt(s : &HashMap<String,String>) -> bool
{
    let mut s = s.get("hgt").unwrap().clone();
    let unit = s.split_off(s.len()-2);
    let hgt = s.parse::<u32>().unwrap();
    match unit.as_str() {
        "in" => hgt >= 59 && hgt <= 76,
        "cm" => hgt>=150 && hgt <= 193,
        _ => false
    }
}

fn has_valid_pid(s : &HashMap<String,String>) -> bool
{
    use regex::Regex;
    let s = s.get("pid").unwrap();
    let re = Regex::new(r"^\d{9}$").unwrap();
    re.is_match(s)
}

fn has_valid_hcl(s : &HashMap<String,String>) -> bool
{
    use regex::Regex;
    let s = s.get("hcl").unwrap();
    let re = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
    re.is_match(s)
}
fn has_valid_ecl(s : &HashMap<String,String>) -> bool
{
    let col = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let s = s.get("ecl").unwrap();
    col.iter().any(|&i| i==s)
}

fn main() {
    let keywords = vec!["byr","iyr","eyr","hgt","hcl","ecl","pid"];
    if let Ok(lines) = read_lines("src/input") {

        let mut pre_checked = 0;
        let mut valid = 0;
        for line in lines {
            if let Ok(ip) = line {
                let split:HashMap<String,String> = ip.split(" ")
                    .map(|kv| kv.split(':').collect::<Vec<&str>>())
                    .map(|vec| {
                        assert_eq!(vec.len(), 2, "Failed with {:?}",vec);
                        (vec[0].to_string(), vec[1].to_string())
                    })
                    .collect();

                if has_all_fields(&split,&keywords)
                {
                    if has_valid_byr(&split) && has_valid_eyr(&split) &&
                        has_valid_iyr(&split) && has_valid_pid(&split) &&
                        has_valid_hgt(&split) && has_valid_ecl(&split) &&
                        has_valid_hcl(&split)
                    {
                        valid += 1;
                    }
                    pre_checked += 1;
                }
            }
        }
        println!("Number of passports: pre-checked={}, validated={}",pre_checked, valid);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}