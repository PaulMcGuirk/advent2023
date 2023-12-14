use std::collections::HashMap;
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    Round,
    Cube,
    Empty,
}

enum TiltDirection {
    North,
    South,
    East,
    West,
}

fn tilt(grid: &Vec<Vec<Tile>>, dir: TiltDirection) -> Vec<Vec<Tile>> {
    let size = grid.len();

    let mut grid = grid.clone();

    for a in 0..size {
        for b in 0..size {
            let (r, c) = match dir {
                TiltDirection::North => (b, a),
                TiltDirection::West => (a, b),
                TiltDirection::South => (size - 1 - b, a),
                TiltDirection::East => (a, size - 1 - b),
            };
            if grid[r][c] != Tile::Round {
                continue;
            }

            let (mut s_r, mut s_c) = (r as isize, c as isize);
            loop {
                let (n_r, n_c) = match dir {
                    TiltDirection::North => (s_r - 1, s_c),
                    TiltDirection::West => (s_r, s_c - 1),
                    TiltDirection::South => (s_r + 1, s_c),
                    TiltDirection::East => (s_r, s_c + 1),
                };
                if n_r < 0 || n_r >= size as isize || n_c < 0 || n_c >= size as isize {
                    break;
                }
                if grid[n_r as usize][n_c as usize] != Tile::Empty {
                    break;
                }

                (s_r, s_c) = (n_r, n_c);
            }

            grid[r][c] = Tile::Empty;
            grid[s_r as usize][s_c as usize] = Tile::Round;
        }
    }

    grid
}

fn get_key(grid: &Vec<Vec<Tile>>) -> String {
    grid.iter()
        .flat_map(|row| {
            row.iter().map(|r| match r {
                Tile::Round => 'O',
                Tile::Empty => '.',
                Tile::Cube => '#',
            })
        })
        .collect::<String>()
}

fn spin_cycle(grid: &Vec<Vec<Tile>>, cycle_count: usize) -> usize {
    let mut grid = grid.clone();
    let s = get_key(&grid);

    let mut seen = HashMap::new();
    let mut history = vec![];
    history.push(get_pressure(&grid));
    seen.insert(s, 0);

    for i in 1..=cycle_count {
        grid = tilt(&grid, TiltDirection::North);
        grid = tilt(&grid, TiltDirection::West);
        grid = tilt(&grid, TiltDirection::South);
        grid = tilt(&grid, TiltDirection::East);

        let s = get_key(&grid);

        if let Some(&prev_seen) = seen.get(&s) {
            let period = i - prev_seen;
            return history[prev_seen + (cycle_count - prev_seen) % period];
        }

        seen.insert(s, i);
        history.push(get_pressure(&grid));
    }

    panic!("Not found");
}

fn get_pressure(grid: &Vec<Vec<Tile>>) -> usize {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(i, row)| (i + 1) * row.iter().filter(|&&c| c == Tile::Round).count())
        .sum()
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    'O' => Tile::Round,
                    '#' => Tile::Cube,
                    '.' => Tile::Empty,
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 14: Parabolic Reflector Dish");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let platform = parse_input(&raw_input);

    let part_one = get_pressure(&tilt(&platform, TiltDirection::North));
    let part_two = spin_cycle(&platform, 1000000000);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
