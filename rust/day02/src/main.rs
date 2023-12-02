use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

struct CubeSet { red: u32, green: u32, blue: u32}

impl CubeSet {
    fn parse(s: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for pc in s.split(',') {
            let mut sub_pcs = pc.split_whitespace();
            let qty = sub_pcs.next().unwrap().trim().parse::<u32>().unwrap();
            let color = sub_pcs.next().unwrap().trim();

            match color {
                "red" => red = qty,
                "green" => green = qty,
                "blue" => blue = qty,
                _ => panic!()
            }
        }

        Self { red, green, blue }
    }
}

struct Game {
    id: u32,
    draws: Vec<CubeSet>
}

impl Game {
    fn parse(s: &str) -> Self {
        let mut pcs = s.split(":");

        let id = pcs.next().unwrap().split_whitespace().skip(1).last().unwrap().parse::<u32>().unwrap();

        let draws = pcs.next().unwrap().split(";").map(|d| CubeSet::parse(d.trim())).collect::<Vec<_>>();

        Self { id, draws }
    }

    fn possible(&self, cube_set: &CubeSet) -> bool {
        self.draws.iter().all(|d| d.red <= cube_set.red && d.green <= cube_set.green && d.blue <= cube_set.blue)
    }

    fn power(&self) -> u32 {
        let min_red = self.draws.iter().map(|d| d.red).max().unwrap();
        let min_green = self.draws.iter().map(|d| d.green).max().unwrap();
        let min_blue = self.draws.iter().map(|d| d.blue).max().unwrap();

        min_red * min_green * min_blue
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    input.trim().lines()
        .map(|line| Game::parse(line.trim()))
        .collect::<Vec<_>>()
}

fn possible_games(games: &Vec<Game>, cube_set: &CubeSet) -> u32 {
    games.iter().filter_map(|g| if g.possible(cube_set) { Some(g.id) } else { None }).sum()
}

fn total_power(games: &Vec<Game>) -> u32 {
    games.iter().map(|g| g.power()).sum()
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 2: Cube Conundrum");

    let now = Instant::now();

    let input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let games = parse_input(&input);

    let part_one = possible_games(&games, &CubeSet { red: 12, green: 13, blue: 14 });
    let part_two = total_power(&games);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}