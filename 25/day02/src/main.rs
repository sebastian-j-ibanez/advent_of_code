use std::io;
use std::ops::Range;
use utils::{Part, read_lines};

fn main() -> io::Result<()> {
    let mut ranges_1 = input_to_ranges()?;
    let part_1 = get_invalid_ids(Part::One, &mut ranges_1);
    let mut ranges_2 = input_to_ranges()?;
    let part_2 = get_invalid_ids(Part::Two, &mut ranges_2);
    println!("part 1: {part_1}");
    println!("part 2: {part_2}");
    Ok(())
}

fn get_invalid_ids(part: Part, ranges: &mut Vec<Range<u64>>) -> u64 {
    ranges
        .iter_mut()
        .map(|r| check_range(&part, r))
        .sum::<u64>()
}

fn check_range(part: &Part, range: &mut Range<u64>) -> u64 {
    let is_invalid = match part {
        Part::One => is_invalid_part1,
        Part::Two => is_invalid_part2,
    };

    range.into_iter().filter(|id: &u64| is_invalid(*id)).sum()
}

fn is_invalid_part1(id: u64) -> bool {
    let s_id = id.to_string();
    let middle = s_id.len().div_ceil(2);
    let (first, second) = s_id.split_at(middle);
    first == second
}

fn is_invalid_part2(id: u64) -> bool {
    let s_id = id.to_string();
    let middle = s_id.len() / 2;
    for i in 0..middle {
        let chars: Vec<char> = s_id.chars().collect();
        let chunks: Vec<&[char]> = chars.chunks(i + 1).collect();
        if chunks.iter().all(|&chunk| chunk == chunks[0]) {
            return true;
        }
    }
    false
}

fn input_to_ranges() -> io::Result<Vec<Range<u64>>> {
    let line = read_lines("input.txt")?;
    let str_ranges = line[0].split_terminator(',');
    let ranges: Vec<Range<u64>> = str_ranges
        .map(|r| {
            let range = r.split_once('-').expect("expected integer");
            let lower: u64 = range.0.parse().expect("expected integer");
            let upper: u64 = range.1.parse().expect("expected integer");
            lower..(upper + 1)
        })
        .collect();

    Ok(ranges)
}
