use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

fn extrapolate(vals: &Vec<i32>) -> (i32, i32) {
    if vals.iter().all(|&v| v == 0) {
        (0, 0)
    } else {
        let diffs = vals
            .iter()
            .zip(vals.iter().skip(1))
            .map(|(val, next_val)| next_val - val)
            .collect::<Vec<_>>();
        let (first, last) = extrapolate(&diffs);
        (vals[0] - first, vals[vals.len() - 1] + last)
    }
}

fn solve(vals: &Vec<Vec<i32>>) -> (i32, i32) {
    vals.iter().fold((0, 0), |(first, last), vals| {
        let (a, b) = extrapolate(&vals);
        (first + a, last + b)
    })
}

fn parse_input(raw_input: &str) -> Vec<Vec<i32>> {
    raw_input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|pc| pc.trim().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 9: Mirage Maintenance");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let histories = parse_input(&raw_input);

    let (part_two, part_one) = solve(&histories);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
