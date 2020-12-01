use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_puzzle_input() -> Vec<String> {
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
