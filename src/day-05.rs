use std::collections::HashSet;

mod utils;

const MAX_ROW: i32 = 127;
const MAX_COL: i32 = 7;

fn main() {
    let input_lines = utils::read_puzzle_input();
    let mut seen_ids = HashSet::<i32>::new();
    for line in input_lines.iter() {
        let row = get_seat_row(line);
        let col = get_seat_col(line);
        let id = row * 8 + col;
        seen_ids.insert(id);
    }

    println!("Part 1: {}", seen_ids.iter().max().unwrap());

    for i in 0..MAX_ROW + 1 {
        for j in 0..MAX_COL + 1 {
            let id = i * 8 + j;
            if !seen_ids.contains(&id)
                && seen_ids.contains(&(id - 1))
                && seen_ids.contains(&(id + 1))
            {
                println!("Part 2: {}", id);
            }
        }
    }
}

fn get_seat_row(line: &str) -> i32 {
    let bytes = line.as_bytes();
    let mut lower_bound = 0;
    let mut upper_bound = MAX_ROW;
    for i in 0..6 {
        let change = get_midpoint(lower_bound, upper_bound);
        if bytes[i] as char == 'F' {
            upper_bound -= change;
        } else if bytes[i] as char == 'B' {
            lower_bound += change;
        }
    }
    if bytes[6] as char == 'F' {
        return lower_bound;
    } else {
        return upper_bound;
    }
}

fn get_seat_col(line: &str) -> i32 {
    let bytes = line.as_bytes();
    let mut lower_bound = 0;
    let mut upper_bound = MAX_COL;
    for i in 7..9 {
        let change = get_midpoint(lower_bound, upper_bound);
        if bytes[i] as char == 'L' {
            upper_bound -= change;
        } else if bytes[i] as char == 'R' {
            lower_bound += change;
        }
    }
    if bytes[9] as char == 'L' {
        return lower_bound;
    } else {
        return upper_bound;
    }
}

fn get_midpoint(lower: i32, upper: i32) -> i32 {
    if upper % 2 == 0 {
        return (upper - lower) / 2;
    } else {
        return ((upper - lower) / 2) + 1;
    }
}
