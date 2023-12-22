use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

// fn navigate_big_dump_two(
//     start: (i32, i32),
//     rocks: &HashSet<(i32, i32)>,
//     num_steps: usize,
// ) -> usize {
//     let ds = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
//     let mut reached = HashSet::new();
//     reached.insert(start);

//     for i in 1..=num_steps {
//         let mut next = HashSet::new();

//         for (r, c) in reached {
//             for (d_r, d_c) in ds.iter() {
//                 let n = (r + d_r, c + d_c);
//                 if !rocks.contains(&n) {
//                     next.insert(n);
//                 }
//             }
//         }

//         reached = next;

//         let val = reached.next();

//         if i > 65 && (i - 65) % 131 == 0 && ((i - 65) / 131) % 2 == 0 {
//             println!("{}, {}", (i - 65) / 131, res);
//         }
//     }

//     reached.len()
// }

fn navigate_big(
    num_rows: i32,
    num_cols: i32,
    start: (i32, i32),
    rocks: &HashSet<(i32, i32)>,
    num_steps: usize,
) -> usize {
    let ds = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut prev = HashSet::<(i32, i32)>::new();
    let mut curr = HashSet::<(i32, i32)>::new();
    curr.insert(start);

    let mut res = if num_steps % 2 == 0 { 1 } else { 0 };

    for i in 1..=num_steps {
        let mut next = HashSet::new();

        for (r, c) in curr.iter() {
            for (d_r, d_c) in ds.iter() {
                let (n_r, n_c) = (r + d_r, c + d_c);
                if !rocks.contains(&(n_r.rem_euclid(num_rows), n_c.rem_euclid(num_cols)))
                    && !prev.contains(&(n_r, n_c))
                {
                    next.insert((n_r, n_c));
                }
            }
        }

        if i % 2 == num_steps % 2 {
            res += next.len();
        }

        prev = curr;
        curr = next;
    }

    res
}

fn navigate_big_two(
    num_rows: i32,
    num_cols: i32,
    start: (i32, i32),
    rocks: &HashSet<(i32, i32)>,
    num_steps: usize,
) -> u64 {
    let ds = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    if num_steps % 2 != 0 {
        panic!("not handled");
    }

    // let mut counts = HashMap::new();
    // counts.insert(())

    // let mut curr = HashMap::new();
    // curr.insert(start, 1);

    let mut prev = HashMap::<(i32, i32), u64>::new();
    let mut curr = HashMap::<(i32, i32), u64>::new();
    curr.insert(start, 1);

    let mut res = if num_steps % 2 == 0 { 1 } else { 0 };

    for i in 1..=num_steps {
        let mut next = HashMap::<(i32, i32), u64>::new();

        for ((r, c), count) in curr.iter() {
            for (d_r, d_c) in ds.iter() {
                let (n_r, n_c) = (
                    (r + d_r).rem_euclid(num_rows),
                    (c + d_c).rem_euclid(num_cols),
                );

                if !rocks.contains(&(n_r, n_c)) && !prev.contains_key(&(n_r, n_c)) {
                    let prev_count = *next.get(&(n_r, n_c)).unwrap_or(&0);
                    next.insert((n_r, n_c), prev_count.max(*count));
                }
            }
        }

        if i % 2 == num_steps % 2 {
            res += next.values().sum::<u64>();
        }

        prev = curr;
        curr = next;
    }

    res
}

fn parse_input(input: &str) -> ((i32, i32), (i32, i32), HashSet<(i32, i32)>) {
    let mut start = None;
    let mut rock = HashSet::new();

    let mut r_max = 0;
    let mut c_max = 0;

    for (r, line) in input.trim().lines().enumerate() {
        r_max = r_max.max(r);
        for (c, ch) in line.trim().chars().enumerate() {
            c_max = c_max.max(c);
            match ch {
                '.' => {}
                '#' => {
                    rock.insert((r as i32, c as i32));
                }
                'S' => {
                    start = Some((r as i32, c as i32));
                }
                _ => panic!(),
            }
        }
    }

    ((r_max as i32 + 1, c_max as i32 + 1), start.unwrap(), rock)
}

fn navigate_big_dump(
    num_rows: i32,
    num_cols: i32,
    start: (i32, i32),
    rocks: &HashSet<(i32, i32)>,
    num_steps: usize,
) -> usize {
    let ds = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut prev = HashSet::<(i32, i32)>::new();
    let mut curr = HashSet::<(i32, i32)>::new();
    curr.insert(start);

    let mut res = if num_steps % 2 == 0 { 1 } else { 0 };

    for i in 1..=num_steps {
        let mut next = HashSet::new();

        for (r, c) in curr.iter() {
            for (d_r, d_c) in ds.iter() {
                let (n_r, n_c) = (r + d_r, c + d_c);
                if !rocks.contains(&(n_r.rem_euclid(num_rows), n_c.rem_euclid(num_cols)))
                    && !prev.contains(&(n_r, n_c))
                {
                    next.insert((n_r, n_c));
                }
            }
        }

        if i % 2 == num_steps % 2 {
            res += next.len();
        }

        if i > 65 && (i - 65) % 131 == 0 && ((i - 65) / 131) % 2 == 0 {
            println!("{}, {}", (i - 65) / 131, res);
        }

        prev = curr;
        curr = next;
    }

    res
}

fn main() {
    // println!("Advent of Code 2023");
    // println!("Day 21: Step Counter");

    let now = Instant::now();

    let raw_input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");

    let ((num_rows, num_cols), start, rocks) = parse_input(&raw_input);

    // let part_one = navigate(start, &rocks, 6);
    let tests = vec![6, 10, 50, 100];

    // for i in tests.into_iter() {
    //     println!(
    //         "{} {}",
    //         i,
    //         navigate_big_two(num_rows, num_cols, start, &rocks, i)
    //     );
    // }
    // // let test_one = navigate_big_two(num_rows, num_cols, start, &rocks, 50);
    // println!("{}", test_one);
    // let part_two = navigate_big_two(num_rows, num_cols, start, &rocks, 26501365);
    // system.reset();
    // let part_two = system.find_cycle();

    // println!("Part one: {}", part_one);

    // println!("Part two: {}", part_two);

    navigate_big_dump(num_rows, num_cols, start, &rocks, 26501365);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
