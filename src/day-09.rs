mod utils;

fn main() {
    let values: Vec<i64> = utils::read_puzzle_input()
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    let invalid_value = get_first_invalid_value(&values, 25);
    println!("Part 1: {}", invalid_value);
    println!(
        "Part 1: {}",
        get_encryption_weakness(&values, invalid_value)
    );
}

fn get_first_invalid_value(values: &Vec<i64>, preamble_length: usize) -> i64 {
    'outer: for i in preamble_length..values.len() {
        let previous = &values[i - preamble_length..i];
        let target = values[i];
        for j in 0..previous.len() {
            for k in j + 1..previous.len() {
                if previous[j] + previous[k] == target {
                    continue 'outer;
                }
            }
        }
        return target;
    }
    return -1;
}

fn get_encryption_weakness(values: &Vec<i64>, invalid_value: i64) -> i64 {
    for i in 0..values.len() {
        let mut sum = 0;
        for j in i..values.len() {
            sum += values[j];
            if sum > invalid_value {
                break;
            } else if sum == invalid_value {
                let range = &values[i..j];
                return range.iter().min().unwrap() + range.iter().max().unwrap();
            }
        }
    }
    return -1;
}
