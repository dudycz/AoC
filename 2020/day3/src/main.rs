fn traverse(land: &Vec<Vec<char>>, right:usize, down:usize) -> usize {
    land.iter()
        .step_by(down)
        .enumerate()
        .filter(|&(row,stripe)| stripe.iter().cycle().nth(row*right).unwrap() == &'#')
        .count()
}

fn main() {
    let slope = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];
    let data: Vec<Vec<char>> = std::fs::read_to_string("input")
        .expect("file not found!")
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect();

    println!("With slope (3,1) encountered {} trees", traverse(&data,3,1));
    let part2= slope.iter()
        .fold(1, |acc, &(r,d)| acc * traverse(&data,r,d));

    println!("Part 2 result is {}", part2);
}