use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";
const TARGETS: &[&str] = &[
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn sum_codes(lines: &Vec<&str>, with_spelled: bool) -> u32 {
    let take = if with_spelled { 20 } else { 10 };
    let targets = (0..take).map(|i| (TARGETS[i], i as u32 % 10)).collect::<Vec<_>>();
    
    lines.iter().map(|&ln| {
        let locs = targets.iter()
            .filter_map(|&(target, val)| {
                if let Some(first) = ln.find(target) {
                    let last = ln.rfind(target).unwrap();
                    Some((first, last, val))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let tens = locs.iter().min_by_key(|(first, _, _)| first).unwrap().2;
        let ones = locs.iter().max_by_key(|(_, last, _)| last).unwrap().2;

        10 * tens + ones
    })
    .sum()
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 1: Trebuchet?!");

    let now = Instant::now();

    let input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let lines = input.trim().lines().collect::<Vec<_>>();

    let part_one = sum_codes(&lines, false);
    let part_two = sum_codes(&lines, true);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}