use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

mod utils;

struct Bag {
    color: String,
    contents: HashMap<String, i32>,
}

fn main() {
    let bags = load_bags(utils::read_puzzle_input());
    println!(
        "Part 1: {}",
        count_options(
            &bags,
            &"shiny gold".to_string(),
            &mut HashSet::<String>::new()
        )
    );
    println!(
        "Part 2: {}",
        count_contents(
            &bags,
            bags.iter()
                .filter(|bag| bag.color.eq("shiny gold"))
                .next()
                .unwrap()
        )
    );
}

fn count_options(bags: &Vec<Bag>, target: &String, seen: &mut HashSet<String>) -> i32 {
    let mut count = 0;
    for bag in bags.iter() {
        if bag.contents.contains_key(target) {
            if !seen.contains(&bag.color) {
                count += 1;
            }
            seen.insert(bag.color.to_owned());
            count += count_options(bags, &bag.color, seen);
        }
    }
    return count;
}

fn count_contents(bags: &Vec<Bag>, target_bag: &Bag) -> i32 {
    let mut total = 0;
    for (color, count) in target_bag.contents.iter() {
        total += count
            + count
                * count_contents(
                    bags,
                    bags.iter()
                        .filter(|bag| bag.color.eq(color))
                        .next()
                        .unwrap(),
                );
    }
    return total;
}

fn load_bags(lines: Vec<String>) -> Vec<Bag> {
    let container_re = Regex::new(r"^([[:alpha:]]+\s[[:alpha:]]+) bags contain").unwrap();
    let contents_re = Regex::new(r"(\d+)\s([[:alpha:]]+\s[[:alpha:]]+)\sbags?").unwrap();
    let mut bags = Vec::<Bag>::new();
    for line in lines.iter() {
        let mut bag_contents = HashMap::<String, i32>::new();
        for cap in contents_re.captures_iter(line) {
            bag_contents.insert(cap[2].to_string(), cap[1].parse::<i32>().unwrap());
        }
        bags.push(Bag {
            color: container_re.captures(line).unwrap()[1].to_string(),
            contents: bag_contents,
        });
    }
    return bags;
}
