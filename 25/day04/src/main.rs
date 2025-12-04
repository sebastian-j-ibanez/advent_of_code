use std::io;
use utils::Part;

fn main() {
    let mut grid_1 = parse_input().expect("could not parse input");
    let mut grid_2 = parse_input().expect("could not parse input");
    let part_1 = get_accessible_sum(Part::One, &mut grid_1);
    let part_2 = get_accessible_sum(Part::Two, &mut grid_2);
    println!("{}\n{}", part_1, part_2);
}

fn get_accessible_sum(part: Part, grid: &mut Vec<Vec<char>>) -> u32 {
    match part {
        Part::One => {
            let mut sum = 0;
            for y in 0..grid.len() {
                for x in 0..grid[y].len() {
                    if grid[y][x] == '@' && is_accessible(grid, x, y) {
                        sum += 1;
                    }
                }
            }
            sum
        }
        Part::Two => {
            let mut sum = 0;
            loop {
                for y in 0..grid.len() {
                    for x in 0..grid[y].len() {
                        if grid[y][x] == '@' && is_accessible(grid, x, y) {
                            sum += 1;
                            grid[y][x] = '.';
                        }
                    }
                }

                let still_accessible = grid.clone().iter().enumerate().any(|(y, line)| {
                    line.iter()
                        .enumerate()
                        .filter(|(_, c)| **c == '@')
                        .any(|(x, _)| is_accessible(grid, x, y))
                });

                if !still_accessible {
                    break;
                }
            }

            sum
        }
    }
}

const GRID_LENGTH: usize = 140;

fn is_accessible(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let max_index = GRID_LENGTH - 1;
    let mut adjacent: Vec<char> = Vec::new();
    if y > 0 && x > 0 {
        adjacent.push(grid[y - 1][x - 1]);
    }

    if x > 0 {
        adjacent.push(grid[y][x - 1]);
    }

    if y > 0 {
        adjacent.push(grid[y - 1][x]);
    }

    if y < max_index && x < max_index {
        adjacent.push(grid[y + 1][x + 1]);
    }

    if x < max_index {
        adjacent.push(grid[y][x + 1]);
    }

    if y < max_index {
        adjacent.push(grid[y + 1][x]);
    }

    if y < max_index && x > 0 {
        adjacent.push(grid[y + 1][x - 1]);
    }

    if y > 0 && x < max_index {
        adjacent.push(grid[y - 1][x + 1]);
    }

    let adjacent_count = adjacent.iter().filter(|c| **c == '@').count();
    adjacent_count < 4
}

fn parse_input() -> io::Result<Vec<Vec<char>>> {
    let lines = utils::read_lines("input.txt")?;
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    Ok(grid)
}
