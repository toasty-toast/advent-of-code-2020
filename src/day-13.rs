mod utils;

fn main() {
    let input_lines = utils::read_puzzle_input();
    let earliest_departure = input_lines[0].parse::<i32>().unwrap();
    let bus_ids: Vec<i32> = input_lines[1]
        .split(",")
        .map(|id| match id {
            "x" => -1,
            _ => id.parse::<i32>().unwrap(),
        })
        .collect();
    let mut earliest_bus_id = -1;
    let mut earliest_bus_departure = i32::MAX;
    for id in bus_ids.iter() {
        if *id == -1 {
            continue;
        }
        let mut departure = *id;
        while departure < earliest_departure {
            departure += id;
        }
        if departure < earliest_bus_departure {
            earliest_bus_id = *id;
            earliest_bus_departure = departure;
        }
    }
    println!(
        "Part 1: {}",
        earliest_bus_id * (earliest_bus_departure - earliest_departure)
    );

    let mut mod_eqs = Vec::<String>::new();
    for i in 0..bus_ids.len() {
        if bus_ids[i] != -1 {
            mod_eqs.push(format!("(t + {}) mod {} = 0", i, bus_ids[i]));
        }
    }
    println!("Part 2 (Give to Wolfram Alpha): {}", mod_eqs.join(", "));
}
