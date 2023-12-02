use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

const NUM_COLORS: usize = 3;
const COLORS: &[&str; NUM_COLORS] = &["red", "green", "blue"];

fn parse_input(s: &str) -> Vec<Vec<Vec<u32>>> {
    s.trim().lines().map(|line| {
        let draws = line.trim().split(":").skip(1).next().unwrap();
        draws.split(";").map(|d| {
            let mut cubes = vec![0, 0, 0];
            for pc in d.trim().split(",") {
                let mut sub_pcs = pc.split_whitespace();
                let qty = sub_pcs.next().unwrap().trim().parse::<u32>().unwrap();
                let color = sub_pcs.next().unwrap().trim();
                let idx = COLORS.iter().enumerate().find(|(_, &c)| c == color).unwrap().0;
                cubes[idx] = qty;
            }
            cubes
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>()
}

fn possible_games(games: &Vec<Vec<Vec<u32>>>, cube_set: &Vec<u32>) -> u32 {
    games.iter().enumerate()
        .filter_map(|(idx, draws)| {
            let possible = draws.iter().all(|d| {
                (0..NUM_COLORS).all(|i| d[i] <= cube_set[i])
            });

            if possible {
                Some(idx as u32 + 1)
            } else {
                None
            }
    }).sum()
}

fn total_power(games: &Vec<Vec<Vec<u32>>>) -> u32 {
    games.iter().map(|draws| (0..NUM_COLORS)
        .map(|i| draws.iter()
            .map(|draw| draw[i]).max().unwrap()
        ).product::<u32>()
    ).sum()
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 2: Cube Conundrum");

    let now = Instant::now();

    let input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let games = parse_input(&input);

    let part_one = possible_games(&games, &vec![12, 13, 14]);
    let part_two = total_power(&games);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}