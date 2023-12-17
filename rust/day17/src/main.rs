use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Step {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    pos: (usize, usize),
    prev_step: Step,
    run: usize,
    dist: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then_with(|| self.pos.0.cmp(&other.pos.0))
            .then_with(|| self.pos.1.cmp(&other.pos.1))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn navigate(grid: &Vec<Vec<u32>>, min_run: usize, max_run: usize) -> u32 {
    let dirs = vec![Step::Up, Step::Down, Step::Left, Step::Right];
    let opp_dirs = vec![Step::Down, Step::Up, Step::Right, Step::Left];

    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut distances = HashMap::<((usize, usize), Step, usize), u32>::new();

    let mut to_visit = BinaryHeap::new();
    to_visit.push(Node {
        pos: (0, 0),
        prev_step: Step::Right,
        run: 0,
        dist: 0,
    });

    while let Some(node) = to_visit.pop() {
        if node.pos == (num_rows - 1, num_cols - 1) {
            if node.run >= min_run {
                return node.dist;
            }
            continue;
        }

        let tuple = (node.pos, node.prev_step, node.run);
        if let Some(earlier_dist) = distances.get(&tuple) {
            if *earlier_dist <= node.dist {
                continue;
            }
        }
        distances.insert(tuple, node.dist);

        let (can_turn, must_turn) = if node.run == 0 {
            (true, false)
        } else {
            (node.run >= min_run, node.run >= max_run)
        };

        for i in 0..4 {
            if opp_dirs[i] == node.prev_step {
                continue;
            }

            let dir = dirs[i];
            if !can_turn && dir != node.prev_step {
                continue;
            }
            if must_turn && dir == node.prev_step {
                continue;
            }

            let (valid, delta) = match dir {
                Step::Up => (node.pos.0 > 0, (-1, 0)),
                Step::Down => (node.pos.0 < num_rows - 1, (1, 0)),
                Step::Left => (node.pos.1 > 0, (0, -1)),
                Step::Right => (node.pos.1 < num_cols - 1, (0, 1)),
            };

            if !valid {
                continue;
            }

            let (d_r, d_c) = delta;
            let n_r = (node.pos.0 as i32 + d_r) as usize;
            let n_c = (node.pos.1 as i32 + d_c) as usize;
            let pos = (n_r, n_c);

            let prev_step = dir;
            let run = if prev_step == node.prev_step {
                node.run + 1
            } else {
                1
            };

            let dist = node.dist + grid[n_r][n_c];

            let new_node = Node {
                pos,
                prev_step,
                run,
                dist,
            };

            to_visit.push(new_node);
        }
    }

    panic!()
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 17: Clumsy Crucible");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");

    let grid = parse_input(&raw_input);

    let part_one = navigate(&grid, 0, 3);
    let part_two = navigate(&grid, 4, 10);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
