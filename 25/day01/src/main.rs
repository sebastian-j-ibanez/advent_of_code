use std::io;
use utils::{Part, read_lines};

pub type Instruction = (String, u32);

const END_POS: u32 = 99;
const START_POS: u32 = 50;

fn main() -> io::Result<()> {
    let raw_lines = read_lines("input.txt")?;
    let instructions = parse_lines(raw_lines);
    let part_1 = get_password(Part::One, &instructions)?;
    let part_2 = get_password(Part::Two, &instructions)?;
    println!("part 1: {part_1}");
    println!("part 2: {part_2}");
    Ok(())
}

fn get_password(part: Part, instructions: &Vec<Instruction>) -> io::Result<u32> {
    let mut ring_buf = RingBuffer::new();
    for i in instructions {
        ring_buf.rotate(&part, &i.0, i.1);
    }
    Ok(ring_buf.zero_count)
}

struct RingBuffer {
    index: u32,
    zero_count: u32,
}

impl RingBuffer {
    fn new() -> Self {
        RingBuffer {
            index: START_POS,
            zero_count: 0,
        }
    }

    fn rotate(&mut self, part: &Part, direction: &str, distance: u32) {
        for _ in 0..distance {
            match direction {
                "L" => self.left(),
                "R" => self.right(),
                s => panic!("expected L or R, got {}", s),
            }
            if let Part::Two = part {
                self.check_and_increment();
            }
        }
        if let Part::One = part {
            self.check_and_increment();
        }
    }

    fn left(&mut self) {
        if self.index == 0 {
            self.index = END_POS;
        } else {
            self.index -= 1;
        }
    }

    fn right(&mut self) {
        if self.index >= END_POS {
            self.index = 0;
        } else {
            self.index += 1;
        }
    }

    fn check_and_increment(&mut self) {
        if self.index == 0 {
            self.zero_count += 1;
        }
    }
}

fn parse_lines(raw_lines: Vec<String>) -> Vec<Instruction> {
    raw_lines
        .iter()
        .map(|l| {
            let l = l.split_at(1);
            (l.0.to_string(), l.1.parse().expect("not a valid number"))
        })
        .collect::<Vec<(String, u32)>>()
}
