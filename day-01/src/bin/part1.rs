use std::fs;

fn main() {
    let contents1 = fs::read_to_string("src/input1.txt").expect("Should have been able to read the file");

    //println!("With text:\n{}", contents1);

    let mut sum = 0;

    for line in contents1.lines() {
        sum += get_numbers_from_string(line);

    }

    println!("{}", sum);
}


fn get_numbers_from_string(input: &str) -> u32{
    let mut vec = Vec::new();

    for c in input.chars() {
        if c.is_digit(10) {
            vec.push(c.to_digit(10).unwrap())
        }
    }

    if vec.len() > 1 {
        let first = vec.first().unwrap().to_string();
        let last = vec.last().unwrap().to_string();
        let joined = first + &last;
        let number = joined.parse::<u32>().unwrap();
        return number;
    } else {
        let first = vec.first().unwrap().to_string();
        let joined = first.clone() + &first;
        let number = joined.parse::<u32>().unwrap();
        return number;
    }
}