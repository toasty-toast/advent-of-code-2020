mod utils;

fn main() {
    let input_lines = utils::read_puzzle_input();
    let grid: Vec<Vec<char>> = input_lines
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    println!("Part 1: {}", count_trees(&grid, 3, 1));
    println!(
        "Part 2: {}",
        count_trees(&grid, 1, 1)
            * count_trees(&grid, 3, 1)
            * count_trees(&grid, 5, 1)
            * count_trees(&grid, 7, 1)
            * count_trees(&grid, 1, 2)
    );
}

fn count_trees(grid: &Vec<Vec<char>>, step_right: usize, step_down: usize) -> i64 {
    let mut col = 0;
    let mut tree_count = 0;
    for row in (0..grid.len()).step_by(step_down) {
        if grid[row][col] == '#' {
            tree_count += 1;
        }
        col = (col + step_right) % grid[0].len();
    }
    return tree_count;
}
