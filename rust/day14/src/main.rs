use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    Round,
    Cube,
    Empty,
}

fn print_grid(grid: &Vec<Vec<Tile>>) {
    for row in grid.iter() {
        for c in row.iter() {
            let ch = match c {
                Tile::Round => 'O',
                Tile::Cube => '█',
                Tile::Empty => ' ',
            };
            print!("{}", ch);
        }
        println!();
    }
    println!();
}

fn tilt_north(grid: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut new_grid = grid.clone();

    for c in 0..num_cols {
        let mut r = 0;
        while r < num_rows {
            if grid[r][c] == Tile::Empty {
                let mut s_r = r;
                loop {
                    r += 1;
                    if r >= num_rows {
                        break;
                    }
                    match grid[r][c] {
                        Tile::Cube => {
                            break;
                        }
                        Tile::Empty => {}
                        Tile::Round => {
                            new_grid[r][c] = Tile::Empty;
                            new_grid[s_r][c] = Tile::Round;
                            s_r += 1;
                        }
                    }
                }
            }
            r += 1;
        }
    }

    new_grid
}

fn tilt_south(grid: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut new_grid = grid.clone();

    for c in 0..num_cols {
        let mut r = num_rows as isize - 1;
        while r >= 0 {
            if grid[r as usize][c] == Tile::Empty {
                let mut s_r = r;
                loop {
                    r -= 1;
                    if r < 0 {
                        break;
                    }
                    match grid[r as usize][c] {
                        Tile::Cube => {
                            break;
                        }
                        Tile::Empty => {}
                        Tile::Round => {
                            new_grid[r as usize][c] = Tile::Empty;
                            new_grid[s_r as usize][c] = Tile::Round;
                            s_r -= 1;
                        }
                    }
                }
            }
            r -= 1;
        }
    }

    new_grid
}

fn tilt_west(grid: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut new_grid = grid.clone();

    for r in 0..num_rows {
        let mut c = 0;
        while c < num_rows {
            if grid[r][c] == Tile::Empty {
                let mut s_c = c;
                loop {
                    c += 1;
                    if c >= num_cols {
                        break;
                    }
                    match grid[r][c] {
                        Tile::Cube => {
                            break;
                        }
                        Tile::Empty => {}
                        Tile::Round => {
                            new_grid[r][c] = Tile::Empty;
                            new_grid[r][s_c] = Tile::Round;
                            s_c += 1;
                        }
                    }
                }
            }
            c += 1;
        }
    }

    new_grid
}

fn tilt_east(grid: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut new_grid = grid.clone();

    for r in 0..num_rows {
        let mut c = num_cols as isize - 1;
        while c >= 0 {
            if grid[r][c as usize] == Tile::Empty {
                let mut s_c = c;
                loop {
                    c -= 1;
                    if c < 0 {
                        break;
                    }
                    match grid[r][c as usize] {
                        Tile::Cube => {
                            break;
                        }
                        Tile::Empty => {}
                        Tile::Round => {
                            new_grid[r][c as usize] = Tile::Empty;
                            new_grid[r][s_c as usize] = Tile::Round;
                            s_c -= 1;
                        }
                    }
                }
            }
            c -= 1;
        }
    }

    new_grid
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

fn cycle(grid: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    // todo: we could probably do this with rotating and tilting the rotations
    let grid = tilt_north(&grid);
    let grid = tilt_west(&grid);
    let grid = tilt_south(&grid);
    let grid = tilt_east(&grid);

    // let grid = rotate(&grid);
    // let grid = tilt_north(&grid);

    // let grid = rotate(&grid);
    // let grid = tilt_north(&grid);

    // let grid = rotate(&grid);
    // let grid = tilt_north(&grid);

    grid
}

fn solve_part_two(grid: &Vec<Vec<Tile>>, num_cylces: usize) -> usize {
    let mut pressures = HashMap::new();
    let pressure = get_pressure(&grid);

    pressures.insert(pressure, 0);

    let mut grid = grid.clone();

    let mut hist = vec![];
    hist.push(pressure);

    for i in 0..200 {
        grid = cycle(&grid);
        let p = get_pressure(&grid);
        println!("{} {}", i + 1, p);
        // hist.push(p);

        // if pressures.contains_key(&p) {
        //     panic!("rep! {}", i + 1);
        // }

        // pressures.insert(p, i + 1);
    }

    println!("{:?}", hist);

    0
}

// fn tranpose(grid: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
//     let num_rows = grid.len();
//     let num_cols = grid[0].len();

//     let mut trans = vec![vec![Tile::Empty; num_rows]; num_cols];
//     for r in 0..num_rows {
//         for c in 0..num_cols {
//             trans[c][r] = grid[r][c]
//         }
//     }

//     trans
// }

fn main() {
    println!("Advent of Code 2023");
    println!("Day 14: Parabolic Reflector Dish");

    let now = Instant::now();

    let raw_input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let platform = parse_input(&raw_input);
    let tilted = tilt_north(&platform);
    let part_one = get_pressure(&tilted);

    let part_two = solve_part_two(&platform, 1000000000);
    // print_grid(&tilted);

    // let cycled = cycle(&platform);
    // print_grid(&cycled);
    // let cycled = cycle(&platform);
    // print_grid(&cycled);

    // let part_one = summarize_grids(&grids, false);
    // let part_two = summarize_grids(&grids, true);

    println!("Part one: {}", part_one);
    println!("Part two: review output manuall");

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
