use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

struct Instruction {
    dir: Direction,
    dist: i64,
}

struct DigPlan {
    insts: Vec<Instruction>,
}

impl DigPlan {
    fn from_str(input: &str) -> Self {
        let insts = input
            .trim()
            .lines()
            .map(|line| {
                let mut pcs = line.trim().split_whitespace();

                let dir = match pcs.next().unwrap() {
                    "R" => Direction::Right,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "U" => Direction::Up,
                    s => panic!("Unknown direction {}", s),
                };

                let dist = pcs.next().unwrap().parse::<i64>().unwrap();

                Instruction { dir, dist }
            })
            .collect::<Vec<_>>();

        Self { insts }
    }

    fn from_str_elvish(input: &str) -> Self {
        let insts = input
            .trim()
            .lines()
            .map(|line| {
                let pcs = line.trim().split_whitespace();
                let inst = pcs
                    .last()
                    .unwrap()
                    .replace("(", "")
                    .replace(")", "")
                    .replace("#", "");

                let dir = match inst.chars().last().unwrap() {
                    '0' => Direction::Right,
                    '1' => Direction::Down,
                    '2' => Direction::Left,
                    '3' => Direction::Up,
                    s => panic!("Unknown direction {}", s),
                };

                let dist = i64::from_str_radix(&inst[0..5], 16).unwrap();

                Instruction { dir, dist }
            })
            .collect::<Vec<_>>();

        Self { insts }
    }

    fn volume(&self) -> i64 {
        let verticies = {
            let mut pt = (0i64, 0i64);
            let mut vertices = vec![pt];

            for inst in self.insts.iter() {
                let (d_r, d_c) = match inst.dir {
                    Direction::Down => (1, 0),
                    Direction::Left => (0, 1),
                    Direction::Right => (0, -1),
                    Direction::Up => (-1, 0),
                };
                pt = (pt.0 + d_r * inst.dist, pt.1 + d_c * inst.dist);
                if pt != (0, 0) {
                    if vertices.iter().any(|&q| q == pt) {
                        panic!("Self intersecting")
                    }

                    vertices.push(pt)
                }
            }
            vertices
        };

        let lines = (0..verticies.len())
            .map(|i| (verticies[i], verticies[(i + 1) % verticies.len()]))
            .collect::<Vec<_>>();
        let perim = lines
            .iter()
            .map(|(q, p)| (p.0 - q.0).abs() + (p.1 - q.1).abs() - 1)
            .sum::<i64>()
            + verticies.len() as i64;

        let area = lines
            .iter()
            .map(|(p1, p2)| {
                let &(r1, c1) = p1;
                let &(r2, c2) = p2;
                (c1 * r2) as i64 - (c2 * r1) as i64 // shoelace formula
            })
            .sum::<i64>()
            .abs()
            / 2;

        area + 1 + perim / 2 // pick's formula (and add the perimeter)
    }
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 18: Lavaduct Lagoon");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");

    let part_one = DigPlan::from_str(&raw_input).volume();
    let part_two = DigPlan::from_str_elvish(&raw_input).volume();

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
