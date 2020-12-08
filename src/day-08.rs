use regex::Regex;
use std::collections::HashSet;

mod utils;

fn main() {
    let program = utils::read_puzzle_input();

    let (_, accumulator) = run_program(&program);
    println!("Part 1: {}", accumulator);

    for i in 0..program.len() {
        let mut program_copy = program.to_vec();
        if program_copy[i].starts_with("nop") {
            program_copy[i] = program_copy[i].replace("nop", "jmp");
        } else if program_copy[i].starts_with("jmp") {
            program_copy[i] = program_copy[i].replace("jmp", "nop");
        } else {
            continue;
        }
        let (terminated, accumulator) = run_program(&program_copy);

        if terminated {
            println!("Part 2: {}", accumulator);
            break;
        }
    }
}

fn run_program(program: &Vec<String>) -> (bool, i32) {
    let mut line_index = 0;
    let mut seen_lines = HashSet::<i32>::new();
    let mut accumulator = 0;
    while !seen_lines.contains(&line_index) && line_index >= 0 && line_index < program.len() as i32
    {
        let program_line = &program[line_index as usize];
        let (instruction, instruction_arg) = parse_program_line(&program_line);
        seen_lines.insert(line_index);

        match instruction.as_str() {
            "acc" => {
                accumulator += instruction_arg;
                line_index += 1
            }
            "nop" => line_index += 1,
            "jmp" => line_index += instruction_arg,
            _ => panic!("Unknown instruction"),
        }
    }
    return (line_index == program.len() as i32, accumulator);
}

fn parse_program_line(line: &String) -> (String, i32) {
    let inst_re = Regex::new(r"([[:alpha:]]+)\s([+-])(\d+)").unwrap();
    let captures = inst_re.captures(line).unwrap();
    let instruction = captures.get(1).unwrap().as_str();
    let mut instruction_arg = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
    let instruction_arg_sign = captures.get(2).unwrap().as_str();
    if instruction_arg_sign.eq("-") {
        instruction_arg *= -1;
    }
    return (instruction.to_string(), instruction_arg);
}
