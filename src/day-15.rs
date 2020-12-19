use std::collections::HashMap;

mod utils;

fn main() {
    let values: Vec<i32> = utils::read_puzzle_input()[0]
        .split(",")
        .map(|value| value.parse::<i32>().unwrap())
        .collect();

    println!("Part 1: {}", get_nth_number(&values, 2020));
    println!("Part 2: {}", get_nth_number(&values, 30000000));
}

fn get_nth_number(input: &Vec<i32>, number: i32) -> i32 {
    let mut value_to_turns = HashMap::<i32, Vec<i32>>::new();
    let mut last_number = 0;
    let mut cur_number = 0;
    for turn in 1..(number + 1) {
        if turn - 1 < input.len() as i32 {
            cur_number = input[turn as usize - 1];
        } else {
            match value_to_turns.get(&last_number) {
                Some(turns) => {
                    if turns.len() == 1 {
                        cur_number = 0;
                    } else {
                        cur_number = (turns[turns.len() - 1] - turns[turns.len() - 2]) as i32;
                    }
                }
                None => {}
            };
        }

        last_number = cur_number;
        let entry = value_to_turns
            .entry(last_number)
            .or_insert(Vec::<i32>::new());
        entry.push(turn);
    }
    return last_number;
}
