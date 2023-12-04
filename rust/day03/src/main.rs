use std::collections::HashMap;
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

#[derive(PartialEq, Eq)]
enum Cell {
    Digit(u32),
    Gear,
    OtherSymbol,
    Empty
}

fn parse_input(s: &str) -> Vec<Vec<Cell>> {
    s.trim().lines()
        .map(|line| line.trim().chars().map(|c| match c {
            '.' => Cell::Empty,
            ch if ch.is_digit(10) => Cell::Digit(ch.to_digit(10).unwrap() as u32),
            '*' => Cell::Gear,
            _ => Cell::OtherSymbol
        }).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn analyze_engine(cells: &Vec<Vec<Cell>>) -> (u32, u32) {
    let num_rows = cells.len();
    let num_cols = cells[0].len();

    let mut part_num_sum = 0;
    let mut gears = HashMap::<(usize, usize), Vec<u32>>::new();

    for r in 0..num_rows {
        
        let mut digits = vec![];

        for c in 0..num_cols {
            let cell = &cells[r][c];

            if let Cell::Digit(d) = cell {
                digits.push(d);
                if c < num_cols - 1 {
                    continue;
                }
            }

            if digits.is_empty() {
                continue;
            }

            // the number could have ended because we hit a non-number or
            // because we're out of space. adjust the end accordingly
            let end = match cell {
                Cell::Digit(_) => c,
                _ => c - 1
            };

            let start = end - (digits.len() - 1);

            // start and end line up with the digits. try to
            // extend beyond to include digagonals
            let end = if end < num_cols - 1 { end + 1 } else { end };
            let start = if start > 0 { start - 1 } else { start };

            let to_check = {
                // the two end caps here might actually be within the digits, but that's okay
                let mut to_check = vec![(r, start), (r, end)];
                if r > 0 {
                    to_check.extend((start..=end).map(|d_c| (r - 1, d_c)));
                }
                if r < num_rows - 1 {
                    to_check.extend((start..=end).map(|d_c| (r + 1, d_c)));
                }
                to_check
            };

            let code = digits.into_iter().fold(0u32, |acc, d| 10 * acc + d);
            digits = vec![];

            let mut adjacent = false;
            for pair in to_check.into_iter() {
                let (d_r, d_c) = pair;
                match cells[d_r][d_c] {
                    Cell::OtherSymbol => adjacent = true,
                    Cell::Gear => {
                        adjacent = true;
                        if !gears.contains_key(&pair) {
                            gears.insert(pair, vec![]);
                        }
                        gears.get_mut(&pair).unwrap().push(code)
                    },
                    _ => {}
                }
            }

            if adjacent {
                part_num_sum += code;
            }
        }
    }

    let gear_ratio_sum = gears.into_iter()
        .filter_map(|(_, codes)| {
            if codes.len() != 2 {
                None
            } else {
                Some(codes[0] * codes[1])
            }
        }).sum();

    (part_num_sum, gear_ratio_sum)
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 3: Gear Ratios");

    let now = Instant::now();

    let input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let cells = parse_input(&input);

    let (part_one, part_two) = analyze_engine(&cells);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}