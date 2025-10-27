use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("input.txt").expect("unable to read input file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let numbers: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| line
            .chars()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>())
        .collect();

    let mut sum = 0;
    for number_line in numbers {
        sum += zip_digits(*number_line.first().unwrap(), *number_line.last().unwrap());
    }

    println!("part 1 result: {}", sum);
    Ok(())
}

// Make number from 2 digits. (eg. 2, 1 => 21).
fn zip_digits(a: u32, b: u32) -> u32 {
    (10 * a) + b
}
