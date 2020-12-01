mod utils;

fn main() {
    let input_lines = utils::read_puzzle_input();
    let values: Vec<i32> = input_lines
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    for i in 0..values.len() {
        for j in (i + 1)..values.len() {
            if values[i] + values[j] == 2020 {
                println!("Part 1: {}", values[i] * values[j]);
            }
        }
    }
    for i in 0..values.len() {
        for j in (i + 1)..values.len() {
            for k in (j + 1)..values.len() {
                if values[i] + values[j] + values[k] == 2020 {
                    println!("Part 1: {}", values[i] * values[j] * values[k]);
                }
            }
        }
    }
}
