use std::collections::HashMap;

mod utils;

fn main() {
    let mut adapters: Vec<i32> = utils::read_puzzle_input()
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    adapters.sort();
    adapters.push(adapters[adapters.len() - 1] + 3);
    let (diffs_of_1, diffs_of_3) = count_diffs(&adapters);
    println!("Part 1: {}", diffs_of_1 * diffs_of_3);
    println!("Part 2: {}", count_arrangements(&adapters));
}

fn count_diffs(adapters: &Vec<i32>) -> (i32, i32) {
    let mut last_value = 0;
    let mut diffs_of_1 = 0;
    let mut diffs_of_3 = 0;
    for adapter in adapters.iter() {
        if adapter - last_value == 1 {
            diffs_of_1 += 1;
        } else if adapter - last_value == 3 {
            diffs_of_3 += 1;
        }
        last_value = *adapter;
    }
    return (diffs_of_1, diffs_of_3);
}

fn count_arrangements(adapters: &Vec<i32>) -> i64 {
    let mut combinations = HashMap::<i32, i64>::new();
    combinations.insert(0, 1);
    for adapter in adapters.iter() {
        let mut count = 0;
        if combinations.contains_key(&(adapter - 1)) {
            count += combinations[&(adapter - 1)];
        }
        if combinations.contains_key(&(adapter - 2)) {
            count += combinations[&(adapter - 2)];
        }
        if combinations.contains_key(&(adapter - 3)) {
            count += combinations[&(adapter - 3)];
        }
        combinations.insert(*adapter, count);
    }
    return combinations[&adapters[adapters.len() - 1]];
}
