use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

#[derive(Clone, Copy)]
struct Interval {
    start: u32,
    end: u32,
}

fn solve(seeds: &Vec<u32>, maps: &Vec<Vec<(Interval, Interval)>>, range: bool) -> u32 {
    let intervals = if range {
        (0..seeds.len())
            .step_by(2)
            .map(|i| Interval {
                start: seeds[i],
                end: seeds[i] + seeds[i + 1] - 1,
            })
            .collect::<Vec<_>>()
    } else {
        seeds
            .iter()
            .map(|&s| Interval { start: s, end: s })
            .collect::<Vec<_>>()
    };

    maps.iter()
        .fold(intervals, |vals, layers| {
            let mut new_vals = vec![];
            let mut unmapped_vals = vals.clone();

            while let Some(int) = unmapped_vals.pop() {
                let interseting_int = layers
                    .iter()
                    .find(|(_, src)| int.start <= src.end && src.start <= int.end);

                if let Some((dest, src)) = interseting_int {
                    let overlap = Interval {
                        start: src.start.max(int.start),
                        end: src.end.min(int.end),
                    };
                    let mapped = Interval {
                        start: dest.start + overlap.start - src.start,
                        end: dest.end + overlap.end - src.end,
                    };
                    new_vals.push(mapped);
                    if int.start < src.start {
                        unmapped_vals.push(Interval {
                            start: int.start,
                            end: src.start - 1,
                        });
                    }
                    if int.end > src.end {
                        unmapped_vals.push(Interval {
                            start: src.end + 1,
                            end: int.end,
                        });
                    }
                } else {
                    new_vals.push(int); // no intersections, so an identity map
                }
            }

            new_vals
        })
        .into_iter()
        .map(|int| int.start)
        .min()
        .unwrap()
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<Vec<(Interval, Interval)>>) {
    let mut pcs = input.trim().split("\n\n");

    let seeds = pcs
        .next()
        .unwrap()
        .trim()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let maps = pcs
        .map(|pc| {
            pc.lines()
                .skip(1)
                .filter_map(|ln| {
                    let data = ln
                        .split_whitespace()
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<Vec<_>>();
                    if data.is_empty() {
                        None
                    } else {
                        let dest_start = data[0];
                        let src_start = data[1];
                        let range = data[2];
                        let dest = Interval {
                            start: dest_start,
                            end: dest_start + range - 1,
                        };
                        let src = Interval {
                            start: src_start,
                            end: src_start + range - 1,
                        };
                        Some((dest, src))
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (seeds, maps)
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 5: If You Give A Seed A Fertilizer");

    let now = Instant::now();

    let input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let (seeds, maps) = parse_input(&input);

    let part_one = solve(&seeds, &maps, false);
    let part_two = solve(&seeds, &maps, true);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
