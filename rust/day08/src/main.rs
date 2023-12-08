use std::collections::HashMap;
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

fn ghost_navigate(insts: &Vec<usize>, nodes: &HashMap<String, Vec<String>>) -> u64 {
    nodes
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|start| navigate(start, true, insts, nodes) as u64)
        .fold(1, |res, num| res * num / gcd(res, num)) // lcm calculation
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b > 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn navigate(
    start: &str,
    spooky: bool,
    insts: &Vec<usize>,
    nodes: &HashMap<String, Vec<String>>,
) -> usize {
    let mut loc = start;
    for i in 0.. {
        if loc == "ZZZ" || (spooky && loc.ends_with("Z")) {
            return i;
        }
        loc = &nodes[loc][insts[i % insts.len()]];
    }

    panic!()
}

fn parse_input(raw_input: &str) -> (Vec<usize>, HashMap<String, Vec<String>>) {
    let pcs = raw_input.trim().split("\n\n").collect::<Vec<_>>();

    let insts = pcs[0]
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect::<Vec<_>>();

    let nodes = pcs[1]
        .trim()
        .lines()
        .map(|line| {
            let sub_pcs = line.split("=").collect::<Vec<_>>();
            let src = String::from(sub_pcs[0].trim());
            let dests = sub_pcs[1]
                .split(",")
                .map(|d| String::from(d.trim().replace("(", "").replace(")", "")))
                .collect::<Vec<_>>();
            (src, dests)
        })
        .collect::<HashMap<_, _>>();

    (insts, nodes)
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 8: Haunted Wasteland");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let (insts, nodes) = parse_input(&raw_input);

    let part_one = navigate("AAA", false, &insts, &nodes);
    let part_two = ghost_navigate(&insts, &nodes);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
