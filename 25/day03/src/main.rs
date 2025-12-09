use std::io;
use utils::{Part, read_lines};

fn main() -> io::Result<()> {
    let batteries = parse_input()?;
    let part_1 = total_output_joltage(Part::One, &batteries);
    let part_2 = total_output_joltage(Part::Two, &batteries);
    println!("{}", part_1);
    println!("{}", part_2);
    Ok(())
}

fn total_output_joltage(part: Part, batteries: &Vec<Vec<u8>>) -> u64 {
    let to_select = match part {
        Part::One => 2,
        Part::Two => 12,
    };
    batteries
        .into_iter()
        .map(|b| max_joltage(b, to_select))
        .sum()
}

const BATTERY_LEN: usize = 100;

fn max_joltage(battery: &Vec<u8>, to_select: usize) -> u64 {
    let mut joltage = Vec::new();
    let mut i = 0;
    while joltage.len() < to_select {
        let needed = to_select - joltage.len();
        let available = BATTERY_LEN - i;
        let window = available - needed + 1;
        let mut best_index = 0;
        let mut best_val = battery[i];
        for j in 0..window {
            if battery[i + j] > best_val {
                best_val = battery[i + j];
                best_index = j;
            }
        }
        let best_in_window = best_index;

        i += best_in_window;
        joltage.push(battery[i]);
        i += 1;
    }

    append_digits(joltage)
}

fn append_digits(joltage: Vec<u8>) -> u64 {
    joltage.into_iter().fold(0, |acc, d| acc * 10 + d as u64)
}

fn parse_input() -> io::Result<Vec<Vec<u8>>> {
    let lines = read_lines("input.txt")?;
    let parsed_lines = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("char could not convert to u8") as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    Ok(parsed_lines)
}
