use std::collections::HashMap;
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

fn count_arrangements(records: &Vec<(Vec<Status>, Vec<usize>)>, multiplier: usize) -> usize {
    records
        .iter()
        .map(|(statuses, pattern)| {
            let mut mult_statuses = vec![];
            let mut mult_pattern = vec![];

            for i in 0..multiplier {
                for j in 0..statuses.len() {
                    mult_statuses.push(statuses[j]);
                }
                if i < multiplier - 1 {
                    mult_statuses.push(Status::Unknown);
                }
                for j in 0..pattern.len() {
                    mult_pattern.push(pattern[j]);
                }
            }
            count_arrangements_for_record(&mult_statuses, &mult_pattern)
        })
        .sum()
}

fn count_arrangements_for_record(statuses: &Vec<Status>, pattern: &Vec<usize>) -> usize {
    let mut starts = HashMap::<usize, usize>::new();
    starts.insert(0, 1);

    for i in 0..pattern.len() {
        let rest_pattern = pattern[i..].to_vec();
        let rest_needed = rest_pattern.iter().sum::<usize>() + rest_pattern.len() - 1;
        let mut new_starts = HashMap::<usize, usize>::new();

        for (&start, &count) in starts.iter() {
            if statuses.len() - start < rest_needed {
                continue;
            }

            let matches = match_next_status(&statuses, &pattern, start, i);
            for (last_in_seq, ct) in matches.iter() {
                let next_start = last_in_seq + 1;
                let prev = new_starts.get(&next_start).unwrap_or(&0);
                new_starts.insert(next_start, prev + count * ct);
            }
        }

        starts = new_starts;
    }

    starts
        .into_iter()
        .filter_map(|(start, ct)| {
            if start > statuses.len() - 1 {
                Some(ct)
            } else if statuses[start..].iter().any(|&s| s == Status::Damaged) {
                None
            } else {
                Some(ct)
            }
        })
        .sum()
}

fn match_next_status(
    statuses: &Vec<Status>,
    pattern: &Vec<usize>,
    status_start: usize,
    next_match_idx: usize,
) -> HashMap<usize, usize> {
    let last = next_match_idx == pattern.len() - 1;
    let mut results = HashMap::new();

    let needed_for_rest =
        pattern[(next_match_idx + 1)..].iter().sum::<usize>() + pattern.len() - next_match_idx - 1;
    let end = statuses.len() - needed_for_rest;
    let to_match = pattern[next_match_idx];

    let mut to_check = vec![status_start];

    while let Some(next) = to_check.pop() {
        if end - next < to_match {
            continue;
        }

        if statuses[next] == Status::Operational {
            to_check.push(next + 1);
            continue;
        }

        if statuses[next] == Status::Unknown {
            to_check.push(next + 1);
        }

        if (0..to_match)
            .all(|i| statuses[next + i] == Status::Damaged || statuses[next + i] == Status::Unknown)
        {
            let last_in_run = next + to_match - 1;

            if last_in_run == statuses.len() - 1 {
                if last {
                    let prev = results.get(&last_in_run).unwrap_or(&0);
                    results.insert(last_in_run, prev + 1);
                }
                continue;
            }

            if statuses[last_in_run + 1] == Status::Damaged {
                continue;
            }

            let prev = results.get(&(last_in_run + 1)).unwrap_or(&0);
            results.insert(last_in_run + 1, prev + 1);
        }
    }

    results
}

fn parse_input(input: &str) -> Vec<(Vec<Status>, Vec<usize>)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let pcs = line.trim().split_whitespace().collect::<Vec<_>>();
            let statuses = pcs[0]
                .chars()
                .map(|c| match c {
                    '#' => Status::Damaged,
                    '?' => Status::Unknown,
                    _ => Status::Operational,
                })
                .collect::<Vec<_>>();

            let pattern = pcs[1]
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            (statuses, pattern)
        })
        .collect::<Vec<_>>()
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 12: Hot Springs");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let records = parse_input(&raw_input);

    let part_one = count_arrangements(&records, 1);
    let part_two = count_arrangements(&records, 5);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
