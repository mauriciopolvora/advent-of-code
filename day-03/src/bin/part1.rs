use std::fs;

fn main() {
    let contents1 =
        fs::read_to_string("src/input1.txt").expect("Should have been able to read the file");

    let numbers = get_engine_parts(get_engine(&contents1));

    let sum: u32 = numbers.iter().sum();
    println!("The sum is {}", sum);
}

fn get_engine(input: &str) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        matrix.push(row);
    }

    matrix
}

fn get_engine_parts(matrix: Vec<Vec<char>>) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    let mut current_number: u32 = 0;

    for i in 0..matrix.len() {
        let mut possible = false;
        for j in 0..matrix[i].len() {
            let c = matrix[i][j];
            if c.is_numeric() {
                if check_surroundings(&matrix, i, j) {
                    possible = true;
                }
                current_number = current_number * 10 + c.to_digit(10).unwrap();
            } else if current_number != 0 {
                if possible {
                    numbers.push(current_number);
                };
                current_number = 0;
                possible = false;
            }
        }
        // Check if there's a number at the end of the row
        if current_number != 0 {
            if possible {
                numbers.push(current_number);
            };
            current_number = 0;
        }
    }
    //println!("{:?}", numbers);
    numbers
}

fn check_surroundings(matrix: &[Vec<char>], i: usize, j: usize) -> bool {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, -1),
        (0, 1),
    ];

    for (di, dj) in directions.iter() {
        let new_i = (i as isize) + di;
        let new_j = (j as isize) + dj;

        if new_i >= 0 && new_j >= 0 {
            if let Some(row) = matrix.get(new_i as usize) {
                if let Some(&c) = row.get(new_j as usize) {
                    if c != '.' && !c.is_numeric() {
                        return true;
                    }
                }
            }
        }
    }

    false
}
