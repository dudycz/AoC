fn decode(s: &str) -> u32 {
    let bin_num = s
        .chars()
        .map(|c| match c {
            'F' | 'L' => '0',
            'R' | 'B' => '1',
            _ => unimplemented!(),
        })
        .collect::<String>();
    u32::from_str_radix(bin_num.as_str(), 2).unwrap()
}

fn main() {
    let mut boarding_cards: Vec<u32> = std::fs::read_to_string("input")
        .expect("file not found!")
        .lines()
        .map(|x| decode(x))
        .collect();

    let max = *boarding_cards.iter().max().unwrap();
    let min = *boarding_cards.iter().min().unwrap();
    let missing_board_id = (min..max).find(|&x| !boarding_cards.contains(&x)).unwrap();

    println!("Max boarding card id is {}", max);
    println!("Missing boarding card id is {}", missing_board_id);
}
