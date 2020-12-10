use std::collections::HashSet;

fn contains_sum(slice: &[i64], number: i64) -> bool {
    let mut table = HashSet::new();
    for s in slice {
        let diff = number - s;
        if table.contains(&diff)  {
            return true;
        }
        table.insert(s);
    }
    false
}

fn find_weakness(data: &[i64], window_size: usize) -> i64 {
    let (_, number) = data.iter()
        .skip(window_size)
        .enumerate()
        .find(|(i,&number)| !contains_sum(&data[*i..*i+window_size], number))
        .unwrap();
    *number
}

fn crash_xmas_encryption(data: &[i64], weak_number: i64) -> i64 {
    let mut begin = 0;
    let mut end =0;
    let mut sum:i64 = data[end];

    loop {
        if sum > weak_number {
            sum -= data[begin];
            begin += 1;
        }
        else if sum < weak_number {
            end += 1;
            sum += data[end];
        }
        else if sum == weak_number {
            break;
        }
    }

    let mut tmp = data[begin..end].to_vec();
    tmp.sort();
    tmp.first().unwrap() + tmp.last().unwrap()
}

fn main() {
    let data:Vec<i64> = std::fs::read_to_string("input")
        .expect("file not found!")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    let weak_number = find_weakness(&data, 25);
    println!("Weak number is {}", weak_number);
    println!("Number that crashes XMAS is {}", crash_xmas_encryption(&data, weak_number));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_part1() {
        let input = vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576];
        assert_eq!(find_weakness(&input, 5), 127);
    }
}

