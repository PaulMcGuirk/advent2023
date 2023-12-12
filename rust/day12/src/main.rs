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

            let last = i == pattern.len() - 1;

            let matches = match_first_status(&statuses[start..].to_vec(), &rest_pattern, last);

            for (last_in_seq, ct) in matches.iter() {
                let next_start = start + last_in_seq + 1;
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

fn match_first_status(
    statuses: &Vec<Status>,
    pattern: &Vec<usize>,
    last: bool,
) -> HashMap<usize, usize> {
    let mut results = HashMap::new();

    let rest = pattern[1..].iter().sum::<usize>() + pattern.len() - 1;

    let mut to_check = vec![statuses[..(statuses.len() - rest)].to_vec()];
    let to_match = pattern[0];

    while let Some(next) = to_check.pop() {
        // println!("checkin {:?}", next);
        if next.len() < to_match {
            continue;
        }

        if next[0] == Status::Operational {
            to_check.push(next[1..].to_vec());
            continue;
        }

        if next[0] == Status::Unknown {
            to_check.push(next[1..].to_vec());
            let mut next = next.clone();
            next[0] = Status::Damaged;
            to_check.push(next);
            continue;
        }

        if (0..to_match).all(|i| next[i] == Status::Damaged || next[i] == Status::Unknown) {
            let run_start = statuses.len() - rest - next.len(); // where the next seq starts in statuses
            let last_in_run = run_start + to_match - 1;

            // println!("{} {}", run_start, last_in_run);

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

            // println!("adding one here");
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

    let raw_input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    // let raw_input = "????.#...#... 4,1,1";
    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");
    // let raw_input = "?###???????? 3,2,1";
    let records = parse_input(&raw_input);

    // for (statuses, pattern) in records.iter() {
    //     let s = statuses
    //         .iter()
    //         .map(|i| match i {
    //             Status::Damaged => '#',
    //             Status::Operational => '.',
    //             Status::Unknown => '?',
    //         })
    //         .collect::<String>();
    //     // println!("old: {}", old);
    //     // println!("new: {}", new);
    //     println!("{}", s);
    //     let old = count_arrangements_for_record_old(statuses, pattern);
    //     let new = count_arrangements_for_record(statuses, pattern);
    //     if old != new {
    //         let s = statuses
    //             .iter()
    //             .map(|i| match i {
    //                 Status::Damaged => '#',
    //                 Status::Operational => '.',
    //                 Status::Unknown => '?',
    //             })
    //             .collect::<String>();
    //         println!("old: {}", old);
    //         println!("new: {}", new);
    //         println!("{}", s);
    //         println!("{:?}", pattern);
    //         break;
    //     }
    // }

    let part_one = count_arrangements(&records, 1);
    let part_two = count_arrangements(&records, 5);
    // // let part_two = data.get_distances(1000000);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
