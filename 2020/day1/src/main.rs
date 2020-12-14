use std::collections::HashSet;

fn part1(slice: &[i64], number: i64) -> i64 {
    let mut table = HashSet::new();
    for s in slice {
        let diff = number - s;
        if table.contains(&diff)  {
            return diff*s;
        }
        table.insert(s);
    }
    0
}

fn part2(slice: &[i64], number: i64) -> i64 {
    for i in 0..slice.len()-2 {
        let mut table = HashSet::new();
        let sum= number - slice[i];
        for s in slice.iter().skip(i+1) {
            let diff = sum - s;
            if table.contains(&diff)  {
                return slice[i]*s*(sum-s);
            }
            table.insert(s);
        }
    }
    0
}


fn main() {
    let data:Vec<i64> = std::fs::read_to_string("input")
        .expect("file not found!")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("Part 1: {}",part1(&data, 2020));
    println!("Part 2: {}",part2(&data, 2020));
}

