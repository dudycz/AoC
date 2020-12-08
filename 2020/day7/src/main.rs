use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

fn parse_line(s: &str) -> (String, Vec<(u32, String)>) {
    let mut iter = s.split(" bags contain ");
    let bag_color = iter.next().unwrap().to_string();
    let re = Regex::new(r"([0-9]+) (.+?) bag").unwrap();
    let mut contains: Vec<(u32, String)> = Vec::new();
    for cap in re.captures_iter(iter.next().unwrap()) {
        contains.push(((&cap[1]).parse().unwrap(), (&cap[2]).to_string()));
    }
    (bag_color, contains)
}

fn calc_part_1(input: &HashMap<String, Vec<(u32, String)>>) -> usize {
    let mut may_have: HashSet<String> = HashSet::new();
    let mut to_process: HashSet<String> = HashSet::new();
    to_process.insert("shiny gold".to_string());
    while to_process.len() > 0 {
        let mut next_batch: HashSet<String> = HashSet::new();
        for color in to_process.iter() {
            for (key, val) in input.iter() {
                if val.iter().any(|x| x.1 == *color) {
                    may_have.insert(key.clone());
                    next_batch.insert(key.clone());
                }
            }
        }
        to_process = next_batch;
    }

    may_have.len()
}

fn calc_part_2(color: &String, input: &HashMap<String, Vec<(u32, String)>>) -> u32 {
    input.get(color)
        .unwrap()
        .iter()
        .map(|(count, color)| count + count * calc_part_2(color, &input))
        .sum::<u32>()
}

fn main() {
    if let Ok(lines) = read_lines("input") {
        let mut data = HashMap::new();
        for line in lines {
            let entry = parse_line(&line.unwrap());
            data.insert(entry.0, entry.1);
        }

        println!("Part 1: {}", calc_part_1(&data));
        println!("Part 2: {}", calc_part_2(&"shiny gold".to_string(), &data));

    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_parser() {
        assert_eq!(
            (
                "light red".to_string(),
                vec![
                    (1, "bright white".to_string()),
                    (2, r"muted yellow".to_string())
                ]
            ),
            parse_line("light red bags contain 1 bright white bag, 2 muted yellow bags.")
        );
        assert_eq!(
            ("faded blue".to_string(), vec![]),
            parse_line("faded blue bags contain no other bags.")
        );
    }
}
