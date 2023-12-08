use std::collections::HashMap;
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

fn ghost_navigate(insts: &Vec<bool>, nodes: &HashMap<String, (String, String)>) -> u64 {
    let starts = nodes
        .keys()
        .filter(|k| k.ends_with("A"))
        .collect::<Vec<_>>();

    let periods = starts
        .iter()
        .map(|start| find_period(start, insts, nodes) as usize)
        .collect::<Vec<_>>();

    lcm(periods)

    // periods.iter().map(|&p| p as u64).product()
}

fn lcm(periods: Vec<usize>) -> u64 {
    let max = *periods.iter().max().unwrap();
    let mut is_prime = vec![true; max + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut primes = vec![];

    for p in 2..=max {
        if !is_prime[p] {
            continue;
        }

        primes.push(p);

        for j in ((p * p)..=max).step_by(p) {
            is_prime[j] = false;
        }
    }

    let mut powers = vec![0; primes.len()];

    for period in periods.iter() {
        let mut period = *period;

        for (i, &p) in primes.iter().enumerate() {
            let mut power = 0;
            while period % p == 0 {
                power += 1;
                period /= p;
            }

            powers[i] = powers[i].max(power);

            if period == 1 {
                break;
            }
        }
    }

    primes
        .iter()
        .enumerate()
        .filter_map(|(i, &p)| {
            if powers[i] == 0 {
                None
            } else {
                Some((p as u64).pow(powers[i]))
            }
        })
        .product()
}

fn find_period(start: &str, insts: &Vec<bool>, nodes: &HashMap<String, (String, String)>) -> usize {
    (0usize..)
        .scan(String::from(start), |loc, idx| {
            *loc = if insts[idx % insts.len()] {
                String::from(&nodes[loc].1)
            } else {
                String::from(&nodes[loc].0)
            };
            Some(loc.clone())
        })
        .take_while(|loc| !loc.ends_with("Z"))
        .count()
        + 1
}

fn navigate(insts: &Vec<bool>, nodes: &HashMap<String, (String, String)>) -> usize {
    (0usize..)
        .scan(String::from("AAA"), |loc, idx| {
            *loc = if insts[idx % insts.len()] {
                String::from(&nodes[loc].1)
            } else {
                String::from(&nodes[loc].0)
            };
            Some(loc.clone())
        })
        .take_while(|loc| loc != "ZZZ")
        .count()
        + 1
}

fn parse_input(raw_input: &str) -> (Vec<bool>, HashMap<String, (String, String)>) {
    let mut pcs = raw_input.trim().split("\n\n");

    let insts = pcs
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|c| c == 'R')
        .collect::<Vec<_>>();

    let nodes = pcs
        .next()
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            let mut sub_pcs = line.split("=");
            let src = sub_pcs.next().unwrap().trim();
            let mut dests = sub_pcs
                .next()
                .unwrap()
                .split(",")
                .map(|d| d.trim().replace("(", "").replace(")", ""));
            (
                String::from(src),
                (
                    String::from(dests.next().unwrap()),
                    String::from(dests.next().unwrap()),
                ),
            )
        })
        .collect::<HashMap<_, _>>();

    (insts, nodes)
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 8: Haunted Wasteland");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");
    //     let raw_input = "RL

    // AAA = (BBB, CCC)
    // BBB = (DDD, EEE)
    // CCC = (ZZZ, GGG)
    // DDD = (DDD, DDD)
    // EEE = (EEE, EEE)
    // GGG = (GGG, GGG)
    // ZZZ = (ZZZ, ZZZ)";
    //     let raw_input = "LLR

    // AAA = (BBB, BBB)
    // BBB = (AAA, ZZZ)
    // ZZZ = (ZZZ, ZZZ)";

    // let raw_input = "LR

    // 11A = (11B, XXX)
    // 11B = (XXX, 11Z)
    // 11Z = (11B, XXX)
    // 22A = (22B, XXX)
    // 22B = (22C, 22C)
    // 22C = (22Z, 22Z)
    // 22Z = (22B, 22B)
    // XXX = (XXX, XXX)";

    let (insts, nodes) = parse_input(&raw_input);

    let part_one = navigate(&insts, &nodes);
    let part_two = ghost_navigate(&insts, &nodes);
    // let ((concat_time, concat_dist), races) = parse_input(&raw_input);

    // let part_one = races
    //     .iter()
    //     .map(|&(t, d)| count_wins(t, d))
    //     .product::<u64>();
    // let part_two = count_wins(concat_time, concat_dist);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
