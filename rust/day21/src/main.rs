use std::collections::HashSet;
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

fn navigate(start: (i32, i32), rocks: &HashSet<(i32, i32)>, num_steps: usize) -> usize {
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
                if !rocks.contains(&(n_r, n_c)) && !prev.contains(&(n_r, n_c)) {
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

fn navigate_infinite(
    num_rows: i32,
    num_cols: i32,
    start: (i32, i32),
    rocks: &HashSet<(i32, i32)>,
    num_steps: i32,
) -> i64 {
    // this solution isn't at all generic. it relies on the fact that the input is square, that
    // we start in the middle, and the that number of steps is such that we'll end on complete
    // squares going to the left, right, up, and down from the initial square
    // under these conditions, the solution is one of two quadratic function in the number of complete
    // squares traversed, where which quadratic function used depends on the parity of the number of complete
    // squares traversed.
    // none of these observations are mine - all stolen from reddit especially
    // https://www.reddit.com/r/adventofcode/comments/18nol3m/2023_day_21_a_geometric_solutionexplanation_for/

    assert_eq!(num_rows, num_cols);
    assert_eq!(num_rows % 2, 1);
    assert_eq!(start.0, num_rows / 2);
    assert_eq!(start.1, num_rows / 2);
    assert_eq!((num_steps as i32 - start.0) % num_rows, 0);

    let num_cycles = (num_steps as i32 - start.0) / num_rows;

    let ds = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut prev = HashSet::<(i32, i32)>::new();
    let mut curr = HashSet::<(i32, i32)>::new();
    curr.insert(start);

    let mut running_count = if num_steps % 2 == 0 { 1 } else { 0 };
    let mut pts = vec![];

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
            running_count += next.len();
        }

        if i > start.0
            && (i - start.0) % num_rows == 0
            && ((i - start.0) / num_rows) % 2 == num_cycles % 2
        {
            pts.push(((i / num_rows / 2) as i64, running_count as i64));
            if pts.len() == 4 {
                break;
            }
        }

        prev = curr;
        curr = next;
    }

    let deltas = pts
        .iter()
        .zip(pts.iter().skip(1))
        .map(|(a, b)| b.1 - a.1)
        .collect::<Vec<_>>();
    let delta_deltas = deltas
        .iter()
        .zip(deltas.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();
    if delta_deltas[0] != delta_deltas[1] {
        panic!();
    }

    let c = delta_deltas[0] / 2;
    let b = pts[1].1 - pts[0].1 - c * (2 * pts[0].0 + 1);
    let a = pts[0].1 - b * pts[0].0 - c * pts[0].0 * pts[0].0;

    let x = (num_cycles / 2) as i64;
    a + b * x + c * x * x
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

fn main() {
    println!("Advent of Code 2023");
    println!("Day 21: Step Counter");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let ((num_rows, num_cols), start, rocks) = parse_input(&raw_input);

    let part_one = navigate(start, &rocks, 64);
    let part_two = navigate_infinite(num_rows, num_cols, start, &rocks, 26501365);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
