fn is_valid(s: &str) -> bool {
    use regex::Regex;
    let re = Regex::new(r"^([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();
    let cap = re.captures(s).unwrap();
    let min:usize = cap[1].parse().unwrap();
    let max:usize = cap[2].parse().unwrap();
    let cnt = cap[4].matches(&cap[3]).count();
    min <= cnt && cnt <= max
}

fn is_valid_part2(s: &str) -> bool {
    use regex::Regex;
    let re = Regex::new(r"^([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();
    let cap = re.captures(s).unwrap();
    let p1:usize = cap[1].parse().unwrap();
    let p2:usize = cap[2].parse().unwrap();
    let char:u8 = cap[3].as_bytes()[0];
    let first:u8 = cap[4].as_bytes()[p1-1];
    let second:u8 = cap[4].as_bytes()[p2-1];

    (first == char || second == char) && first != second
}

fn main() {

    let count = std::fs::read_to_string("input")
        .expect("file not found!")
        .lines()
        .filter(|x| is_valid(x))
        .count();

    let count2 = std::fs::read_to_string("input")
        .expect("file not found!")
        .lines()
        .filter(|x| is_valid_part2(x))
        .count();

    println!("Liczba poprawnych haseł {}", count);
    println!("Liczba poprawnych haseł v2 {}", count2);
}
