use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

fn part_one(seeds: &Vec<u64>, maps: &Vec<Vec<(u64, u64, u64)>>) -> u64 {
    seeds.iter()
        .map(|&seed| map(seed, maps))
        .min()
        .unwrap()
}

fn map(seed: u64, maps: &Vec<Vec<(u64, u64, u64)>>) -> u64 {
    let mut val = seed;

    for map in maps.iter() {
        let mut new_val = None;
        for &(dest_min, src_min, range) in map.iter() {
            if val >= src_min && val <= src_min + range {
                new_val = Some(dest_min + val - src_min)
            };
        }

        val = if let Some(val) = new_val {
            val
        } else {
            val
        }
    }

    val
}

fn part_two(seeds: &Vec<u64>, maps: &Vec<Vec<(u64, u64, u64)>>) -> u64 {
    let mut ranges = vec![];
    for i in (0..seeds.len()).step_by(2) {
        let start = seeds[i];
        let end = start + seeds[i + 1] - 1;
        ranges.push((start, end));
    }

    for map in maps.iter() {
        let mut new_ranges = vec![];

        for &(range_start, range_end) in ranges.iter() {
            let mut remaining_ranges = vec![(range_start, range_end)];

            for &(dest_min, src_min, range) in map.iter() {
                let mut x = vec![];
                let src_max = src_min + range - 1;
                let mut ranges_to_map = vec![];
                for &(s, r) in remaining_ranges.iter() {
                    let intersects = s <= src_max && src_min <= r;
                    if !intersects {
                        x.push((s, r));
                        continue;
                    }

                    ranges_to_map.push((src_min.max(s), src_max.min(r)));
                    if s < src_min {
                        x.push((s, src_min - 1));
                    }
                    if r > src_max {
                        x.push((src_max + 1, r));
                    }
                }

                remaining_ranges = x;

                for (s, r) in ranges_to_map.into_iter() {
                    let mapped_s = dest_min + s - src_min;
                    let mapped_r = dest_min + r - src_min;
                    new_ranges.push((mapped_s, mapped_r));
                }
                
            }

            new_ranges.extend(remaining_ranges.into_iter());
        }

        ranges = new_ranges;
    }

    ranges.into_iter().map(|r| r.0).min().unwrap()
    
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<Vec<(u64, u64, u64)>>) {
    let mut pcs = input.trim().split("\n\n");

    let seeds = pcs.next().unwrap().trim().split(":").skip(1).next().unwrap().trim().split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let maps = pcs.map(|pc|
        pc.lines()
            .skip(1)
            .filter_map(|ln| {
                let pcs = ln.split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();
                if pcs.is_empty() {
                    None
                } else {
                    Some((pcs[0], pcs[1], pcs[2]))
                }
            }).collect::<Vec<_>>()
    ).collect::<Vec<_>>();

    (seeds, maps)
}
fn main() {
    println!("Advent of Code 2023");
    println!("Day 5: If You Give A Seed A Fertilizer");

    let now = Instant::now();

    let input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let (seeds, maps) = parse_input(&input);

    let part_one = part_one(&seeds, &maps);
    let part_two = part_two(&seeds, &maps);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}