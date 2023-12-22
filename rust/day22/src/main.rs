use std::collections::HashSet;
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

#[derive(Copy, Clone)]
struct Brick {
    min_x: usize,
    min_y: usize,
    min_z: usize,
    max_x: usize,
    max_y: usize,
    max_z: usize,
}

fn solve(bricks: &Vec<Brick>) -> (usize, usize) {
    let bricks = {
        let mut bricks = bricks.clone();
        bricks.sort_by_key(|b| b.min_z);
        bricks
    };

    let (baseline, supported_by) = resolve_falls(&bricks, None);

    let safe_count = (0..bricks.len())
        .filter(|i| !supported_by.iter().any(|s| s.len() == 1 && s.contains(i)))
        .count();

    let total_falls = (0..bricks.len())
        .map(|i| {
            let (fallen, _) = resolve_falls(&bricks, Some(i));
            (0..bricks.len())
                .filter(|&j| j != i && baseline[j].min_z > fallen[j].min_z)
                .count()
        })
        .sum();

    (safe_count, total_falls)
}

fn resolve_falls(bricks: &Vec<Brick>, omit: Option<usize>) -> (Vec<Brick>, Vec<HashSet<usize>>) {
    let x_bound = bricks.iter().map(|b| b.max_x).max().unwrap() + 1;
    let y_bound = bricks.iter().map(|b| b.max_y).max().unwrap() + 1;
    let z_bound = bricks.iter().map(|b| b.max_z).max().unwrap() + 1;

    let mut tiles = vec![vec![vec![Option::<usize>::None; z_bound]; y_bound]; x_bound];
    let mut supported_by = vec![];
    let mut fallen = vec![];

    for (i, brick) in bricks.iter().enumerate() {
        if Some(i) == omit {
            supported_by.push(HashSet::new());
            fallen.push(brick.clone());
            continue;
        }
        let z = (1usize..brick.min_z)
            .rev()
            .find(|&z| {
                (brick.min_x..=brick.max_x)
                    .any(|x| (brick.min_y..=brick.max_y).any(|y| tiles[x][y][z].is_some()))
            })
            .unwrap_or(0)
            + 1;

        let below = (brick.min_x..=brick.max_x)
            .flat_map(|x| {
                (brick.min_y..=brick.max_y)
                    .filter_map(|y| tiles[x][y][z - 1])
                    .collect::<HashSet<_>>()
            })
            .collect::<HashSet<_>>();

        supported_by.push(below);
        let fallen_brick = Brick {
            min_x: brick.min_x,
            min_y: brick.min_y,
            min_z: z,
            max_x: brick.max_x,
            max_y: brick.max_y,
            max_z: z + brick.max_z - brick.min_z,
        };

        for x in fallen_brick.min_x..=fallen_brick.max_x {
            for y in fallen_brick.min_y..=fallen_brick.max_y {
                for z in fallen_brick.min_z..=fallen_brick.max_z {
                    tiles[x][y][z] = Some(i);
                }
            }
        }

        fallen.push(fallen_brick);
    }

    (fallen, supported_by)
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .trim()
        .lines()
        .map(|line| {
            let ends = line
                .split("~")
                .map(|side| {
                    side.trim()
                        .split(",")
                        .map(|v| v.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let min = (0..3)
                .map(|i| ends.iter().map(|end| end[i]).min().unwrap())
                .collect::<Vec<_>>();
            let max = (0..3)
                .map(|i| ends.iter().map(|end| end[i]).max().unwrap())
                .collect::<Vec<_>>();

            Brick {
                min_x: min[0],
                min_y: min[1],
                min_z: min[2],
                max_x: max[0],
                max_y: max[1],
                max_z: max[2],
            }
        })
        .collect::<Vec<_>>()
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 22: Sand Slabs");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let bricks = parse_input(&raw_input);

    let (part_one, part_two) = solve(&bricks);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
