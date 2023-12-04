use std::fs;

fn main() {
    let contents =
        fs::read_to_string("src/input1.txt").expect("Should have been able to read the file");
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut sum_gear_ratios = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '*' {
                if let Some((num1, num1_row, num1_col, num2, num2_row, num2_col)) =
                    find_adjacent_parts(&grid, row, col)
                {
                    sum_gear_ratios += num1 * num2;
                    println!(
                        "Gear at ({}, {}): Numbers {} at ({}, {}) and {} at ({}, {})",
                        row, col, num1, num1_row, num1_col, num2, num2_row, num2_col
                    );
                }
            }
        }
    }

    println!("Total sum of all gear ratios: {}", sum_gear_ratios);
}

fn find_adjacent_parts(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
) -> Option<(i64, usize, usize, i64, usize, usize)> {
    let directions = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    let mut parts = Vec::new();

    for &(dr, dc) in &directions {
        let (new_row, new_col) = (row as isize + dr, col as isize + dc);

        if new_row >= 0
            && new_col >= 0
            && new_row < grid.len() as isize
            && new_col < grid[new_row as usize].len() as isize
        {
            let (new_row, new_col) = (new_row as usize, new_col as usize);

            if grid[new_row][new_col].is_numeric() {
                let number = extract_number(grid, new_row, new_col);
                if !parts.iter().any(|&(n, _, _)| n == number) {
                    parts.push((number, new_row, new_col));
                }
            }
        }
    }

    if parts.len() == 2 {
        Some((
            parts[0].0, parts[0].1, parts[0].2, parts[1].0, parts[1].1, parts[1].2,
        ))
    } else {
        None
    }
}

fn extract_number(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i64 {
    let horizontal_number = extract_number_in_direction(grid, row, col, 0, 1);
    let vertical_number = extract_number_in_direction(grid, row, col, 1, 0);

    std::cmp::max(horizontal_number, vertical_number)
}

fn extract_number_in_direction(
    grid: &Vec<Vec<char>>,
    mut row: usize,
    mut col: usize,
    row_step: isize,
    col_step: isize,
) -> i64 {
    let mut number = 0i64;

    // Move to the start of the number (if we're not already there)
    while row as isize - row_step >= 0
        && col as isize - col_step >= 0
        && grid[(row as isize - row_step) as usize][(col as isize - col_step) as usize].is_numeric()
    {
        row = (row as isize - row_step) as usize;
        col = (col as isize - col_step) as usize;
    }

    // Extract the number
    while row < grid.len() && col < grid[row].len() && grid[row][col].is_numeric() {
        number = number * 10 + grid[row][col].to_digit(10).unwrap() as i64;
        row = (row as isize + row_step) as usize;
        col = (col as isize + col_step) as usize;
    }

    number
}
