use std::collections::HashSet;

fn main() {
    let answers: Vec<Vec<Vec<char>>> = std::fs::read_to_string("input")
        .expect("file not found!")
        .split("\n\n")
        .map(|group| group.lines().map(|c| c.chars().collect()).collect())
        .collect();

    dbg!(answers.iter().fold(0, |acc, group| acc
        + group.iter().flatten().collect::<HashSet<&char>>().len()));

    dbg!(answers.iter().fold(0, |acc, group| acc
        + (b'a'..=b'z')
            .filter(|&c| group.iter().all(|v| v.contains(&(c as char))))
            .count()));
}
