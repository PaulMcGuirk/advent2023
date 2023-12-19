use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

enum Action {
    Send(usize),
    Reject,
    Acccept,
}

enum Condition {
    LessThan(char, u32),
    GreaterThan(char, u32),
    True,
}

struct Step {
    condition: Condition,
    action: Action,
}

fn part_one(workflows: &Vec<Vec<Step>>, parts: &Vec<HashMap<char, u32>>) -> u32 {
    parts
        .iter()
        .filter_map(|part| {
            let region = part
                .iter()
                .map(|(c, val)| (*c, (*val, *val)))
                .collect::<HashMap<_, _>>();
            if get_acceptable_volume(workflows, &region) > 0 {
                Some(part.values().sum::<u32>())
            } else {
                None
            }
        })
        .sum()
}

fn part_two(workflows: &Vec<Vec<Step>>) -> u64 {
    let seed = {
        let mut seed = HashMap::new();
        seed.insert('x', (1, 4000));
        seed.insert('m', (1, 4000));
        seed.insert('a', (1, 4000));
        seed.insert('s', (1, 4000));
        seed
    };
    get_acceptable_volume(workflows, &seed)
}

fn get_acceptable_volume(workflows: &Vec<Vec<Step>>, region: &HashMap<char, (u32, u32)>) -> u64 {
    let mut res = 0;
    let mut to_process = vec![(region.clone(), 0)];

    while let Some(next) = to_process.pop() {
        let (range, idx) = next;

        assert!(range.values().all(|r| r.1 >= r.0));

        let workflow = &workflows[idx];

        for rule in workflow.iter() {
            let range = range.clone();
            let pass = match rule.condition {
                Condition::LessThan(c, val) => {
                    if &val > &range[&c].1 {
                        true
                    } else if val > range[&c].0 {
                        let mut lower = range.clone();
                        lower.insert(c, (range[&c].0, val - 1));
                        to_process.push((lower, idx));
                        let mut upper = range.clone();
                        upper.insert(c, (val, range[&c].1));
                        to_process.push((upper, idx));
                        break;
                    } else {
                        false
                    }
                }
                Condition::GreaterThan(c, val) => {
                    if val < range[&c].0 {
                        true
                    } else if val < range[&c].1 {
                        let mut lower = range.clone();
                        lower.insert(c, (range[&c].0, val));
                        to_process.push((lower, idx));
                        let mut upper = range.clone();
                        upper.insert(c, (val + 1, range[&c].1));
                        to_process.push((upper, idx));
                        break;
                    } else {
                        false
                    }
                }
                Condition::True => true,
            };

            if !pass {
                continue;
            }

            match rule.action {
                Action::Acccept => {
                    res += range
                        .values()
                        .map(|r| (r.1 - r.0 + 1) as u64)
                        .product::<u64>();
                }
                Action::Reject => {}
                Action::Send(new_idx) => {
                    to_process.push((range, new_idx));
                }
            }
            break;
        }
    }
    res
}

fn parse_input(s: &str) -> (Vec<Vec<Step>>, Vec<HashMap<char, u32>>) {
    let mut pcs = s.trim().split("\n\n");

    let workflow_re = Regex::new(r"^(?P<name>[a-zA-Z]+)\{(?P<steps>.*)\}$").unwrap();
    let condition_re =
        Regex::new(r"^(?P<attr>[xmas])(?P<op>[<>])(?P<val>\d+):(?P<action>[AR]|[a-z]+)$").unwrap();

    let step_data = {
        let mut step_data = pcs
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let caps = workflow_re.captures(line).unwrap();
                let name = caps["name"].to_string();
                let steps = caps["steps"].to_string();
                (name, steps)
            })
            .collect::<Vec<_>>();
        step_data.sort_by_key(|(name, _)| if name == "in" { 0 } else { 1 });
        step_data
    };

    let step_names = step_data
        .iter()
        .enumerate()
        .map(|(i, d)| (d.0.clone(), i))
        .collect::<HashMap<_, _>>();

    let mut workflows: Vec<Option<Vec<Step>>> = vec![];
    for _ in 0..step_names.len() {
        workflows.push(Some(vec![]));
    }

    for (name, steps) in step_data.into_iter() {
        let idx = step_names[&name];
        let steps = steps
            .split(",")
            .map(|step| {
                let (condition, action_str) = if let Some(caps) = condition_re.captures(step) {
                    let attr = caps["attr"].chars().next().unwrap();
                    let val = caps["val"].parse::<u32>().unwrap();
                    let cond = match &caps["op"] {
                        ">" => Condition::GreaterThan(attr, val),
                        "<" => Condition::LessThan(attr, val),
                        _ => panic!(),
                    };
                    (cond, caps["action"].to_string())
                } else {
                    (Condition::True, step.to_string())
                };

                let action = match action_str.as_str() {
                    "A" => Action::Acccept,
                    "R" => Action::Reject,
                    s => Action::Send(*step_names.get(s).unwrap()),
                };

                Step { condition, action }
            })
            .collect::<Vec<_>>();
        workflows[idx] = Some(steps);
    }

    let workflows = workflows
        .into_iter()
        .map(|s| s.unwrap())
        .collect::<Vec<_>>();

    let parts = pcs
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.replace("}", "")
                .replace("{", "")
                .split(",")
                .map(|att| {
                    let mut sub_pcs = att.split("=");
                    (
                        sub_pcs.next().unwrap().chars().next().unwrap(),
                        sub_pcs.next().unwrap().parse::<u32>().unwrap(),
                    )
                })
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>();

    (workflows, parts)
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 19: Aplenty");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");

    let (workflows, parts) = parse_input(&raw_input);

    let part_one = part_one(&workflows, &parts);
    let part_two = part_two(&workflows);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
