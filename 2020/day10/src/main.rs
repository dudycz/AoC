fn count_diffs(data: &[u32], val: u32) -> u32 {
    data.windows(2)
        .map(|s| (s[1] - s[0]))
        .filter(|diff| *diff == val)
        .count() as u32
}

fn count_permutations(data: &[u32]) -> u64 {
    let mut amount = 0;
    let mut ret: u64 = 1;
    for slice in data.windows(2) {
        if slice[1] - slice[0] == 1 {
            amount += 1;
        } else {
            match amount {
                2 => ret *= 2,
                3 => ret *= 4,
                4 => ret *= 7,
                _ => (),
            }
            amount = 0;
        }
    }
    ret
}

fn main() {
    let mut data: Vec<u32> = std::fs::read_to_string("input")
        .expect("file not found!")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    data.sort();
    data.push(data.last().unwrap() + 3);
    data.insert(0, 0);

    println!(
        "Part 1 answer is {}",
        count_diffs(&data, 1) * count_diffs(&data, 3)
    );
    println!("Part 2 answer is {}", count_permutations(&data));
}
