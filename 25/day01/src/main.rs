use std::io;
use utils::{Part, read_lines};

fn main() -> io::Result<()> {
    let raw_lines = read_lines("input.txt")?;
    let part_1 = get_password(Part::One, raw_lines.clone())?;
    let part_2 = get_password(Part::Two, raw_lines)?;
    println!("part 1: {part_1}");
    println!("part 2: {part_2}");
    Ok(())
}

struct RingBuffer {
    index: u32,
    zero_sum: u32,
}

impl RingBuffer {
    fn new() -> Self {
        RingBuffer {
            index: 50,
            zero_sum: 0,
        }
    }

    fn left(&mut self) {
        if self.index == 0 {
            self.index = 99;
        } else {
            self.index -= 1;
        }
    }

    fn right(&mut self) {
        if self.index == 99 {
            self.index = 0;
        } else {
            self.index += 1;
        }
    }

    fn rotate_part1(&mut self, direction: String, distance: u32) {
        for _ in 0..distance {
            match direction.as_str() {
                "L" => self.left(),
                "R" => self.right(),
                s => panic!("expected L or R, got {}", s),
            }
        }
        if self.index == 0 {
            self.zero_sum += 1;
        }
    }

    fn rotate_part2(&mut self, direction: String, distance: u32) {
        for _ in 0..distance {
            match direction.as_str() {
                "L" => self.left(),
                "R" => self.right(),
                s => panic!("expected L or R, got {}", s),
            }
            if self.index == 0 {
                self.zero_sum += 1;
            }
        }
    }
}

fn get_password(part: Part, raw_lines: Vec<String>) -> io::Result<u32> {
    let lines = parse_lines(raw_lines);
    let mut ring_buf = RingBuffer::new();

    let op = match part {
        Part::One => RingBuffer::rotate_part1,
        Part::Two => RingBuffer::rotate_part2,
    };
    for line in lines {
        op(&mut ring_buf, line.0, line.1);
    }

    Ok(ring_buf.zero_sum)
}

fn parse_lines(raw_lines: Vec<String>) -> Vec<(String, u32)> {
    raw_lines
        .iter()
        .map(|l| {
            let l = l.split_at(1);
            (l.0.to_string(), l.1.parse().expect("not a valid number"))
        })
        .collect::<Vec<(String, u32)>>()
}
