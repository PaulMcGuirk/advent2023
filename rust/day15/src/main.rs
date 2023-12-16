use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

fn hash(s: &str) -> usize {
    s.chars()
        .fold(0, |val, ch| (val + (ch as u8 as usize)) * 17 % 256)
}

fn solve(input: &Vec<String>) -> (usize, usize) {
    let mut chk = 0;
    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];

    for line in input.iter() {
        chk += hash(line);
        if line.contains('=') {
            let mut pcs = line.split("=");
            let label = String::from(pcs.next().unwrap());
            let focal_length = pcs.next().unwrap().parse::<usize>().unwrap();
            let box_num = hash(&label);
            let box_ = boxes.get_mut(box_num).unwrap();

            if let Some((i, _)) = box_.iter().enumerate().find(|(_, val)| val.0 == label) {
                box_[i] = (label, focal_length);
            } else {
                box_.push((label, focal_length));
            }
        } else {
            let label = line.split("-").next().unwrap();
            let box_num = hash(&label);
            let box_ = boxes.get_mut(box_num).unwrap();
            if let Some((i, _)) = box_.iter().enumerate().find(|(_, val)| val.0 == label) {
                box_.remove(i);
            }
        }
    }

    let power = boxes
        .into_iter()
        .enumerate()
        .map(|(i, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(j, (_, power))| (i + 1) * (j + 1) * power)
                .sum::<usize>()
        })
        .sum();

    (chk, power)
}

fn parse_input(input: &str) -> Vec<String> {
    input.trim().split(",").map(|s| String::from(s)).collect()
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 15: Lens Library");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let insts = parse_input(&raw_input);

    let (part_one, part_two) = solve(&insts);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
