use regex::Regex;
use std::collections::HashMap;

mod utils;

fn main() {
    let input_lines = utils::read_puzzle_input();
    let passports = load_passports(input_lines);
    println!(
        "Part 1: {}",
        passports
            .iter()
            .filter(|passport| is_passport_valid(&passport))
            .count()
    );
    println!(
        "Part 2: {}",
        passports
            .iter()
            .filter(|passport| is_passport_valid_strict(&passport))
            .count()
    );
}

fn is_passport_valid(passport: &HashMap<String, String>) -> bool {
    return passport.contains_key("ecl")
        && passport.contains_key("pid")
        && passport.contains_key("eyr")
        && passport.contains_key("hcl")
        && passport.contains_key("byr")
        && passport.contains_key("iyr")
        && passport.contains_key("hgt");
}

fn is_passport_valid_strict(passport: &HashMap<String, String>) -> bool {
    if !passport.contains_key("ecl")
        || !passport.contains_key("pid")
        || !passport.contains_key("eyr")
        || !passport.contains_key("hcl")
        || !passport.contains_key("byr")
        || !passport.contains_key("iyr")
        || !passport.contains_key("hgt")
    {
        return false;
    }

    let byr = passport["byr"].parse::<i32>().unwrap();
    if byr < 1920 || byr > 2002 {
        return false;
    }

    let iyr = passport["iyr"].parse::<i32>().unwrap();
    if iyr < 2010 || iyr > 2020 {
        return false;
    }

    let eyr = passport["eyr"].parse::<i32>().unwrap();
    if eyr < 2020 || eyr > 2030 {
        return false;
    }

    let hgt_cm_re = Regex::new(r"^1([5-8][0-9]|9[0-3])cm$").unwrap();
    let hgt_in_re = Regex::new(r"^(59|[6][0-9]|7[0-6])in$").unwrap();
    if !hgt_cm_re.is_match(&passport["hgt"]) && !hgt_in_re.is_match(&passport["hgt"]) {
        return false;
    }

    let hcl_re = Regex::new(r"^#[0-9a-r]{6}$").unwrap();
    if !hcl_re.is_match(&passport["hcl"]) {
        return false;
    }

    let ecl = &passport["ecl"];
    if !ecl.eq("amb")
        && !ecl.eq("blu")
        && !ecl.eq("brn")
        && !ecl.eq("gry")
        && !ecl.eq("grn")
        && !ecl.eq("hzl")
        && !ecl.eq("oth")
    {
        return false;
    }

    let pid_re = Regex::new(r"^[0-9]{9}$").unwrap();
    if !pid_re.is_match(&passport["pid"]) {
        return false;
    }

    return true;
}

fn load_passports(lines: Vec<String>) -> Vec<HashMap<String, String>> {
    let re = Regex::new(r"([a-zA-Z0-9#]+):([a-zA-Z0-9#]+)").unwrap();
    let mut passports = Vec::<HashMap<String, String>>::new();
    let mut passport = HashMap::<String, String>::new();
    for line in lines.iter() {
        if line.is_empty() {
            passports.push(passport.clone());
            passport = HashMap::<String, String>::new();
        }
        for capture in re.captures_iter(line) {
            passport.insert(capture[1].to_string(), capture[2].to_string());
        }
    }
    passports.push(passport.clone());
    return passports;
}
