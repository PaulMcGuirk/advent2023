use std::collections::HashMap;
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

fn hash(s: &str) -> usize {
    s.chars()
        .fold(0, |val, ch| (val + (ch as u8 as usize)) * 17 % 256)
}

fn parse_input(input: &str) -> Vec<String> {
    input.trim().split(",").map(|s| String::from(s)).collect()
}

fn solve_part_two(input: &Vec<String>) -> usize {
    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];

    for line in input.iter() {
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
                println!("removing {}", label);
                box_.remove(i);
            }
        }
    }

    for (i, lenses) in boxes.iter().enumerate() {
        if lenses.is_empty() {
            continue;
        }
        print!("Box {} ", i);
        for (labe, focal) in lenses.iter() {
            print!("[{} {}] ", labe, focal);
        }
        println!();
    }

    boxes
        .into_iter()
        .enumerate()
        .map(|(i, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(j, (_, power))| (i + 1) * (j + 1) * power)
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 15: Lens Library");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");
    // let raw_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let insts = parse_input(&raw_input);

    let part_one = insts.iter().map(|s| hash(s)).sum::<usize>();
    let part_two = solve_part_two(&insts);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
