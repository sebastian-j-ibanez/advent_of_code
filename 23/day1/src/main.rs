use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

// Mapping of number strings to numeric literals.
const NUMBERS: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

fn main() -> io::Result<()> {
    let file = File::open("input.txt").expect("unable to read input file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let numbers = get_numbers(lines, true);
    let sum: u32 = numbers
        .iter()
        .map(|row| zip_digits(*row.first().unwrap(), *row.last().unwrap()))
        .sum();

    println!("part 1 result: {}", sum);
    Ok(())
}

// Filter and collect numbers in a line.
fn get_numbers(mut lines: Vec<String>, part_2: bool) -> Vec<Vec<u32>> {
    lines
        .iter_mut()
        .map(|line| {
            if part_2 {
                replace_number_str(line);
            }
            line.chars()
                .filter(|c| c.is_numeric())
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

// Make number from 2 digits. (eg. 2, 1 => 21).
fn zip_digits(a: u32, b: u32) -> u32 {
    (10 * a) + b
}

// Replace number strings with numeric literal (one -> 1, two -> 2).
fn replace_number_str(line: &mut String) {
    for number in NUMBERS {
        if line.contains(number.0) {
            *line = line.replace(number.0, &format!("{}{}{}", number.0, number.1, number.0));
        }
    }
}
