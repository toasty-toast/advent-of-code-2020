use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

mod utils;

fn main() {
    let program = utils::read_puzzle_input();
    println!("Part 1: {}", run_program_v1(&program).values().sum::<u64>());
    println!("Part 2: {}", run_program_v2(&program).values().sum::<u64>());
}

fn run_program_v1(program: &Vec<String>) -> HashMap<u64, u64> {
    let mut mask: [char; 36] = [
        'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X',
        'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X',
    ];
    let mut memory = HashMap::<u64, u64>::new();
    for line in program.iter() {
        if line.starts_with("mask") {
            let mask_str = &line.split(" ").collect::<Vec<&str>>()[2];
            for (i, c) in mask_str.chars().enumerate() {
                mask[i] = c;
            }
        } else {
            let captures = Regex::new(r"mem\[(\d+)\]\s=\s(\d+)")
                .unwrap()
                .captures(line)
                .unwrap();
            let address = captures[1].parse::<u64>().unwrap();
            let mut value = captures[2].parse::<u64>().unwrap();
            if memory.contains_key(&address) {
                memory.remove(&address);
            }
            for (i, c) in mask.iter().enumerate() {
                let bit = (mask.len() - 1 - i) as u64;
                if *c != 'X' {
                    let override_value = c.to_digit(10).unwrap() as u64;
                    value = set_bit(value, bit, override_value);
                }
            }
            memory.insert(address, value);
        }
    }
    return memory;
}

fn run_program_v2(program: &Vec<String>) -> HashMap<u64, u64> {
    let mut mask: [char; 36] = [
        'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X',
        'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X',
    ];
    let mut memory = HashMap::<u64, u64>::new();
    for line in program.iter() {
        if line.starts_with("mask") {
            let mask_str = &line.split(" ").collect::<Vec<&str>>()[2];
            for (i, c) in mask_str.chars().enumerate() {
                mask[i] = c;
            }
        } else {
            let captures = Regex::new(r"mem\[(\d+)\]\s=\s(\d+)")
                .unwrap()
                .captures(line)
                .unwrap();
            let mut base_address = captures[1].parse::<u64>().unwrap();
            let value = captures[2].parse::<u64>().unwrap();
            for (i, c) in mask.iter().enumerate() {
                if *c == '1' {
                    base_address = set_bit(base_address, (mask.len() - 1 - i) as u64, 1);
                }
            }
            let mut floating_bits = Vec::<u64>::new();
            for (i, c) in mask.iter().enumerate() {
                if *c == 'X' {
                    floating_bits.push((mask.len() - 1 - i) as u64);
                }
            }
            if floating_bits.len() == 0 {
                if memory.contains_key(&base_address) {
                    memory.remove(&base_address);
                }
                memory.insert(base_address, value);
            } else {
                for i in 0..floating_bits.len() + 1 {
                    for perm in floating_bits.iter().combinations(i) {
                        let mut address = base_address;
                        for bit in floating_bits.iter() {
                            if perm.contains(&bit) {
                                address = set_bit(address, *bit, 1);
                            } else {
                                address = set_bit(address, *bit, 0);
                            }
                        }
                        if memory.contains_key(&address) {
                            memory.remove(&address);
                        }
                        memory.insert(address, value);
                    }
                }
            }
            memory.insert(base_address, value);
        }
    }
    return memory;
}

fn set_bit(value: u64, bit: u64, bit_value: u64) -> u64 {
    return (value & !(1 << bit)) | (bit_value << bit);
}
