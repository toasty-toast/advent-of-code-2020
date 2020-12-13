mod utils;

#[derive(Copy, Clone, PartialEq)]
enum GridCellType {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

fn main() {
    let grid = load_grid(utils::read_puzzle_input());
    let mut working_grid = grid.to_vec();
    loop {
        let (changed, updated_grid) = move_seats_by_neighbor(&working_grid);
        working_grid = updated_grid;
        if !changed {
            break;
        }
    }
    println!("Part 1: {}", count_occupied_seats(&working_grid));

    let mut working_grid = grid.to_vec();
    loop {
        let (changed, updated_grid) = move_seats_by_sight(&working_grid);
        working_grid = updated_grid;
        if !changed {
            break;
        }
    }
    println!("Part 2: {}", count_occupied_seats(&working_grid));
}

fn count_occupied_seats(grid: &Vec<Vec<GridCellType>>) -> i32 {
    return grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| match cell {
                    GridCellType::OccupiedSeat => 1,
                    _ => 0,
                })
                .sum::<i32>()
        })
        .sum();
}

fn move_seats_by_neighbor(grid: &Vec<Vec<GridCellType>>) -> (bool, Vec<Vec<GridCellType>>) {
    let mut new_grid = grid.to_vec();
    let mut cell_changed = false;
    for i in 0..new_grid.len() {
        for j in 0..new_grid[i].len() {
            if grid[i][j] == GridCellType::Floor {
                continue;
            }

            let occupied_adjacent = count_occupied_adjacent(&grid, i, j);
            if grid[i][j] == GridCellType::EmptySeat && occupied_adjacent == 0 {
                new_grid[i][j] = GridCellType::OccupiedSeat;
                cell_changed = true;
            } else if grid[i][j] == GridCellType::OccupiedSeat && occupied_adjacent >= 4 {
                new_grid[i][j] = GridCellType::EmptySeat;
                cell_changed = true;
            }
        }
    }
    return (cell_changed, new_grid);
}

fn count_occupied_adjacent(grid: &Vec<Vec<GridCellType>>, row: usize, col: usize) -> i32 {
    let mut occupied_adjacent = 0;
    let cells_to_check: [(i32, i32); 8] = [
        (row as i32 - 1, col as i32 - 1),
        (row as i32 - 1, col as i32),
        (row as i32 - 1, col as i32 + 1),
        (row as i32, col as i32 - 1),
        (row as i32, col as i32 + 1),
        (row as i32 + 1, col as i32 - 1),
        (row as i32 + 1, col as i32),
        (row as i32 + 1, col as i32 + 1),
    ];
    for (i, j) in cells_to_check.iter() {
        if *i < 0 || *i as usize > grid.len() - 1 || *j < 0 || *j as usize > grid[row].len() - 1 {
            continue;
        }
        match grid[*i as usize][*j as usize] {
            GridCellType::OccupiedSeat => occupied_adjacent += 1,
            _ => {}
        }
    }
    return occupied_adjacent;
}

fn move_seats_by_sight(grid: &Vec<Vec<GridCellType>>) -> (bool, Vec<Vec<GridCellType>>) {
    let mut new_grid = grid.to_vec();
    let mut cell_changed = false;
    for i in 0..new_grid.len() {
        for j in 0..new_grid[i].len() {
            if grid[i][j] == GridCellType::Floor {
                continue;
            }

            let occupied = count_occupied_by_sight(grid, i, j);
            if grid[i][j] == GridCellType::EmptySeat && occupied == 0 {
                new_grid[i][j] = GridCellType::OccupiedSeat;
                cell_changed = true;
            } else if grid[i][j] == GridCellType::OccupiedSeat && occupied >= 5 {
                new_grid[i][j] = GridCellType::EmptySeat;
                cell_changed = true;
            }
        }
    }
    return (cell_changed, new_grid);
}

fn count_occupied_by_sight(grid: &Vec<Vec<GridCellType>>, row: usize, col: usize) -> i32 {
    let funcs = [
        |i: i32, j: i32| return (i - 1, j - 1),
        |i: i32, j: i32| return (i - 1, j),
        |i: i32, j: i32| return (i - 1, j + 1),
        |i: i32, j: i32| return (i, j - 1),
        |i: i32, j: i32| return (i, j + 1),
        |i: i32, j: i32| return (i + 1, j - 1),
        |i: i32, j: i32| return (i + 1, j),
        |i: i32, j: i32| return (i + 1, j + 1),
    ];

    let mut occupied = 0;
    for func in funcs.iter() {
        let (mut i, mut j) = func(row as i32, col as i32);
        while i >= 0 && i < grid.len() as i32 && j >= 0 && j < grid[0].len() as i32 as i32 {
            match grid[i as usize][j as usize] {
                GridCellType::OccupiedSeat => {
                    occupied += 1;
                    break;
                }
                GridCellType::EmptySeat => break,
                _ => {}
            }
            let (new_i, new_j) = func(i, j);
            i = new_i;
            j = new_j;
        }
    }
    return occupied;
}

fn load_grid(lines: Vec<String>) -> Vec<Vec<GridCellType>> {
    return lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => GridCellType::Floor,
                    'L' => GridCellType::EmptySeat,
                    '#' => GridCellType::OccupiedSeat,
                    _ => panic!("Unknown character in input."),
                })
                .collect()
        })
        .collect();
}
