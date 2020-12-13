mod utils;

enum Action {
    North,
    South,
    East,
    West,
    LeftTurn,
    RightTurn,
    Forward,
}

enum Direction {
    North,
    South,
    East,
    West,
}

struct NavigationInstruction {
    action: Action,
    value: i32,
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let instructions = load_instructions(utils::read_puzzle_input());
    let mut ship = Point { x: 0, y: 0 };
    let mut ship_direction = Direction::East;
    for instruction in instructions.iter() {
        match instruction.action {
            Action::North => ship.y += instruction.value,
            Action::South => ship.y -= instruction.value,
            Action::East => ship.x += instruction.value,
            Action::West => ship.x -= instruction.value,
            Action::LeftTurn => {
                for _ in 0..(instruction.value / 90) {
                    match ship_direction {
                        Direction::North => ship_direction = Direction::West,
                        Direction::South => ship_direction = Direction::East,
                        Direction::East => ship_direction = Direction::North,
                        Direction::West => ship_direction = Direction::South,
                    }
                }
            }
            Action::RightTurn => {
                for _ in 0..(instruction.value / 90) {
                    match ship_direction {
                        Direction::North => ship_direction = Direction::East,
                        Direction::South => ship_direction = Direction::West,
                        Direction::East => ship_direction = Direction::South,
                        Direction::West => ship_direction = Direction::North,
                    }
                }
            }
            Action::Forward => match ship_direction {
                Direction::North => ship.y += instruction.value,
                Direction::South => ship.y -= instruction.value,
                Direction::East => ship.x += instruction.value,
                Direction::West => ship.x -= instruction.value,
            },
        }
    }
    println!("Part 1: {}", ship.x.abs() + ship.y.abs());

    let mut ship = Point { x: 0, y: 0 };
    let mut waypoint = Point { x: 10, y: 1 };
    for instruction in instructions.iter() {
        match instruction.action {
            Action::North => waypoint.y += instruction.value,
            Action::South => waypoint.y -= instruction.value,
            Action::East => waypoint.x += instruction.value,
            Action::West => waypoint.x -= instruction.value,
            Action::LeftTurn => {
                let old_x = waypoint.x;
                let old_y = waypoint.y;
                let angle_rad = instruction.value as f32 * std::f32::consts::PI / 180.0;
                waypoint.x = ((old_x as f32) * (angle_rad).cos()
                    - (old_y as f32) * (angle_rad).sin())
                .round() as i32;
                waypoint.y = ((old_x as f32) * (angle_rad).sin()
                    + (old_y as f32) * (angle_rad).cos())
                .round() as i32;
            }
            Action::RightTurn => {
                let old_x = waypoint.x;
                let old_y = waypoint.y;
                let angle_rad = instruction.value as f32 * std::f32::consts::PI / 180.0 * -1.0;
                waypoint.x = ((old_x as f32) * (angle_rad).cos()
                    - (old_y as f32) * (angle_rad).sin())
                .round() as i32;
                waypoint.y = ((old_x as f32) * (angle_rad).sin()
                    + (old_y as f32) * (angle_rad).cos())
                .round() as i32;
            }
            Action::Forward => {
                for _ in 0..instruction.value {
                    ship.x += waypoint.x;
                    ship.y += waypoint.y;
                }
            }
        }
    }
    println!("Part 2: {}", ship.x.abs() + ship.y.abs());
}

fn load_instructions(input_lines: Vec<String>) -> Vec<NavigationInstruction> {
    return input_lines
        .iter()
        .map(|line| NavigationInstruction {
            action: match &line[0..1] {
                "N" => Action::North,
                "S" => Action::South,
                "E" => Action::East,
                "W" => Action::West,
                "L" => Action::LeftTurn,
                "R" => Action::RightTurn,
                "F" => Action::Forward,
                _ => panic!("Unrecognized action in input"),
            },
            value: line[1..line.len()].parse::<i32>().unwrap(),
        })
        .collect();
}
