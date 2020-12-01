use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_lines = read_puzzle_input();
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

fn read_puzzle_input() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("The input file must be provided as an argument.");
    }
    let input_path = Path::new(&args[1]);
    let file = match File::open(&input_path) {
        Err(why) => panic!("Unable to open file {}: {}", input_path.display(), why),
        Ok(file) => file,
    };
    return io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
}
