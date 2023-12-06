use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

fn find_bound(time: u64, record: u64, find_min: bool) -> u64 {
    // finds the earliest or latest time the boat can start and beat the record
    // using a binary search
    let (mut lower, mut upper) = if find_min {
        (0, time / 2)
    } else {
        (time / 2, time)
    };

    while upper - lower > 1 {
        if upper == lower {
            panic!();
        }
        let t = (lower + upper) / 2;
        let beats = t * (time - t) > record;

        if beats == find_min {
            upper = t;
        } else {
            lower = t;
        }
    }

    if find_min {
        upper
    } else {
        lower
    }
}

fn count_wins(time: u64, record: u64) -> u64 {
    let t_min = find_bound(time, record, true);
    let t_max = find_bound(time, record, false);

    t_max - t_min + 1
}

fn parse_input(input: &str) -> ((u64, u64), Vec<(u64, u64)>) {
    let data = input
        .trim()
        .lines()
        .map(|ln| ln.split_whitespace().skip(1).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut concat = data.iter().map(|d| d.join("").parse::<u64>().unwrap());

    let concat_time = concat.next().unwrap();
    let concat_dist = concat.next().unwrap();

    let races = data[0]
        .iter()
        .zip(data[1].iter())
        .map(|(t, d)| (t.parse::<u64>().unwrap(), d.parse::<u64>().unwrap()))
        .collect::<Vec<_>>();

    ((concat_time, concat_dist), races)
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 6: Wait For It");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let ((concat_time, concat_dist), races) = parse_input(&raw_input);

    let part_one = races
        .iter()
        .map(|&(t, d)| count_wins(t, d))
        .product::<u64>();
    let part_two = count_wins(concat_time, concat_dist);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
