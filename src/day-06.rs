use std::collections::HashMap;

mod utils;

struct Group {
    count: i32,
    responses: HashMap<char, i32>,
}

fn main() {
    let input_lines = utils::read_puzzle_input();
    let groups = load_groups(input_lines);
    println!(
        "Part 1: {}",
        groups
            .iter()
            .map(|group| group.responses.iter().count())
            .sum::<usize>()
    );

    println!(
        "Part 2: {}",
        groups
            .iter()
            .map(|group| group
                .responses
                .values()
                .filter(|value| value == &&group.count)
                .count())
            .sum::<usize>()
    );
}

fn load_groups(lines: Vec<String>) -> Vec<Group> {
    let mut groups = Vec::<Group>::new();
    let mut group = Group {
        count: 0,
        responses: HashMap::<char, i32>::new(),
    };
    for line in lines.iter() {
        if line.is_empty() {
            groups.push(group);
            group = Group {
                count: 0,
                responses: HashMap::<char, i32>::new(),
            };
            continue;
        }

        group.count += 1;
        for c in line.chars() {
            let mut new_value = 1;
            if group.responses.contains_key(&c) {
                new_value = group.responses[&c] + 1;
                group.responses.remove(&c);
            }
            group.responses.insert(c, new_value);
        }
    }
    groups.push(group);
    return groups;
}
