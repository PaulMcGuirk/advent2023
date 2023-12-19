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

fn process(start: usize, workflows: &Vec<Vec<Step>>, parts: &Vec<HashMap<char, u32>>) -> u32 {
    parts
        .iter()
        .filter_map(|part| {
            if is_accepted(start, workflows, part) {
                Some(part.values().sum::<u32>())
            } else {
                None
            }
        })
        .sum()
}

fn is_accepted(start: usize, workflows: &Vec<Vec<Step>>, part: &HashMap<char, u32>) -> bool {
    let mut idx = start;
    loop {
        let workflow = &workflows[idx];
        for rule in workflow.iter() {
            let pass = match rule.condition {
                Condition::LessThan(c, val) => part[&c] < val,
                Condition::GreaterThan(c, val) => part[&c] > val,
                Condition::True => true,
            };
            if !pass {
                continue;
            }
            match rule.action {
                Action::Acccept => return true,
                Action::Reject => return false,
                Action::Send(i) => {
                    idx = i;
                    break;
                }
            }
        }
    }
}

fn count_acceptable(start: usize, workflows: &Vec<Vec<Step>>) -> u64 {
    let mut res = 0;
    let mut to_process = {
        let mut seed = HashMap::new();
        seed.insert('x', (1, 4000));
        seed.insert('m', (1, 4000));
        seed.insert('a', (1, 4000));
        seed.insert('s', (1, 4000));
        vec![(seed, start)]
    };

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

fn parse_input(s: &str) -> (usize, Vec<Vec<Step>>, Vec<HashMap<char, u32>>) {
    let mut pcs = s.trim().split("\n\n");

    let step_data = pcs
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.replace("}", "")
                .split("{")
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let step_names = step_data
        .iter()
        .enumerate()
        .map(|(i, d)| (d[0].to_string(), i))
        .collect::<HashMap<_, _>>();

    let mut rules: Vec<Option<Vec<Step>>> = vec![];
    for _ in 0..step_names.len() {
        rules.push(Some(vec![]));
    }

    for d in step_data.into_iter() {
        let name = step_names[&d[0]];
        let steps = d[1]
            .split(",")
            .map(|r| {
                let parts = r.split(":").collect::<Vec<_>>();
                let cond = if parts.len() > 1 {
                    let cond = parts[0];
                    let symb = if cond.contains(">") { ">" } else { "<" };
                    let mut sides = cond.split(symb);
                    let lhs = sides.next().unwrap().chars().next().unwrap();
                    let rhs = sides.next().unwrap().parse::<u32>().unwrap();
                    if symb == ">" {
                        Condition::GreaterThan(lhs, rhs)
                    } else {
                        Condition::LessThan(lhs, rhs)
                    }
                } else {
                    Condition::True
                };
                let action = match parts.last().unwrap() {
                    &"A" => Action::Acccept,
                    &"R" => Action::Reject,
                    s => Action::Send(*step_names.get(*s).unwrap()),
                };
                Step {
                    condition: cond,
                    action,
                }
            })
            .collect::<Vec<_>>();
        rules[name] = Some(steps);
    }

    let rules = rules.into_iter().map(|s| s.unwrap()).collect::<Vec<_>>();

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

    (step_names["in"], rules, parts)
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 19: Aplenty");

    let now = Instant::now();

    let raw_input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");

    let (start, workflows, parts) = parse_input(&raw_input);

    let part_one = process(start, &workflows, &parts);
    let part_two = count_acceptable(start, &workflows);

    // let part_one = DigPlan::from_str(&raw_input).volume();
    // let part_two = DigPlan::from_str_elvish(&raw_input).volume();

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
