use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Step {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    pos: (usize, usize),
    prev_step: Option<Step>,
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
    let size = {
        let num_rows = grid.len();
        let num_cols = grid[0].len();
        assert_eq!(num_rows, num_cols);
        num_rows
    };

    // let mut distance = vec![vec![u32::MAX; size]; size];
    // let mut prev = vec![vec![Option::<(usize, usize)>::None; size]; size];

    // distance[0][0] = 0;

    let mut distances = HashMap::<((usize, usize), Step, usize), u32>::new();
    // let mut distances = HashMap::<(usize, usize), u32>::new();

    let mut to_visit = BinaryHeap::new();
    to_visit.push(Node {
        pos: (0, 0),
        prev_step: None,
        run: 0,
        dist: 0,
    });

    // let mut to_visit = vec![(0usize, 0usize)];

    while let Some(node) = to_visit.pop() {
        // println!("{:?}", node.pos);
        let verbose = node.pos == (0, 2);
        // if verbose {
        //     println!(
        //         "{:?} {:?} {}",
        //         node.prev_step, node.prev_prev_step, node.dist
        //     );
        // }
        if node.pos == (size - 1, size - 1) {
            if node.run >= min_run {
                return node.dist;
            }
            continue;
        }

        if let Some(prev_step) = node.prev_step {
            let tup = (node.pos, prev_step, node.run);
            if let Some(earlier_dist) = distances.get(&tup) {
                if *earlier_dist <= node.dist {
                    continue;
                }
            }
            distances.insert(tup, node.dist);
        }

        // let tup = (node.pos, node.steps_str);
        // if let Some(earlier_dist) = distances.get(&tup) {
        //     if *earlier_dist <= node.dist {
        //         continue;
        //     }
        // }
        // if let Some(earlier_dist) = distances.get(&(node.pos)) {
        //     if *earlier_dist <= node.dist {
        //         continue;
        //     }
        // }

        // if let Some(prev_step) = node.prev_step {
        //     if let Some(prev_prev_step) = node.prev_prev_step {
        //         if let Some(earlier_dist) = distances.get(&(node.pos, prev_step, prev_prev_step)) {
        //             if *earlier_dist < node.dist {
        //                 if verbose {
        //                     println!("rejected! {}<  {}", *earlier_dist, node.dist);
        //                 }
        //                 continue;
        //             } else if verbose {
        //                 println!("okay I guess {} >= {}", *earlier_dist, node.dist)
        //             }
        //         }
        //     }
        // }
        // if verbose {
        //     println!("continued");
        // }

        let deltas = {
            // let rep = if let Some(prev_prev_step) = node.prev_prev_step {
            //     if prev_prev_step == node.prev_step.unwrap() {
            //         Some(prev_prev_step)
            //     } else {
            //         None
            //     }
            // } else {
            //     None
            // };
            // let rep = None;

            let (can_turn, must_turn) = if let Some(_) = node.prev_step {
                (node.run >= min_run, node.run >= max_run)
            } else {
                (true, false)
            };

            let mut deltas: Vec<((i32, i32), Step)> = vec![];
            if node.prev_step != Some(Step::Down)
                && node.pos.0 > 0
                && (node.prev_step != Some(Step::Up) || !must_turn)
                && (node.prev_step == Some(Step::Up) || can_turn)
            {
                deltas.push(((-1, 0), Step::Up));
            }
            if node.prev_step != Some(Step::Up)
                && node.pos.0 < size - 1
                && (node.prev_step != Some(Step::Down) || !must_turn)
                && (node.prev_step == Some(Step::Down) || can_turn)
            {
                deltas.push(((1, 0), Step::Down));
            }
            if node.prev_step != Some(Step::Right)
                && node.pos.1 > 0
                && (node.prev_step != Some(Step::Left) || !must_turn)
                && (node.prev_step == Some(Step::Left) || can_turn)
            {
                deltas.push(((0, -1), Step::Left));
            }
            if node.prev_step != Some(Step::Left)
                && node.pos.1 < size - 1
                && (node.prev_step != Some(Step::Right) || !must_turn)
                && (node.prev_step == Some(Step::Right) || can_turn)
            {
                deltas.push(((0, 1), Step::Right));
            }

            deltas
        };

        for delta in deltas.into_iter() {
            let ((d_r, d_c), step) = delta;
            let n_r = (node.pos.0 as i32 + d_r) as usize;
            let n_c = (node.pos.1 as i32 + d_c) as usize;
            let pos = (n_r, n_c);

            let prev_step = Some(step);
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

            // distances.insert(
            //     (new_node.pos, new_node.prev_step, new_node.prev_prev_step),
            //     new_node.dist,
            // );

            // if let Some(earlier_dist) = distances.get(&(new_node.pos)) {
            //     if *earlier_dist <= node.dist {
            //         continue;
            //     }
            // }
            // distances.insert(new_node.pos, new_node.dist);
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

    let raw_input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");

    let grid = parse_input(&raw_input);

    let part_one = navigate(&grid, 0, 3);
    let part_two = navigate(&grid, 4, 10);
    // let part_two = solve_part_two(size, &grid);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
