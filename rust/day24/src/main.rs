use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

fn count_intersections(hail: &Vec<(Vec<i64>, Vec<i64>)>, min: i64, max: i64) -> usize {
    (0..(hail.len() - 1))
        .map(|i| {
            ((i + 1)..hail.len())
                .filter(|&j| intersects(&hail[i], &hail[j], min, max))
                .count()
        })
        .sum()
}

fn intersects(hi: &(Vec<i64>, Vec<i64>), hj: &(Vec<i64>, Vec<i64>), min: i64, max: i64) -> bool {
    let (ri, vi) = hi;
    let (rj, vj) = hj;

    if vi[0] * vj[1] == vi[1] * vj[0] {
        return false; // parallel
    }

    let x = (rj[1] as f64 - ri[1] as f64 + (vi[1] as f64) * (ri[0] as f64) / (vi[0] as f64)
        - (vj[1] as f64) * (rj[0] as f64) / (vj[0] as f64))
        / ((vi[1] as f64) / (vi[0] as f64) - (vj[1] as f64) / (vj[0] as f64));

    if x < (min as f64) || x > (max as f64) {
        return false;
    }

    let ti = (x - ri[0] as f64) / (vi[0] as f64);
    if ti <= 0.0 {
        return false;
    }

    let tj = (x - rj[0] as f64) / (vj[0] as f64);
    if tj <= 0.0 {
        return false;
    }

    let y = ri[1] as f64 + (vi[1] as f64) * ti;

    y >= (min as f64) && y <= (max as f64)
}

fn aim(hail: &Vec<(Vec<i64>, Vec<i64>)>) -> i64 {
    // if any of the hailstones have the same velocity, this greatly
    // reduces the possible velocities of the rock
    let vs = (0..3)
        .map(|i| {
            let mut vels = HashSet::new();
            let mut v = -1000;
            while v <= 1000 {
                vels.insert(v);
                v += 1
            }

            for (j, h1) in hail.iter().enumerate() {
                for h2 in hail.iter().skip(j) {
                    if h1.1[i] != h2.1[i] {
                        continue;
                    }

                    let v_c = h1.1[i];
                    let d = (h1.0[i] - h2.0[i]).abs();

                    vels = vels
                        .into_iter()
                        .filter(|&v| v == v_c || d % ((v_c - v).abs()) == 0)
                        .collect::<HashSet<_>>();
                }
            }

            vels.into_iter().collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for &vx in vs[0].iter() {
        for &vy in vs[1].iter() {
            for &vz in vs[2].iter() {
                let hail = hail
                    .iter()
                    .map(|(pos, vel)| (pos.clone(), vec![vel[0] - vx, vel[1] - vy, vel[2] - vz]))
                    .collect::<Vec<_>>();

                let to_check = hail
                    .iter()
                    .filter(|(_, vel)| vel[0] != 0 && vel[1] != 0)
                    .collect::<Vec<_>>();

                let (ri, vi) = &to_check[0];
                let (rj, vj) = &to_check[1];

                // see where the first two hailstones intersect in the xy plane

                if vi[0] * vj[1] == vi[1] * vj[0] {
                    continue; // parallel - won't ever intesect
                }

                let x0 = (rj[1] as f64 - ri[1] as f64
                    + (vi[1] as f64) * (ri[0] as f64) / (vi[0] as f64)
                    - (vj[1] as f64) * (rj[0] as f64) / (vj[0] as f64))
                    / ((vi[1] as f64) / (vi[0] as f64) - (vj[1] as f64) / (vj[0] as f64));
                let x0 = x0.round() as i64;

                let ti = (x0 as f64 - ri[0] as f64) / (vi[0] as f64);
                let ti = ti as i64;

                if x0 != ri[0] + vi[0] * ti {
                    continue; // not actually an intersection over integers
                }

                let y0 = ri[1] + vi[1] * ti;
                let z0 = ri[2] + vi[2] * ti;

                let tj = (x0 as f64 - rj[0] as f64) / (vj[0] as f64);
                let tj = tj as i64;

                if x0 != rj[0] + vj[0] * tj {
                    continue; // not actually an intersection over integers
                }

                if y0 != rj[1] + vj[1] * tj {
                    continue;
                }

                if z0 != rj[2] + vj[2] * tj {
                    continue;
                }

                return x0 + y0 + z0;
            }
        }
    }
    panic!()
}

fn parse_input(input: &str) -> Vec<(Vec<i64>, Vec<i64>)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let pcs = line
                .trim()
                .split("@")
                .map(|pc| {
                    pc.trim()
                        .split(",")
                        .map(|n| n.trim().parse::<i64>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            (pcs[0].clone(), pcs[1].clone())
        })
        .collect()
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 24: Never Tell Me The Odds");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let hail = parse_input(&raw_input);

    let part_one = count_intersections(&hail, 200000000000000, 400000000000000);
    let part_two = aim(&hail);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
