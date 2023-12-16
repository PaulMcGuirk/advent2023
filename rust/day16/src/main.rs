use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

fn trace(
    size: i32,
    grid: &HashMap<(i32, i32), char>,
    start_pos: (i32, i32),
    start_loc: (i32, i32),
) -> usize {
    let mut visited = HashMap::<(i32, i32), HashSet<(i32, i32)>>::new();
    visited.insert((0, 1), HashSet::new());
    visited.insert((0, -1), HashSet::new());
    visited.insert((1, 0), HashSet::new());
    visited.insert((-1, 0), HashSet::new());

    let mut starts = vec![(start_pos, start_loc)];

    while let Some(start) = starts.pop() {
        let (mut pos, mut dir) = start;

        loop {
            let next = (pos.0 + dir.0, pos.1 + dir.1);
            if next.0 < 0 || next.1 < 0 || next.0 >= size || next.1 >= size {
                break;
            }

            if visited[&dir].contains(&next) {
                break;
            }

            visited.get_mut(&dir).unwrap().insert(next.clone());

            pos = next;

            let (d_r, d_c) = dir;

            dir = match grid[&next] {
                '-' if d_c == 0 => {
                    starts.push((pos, (0, 1)));
                    (0, -1)
                }
                '|' if d_r == 0 => {
                    starts.push((pos, (1, 0)));
                    (-1, 0)
                }
                '/' => (-d_c, -d_r),
                '\\' => (d_c, d_r),
                _ => dir,
            }
        }
    }

    visited
        .values()
        .flat_map(|v| v)
        .collect::<HashSet<_>>()
        .len()
}

fn solve_part_one(size: i32, grid: &HashMap<(i32, i32), char>) -> usize {
    trace(size, grid, (0, -1), (0, 1))
}

fn solve_part_two(size: i32, grid: &HashMap<(i32, i32), char>) -> usize {
    (0..size)
        .map(|s| {
            let vals = vec![
                trace(size, &grid, (s, -1), (0, 1)),
                trace(size, &grid, (s, size), (0, -1)),
                trace(size, &grid, (-1, s), (1, 0)),
                trace(size, &grid, (size, s), (-1, 0)),
            ];
            vals.into_iter().max().unwrap()
        })
        .max()
        .unwrap()
}

fn parse_input(input: &str) -> (i32, HashMap<(i32, i32), char>) {
    let grid = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(c, ch)| ((r as i32, c as i32), ch))
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<(i32, i32), char>>();

    let size = {
        let &max_r = grid.keys().map(|(r, _)| r).max().unwrap();
        let &max_c = grid.keys().map(|(r, _)| r).max().unwrap();
        assert_eq!(max_c, max_r);
        max_r + 1
    };

    (size as i32, grid)
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 16: The Floor Will Be Lava");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");

    let (size, grid) = parse_input(&raw_input);

    let part_one = solve_part_one(size, &grid);
    let part_two = solve_part_two(size, &grid);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
