use regex::Regex;

mod utils;

fn main() {
    let input_lines = utils::read_puzzle_input();
    let re = Regex::new(r"(\d+)\-(\d+)\s*([[:alpha:]]):\s*([[:alpha:]]+)").unwrap();
    let mut valid_passwords_by_count = 0;
    let mut valid_passwords_by_position = 0;
    for line in input_lines.iter() {
        if !re.is_match(line) {
            continue;
        }

        let captures = re.captures(line).unwrap();
        let first_value = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let second_value = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let character = captures.get(3).unwrap().as_str().chars().next().unwrap();
        let password = captures.get(4).unwrap().as_str();

        let mut character_count = 0;
        for c in password.chars() {
            if character == c {
                character_count += 1;
            }
        }
        if character_count >= first_value && character_count <= second_value {
            valid_passwords_by_count += 1;
        }

        if (password.chars().nth((first_value - 1) as usize).unwrap() == character)
            ^ (password.chars().nth((second_value - 1) as usize).unwrap() == character)
        {
            valid_passwords_by_position += 1;
        }
    }
    println!("Part 1: {}", valid_passwords_by_count);
    println!("Part 2: {}", valid_passwords_by_position);
}
