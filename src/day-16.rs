use std::collections::HashMap;
use std::collections::HashSet;

mod utils;

struct ValidRange {
    min: u32,
    max: u32,
}

struct Rule {
    name: String,
    valid_ranges: Vec<ValidRange>,
}

struct Input {
    rules: Vec<Rule>,
    ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

fn main() {
    let input = parse_input(utils::read_puzzle_input());
    println!("Part 1: {}", count_error_rate(&input));
    let decoded_ticket = decode_ticket(&input.ticket, &get_valid_tickets(&input), &input.rules);
    println!(
        "Part 2: {}",
        decoded_ticket
            .iter()
            .filter(|(name, _)| name.starts_with("departure"))
            .map(|(_, value)| *value as u64)
            .product::<u64>()
    );
}

fn decode_ticket(
    ticket: &Vec<u32>,
    valid_tickets: &Vec<Vec<u32>>,
    rules: &Vec<Rule>,
) -> HashMap<String, u32> {
    let mut unsolved_indices = HashSet::<usize>::new();
    for i in 0..ticket.len() {
        unsolved_indices.insert(i);
    }
    let mut solved_ticket = HashMap::<String, u32>::new();
    while unsolved_indices.len() > 0 {
        for i in 0..ticket.len() {
            if !unsolved_indices.contains(&i) {
                continue;
            }
            let mut matched_rules = Vec::<&Rule>::new();
            for rule in rules.iter() {
                if solved_ticket.contains_key(&rule.name) {
                    continue;
                }
                if valid_tickets
                    .iter()
                    .map(|ticket| ticket[i])
                    .all(|value| is_ticket_value_valid(&value, &rule.valid_ranges))
                {
                    matched_rules.push(rule);
                }
            }
            if matched_rules.len() == 1 {
                unsolved_indices.remove(&i);
                solved_ticket.insert(matched_rules[0].name.to_string(), ticket[i]);
                break;
            }
        }
    }
    return solved_ticket;
}

fn get_valid_tickets(input: &Input) -> Vec<Vec<u32>> {
    let mut tickets = Vec::<Vec<u32>>::new();
    if input.ticket.iter().all(|value| {
        input
            .rules
            .iter()
            .any(|rule| is_ticket_value_valid(value, &rule.valid_ranges))
    }) {
        tickets.push(input.ticket.to_vec());
    }
    for ticket in input.nearby_tickets.iter() {
        if ticket.iter().all(|value| {
            input
                .rules
                .iter()
                .any(|rule| is_ticket_value_valid(value, &rule.valid_ranges))
        }) {
            tickets.push(ticket.to_vec());
        }
    }
    return tickets;
}

fn count_error_rate(input: &Input) -> u32 {
    let mut error_rate = 0;
    for ticket in input.nearby_tickets.iter() {
        for value in ticket.iter() {
            if !input
                .rules
                .iter()
                .any(|rule| is_ticket_value_valid(value, &rule.valid_ranges))
            {
                error_rate += value;
            }
        }
    }
    return error_rate;
}

fn is_ticket_value_valid(value: &u32, ranges: &Vec<ValidRange>) -> bool {
    for range in ranges.iter() {
        if value >= &range.min && value <= &range.max {
            return true;
        }
    }
    return false;
}

fn parse_input(input: Vec<String>) -> Input {
    let mut cur_line = 0;
    let mut rules = Vec::<Rule>::new();
    while !input[cur_line].is_empty() {
        let mut valid_ranges = Vec::<ValidRange>::new();
        let parts: Vec<&str> = input[cur_line].split(": ").collect();
        let ranges: Vec<&str> = parts[1].split(" or ").collect();
        for range in ranges.iter() {
            let range_bounds: Vec<u32> = range
                .split("-")
                .map(|value| value.parse::<u32>().unwrap())
                .collect();
            valid_ranges.push(ValidRange {
                min: range_bounds[0],
                max: range_bounds[1],
            });
        }
        rules.push(Rule {
            name: parts[0].to_string(),
            valid_ranges: valid_ranges,
        });
        cur_line += 1;
    }
    cur_line += 2;
    let ticket = input[cur_line]
        .split(",")
        .map(|value| value.parse::<u32>().unwrap())
        .collect();
    cur_line += 3;
    let mut nearby_tickets = Vec::<Vec<u32>>::new();
    while cur_line < input.len() {
        nearby_tickets.push(
            input[cur_line]
                .split(",")
                .map(|value| value.parse::<u32>().unwrap())
                .collect(),
        );
        cur_line += 1;
    }
    return Input {
        rules: rules,
        ticket: ticket,
        nearby_tickets: nearby_tickets,
    };
}
