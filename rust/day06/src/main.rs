use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

fn num_wins(time: u64, record: u64) -> u64 {
    let tf = time as f64;
    let df = record as f64;

    // distance traveled is t * (T - t) where t is the waiting time and T is the total race time
    // use the quadartic equation to find the bounds
    let ta = (tf - ((tf * tf) - 4.0 * df).sqrt()) / 2.0;
    let tb = (tf + ((tf * tf) - 4.0 * df).sqrt()) / 2.0;

    let ta = ta.ceil() as u64;
    let tb = tb.ceil() as u64;

    // do a little search around the above just in case we run into floating point error
    let ta = ((ta - 1)..=(ta + 1))
        .find(|t| t * (time - t) > record)
        .unwrap();
    let tb = ((tb - 1)..=(tb + 1))
        .rev()
        .find(|t| t * (time - t) > record)
        .unwrap();

    // [ta, tb] is the range of times that will run the race
    tb - ta + 1
}

fn parse_input(input: &str) -> ((u64, u64), Vec<(u64, u64)>) {
    let data = input
        .trim()
        .lines()
        .map(|ln| ln.split_whitespace().skip(1).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let concat_time = data[0].join("").parse::<u64>().unwrap();
    let concat_dist = data[1].join("").parse::<u64>().unwrap();

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

    let part_one = races.iter().map(|&(t, d)| num_wins(t, d)).product::<u64>();
    let part_two = num_wins(concat_time, concat_dist);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
