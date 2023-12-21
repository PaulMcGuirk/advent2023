use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

#[derive(Copy, Clone, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(PartialEq, Eq)]
enum ModuleType {
    Broadcast,
    Flipflip,
    Conjunction,
}

struct Module {
    name: String,
    module_type: ModuleType,
    status: bool,
    history: HashMap<String, Pulse>,
    destinations: Vec<String>,
}

impl Module {
    fn from_str(s: &str) -> Self {
        let mut pcs = s.trim().split(" -> ");
        let identifier = pcs.next().unwrap();

        let first_char = identifier.chars().next().unwrap();

        let (name, module_type) = if first_char.is_alphabetic() {
            (identifier.to_string(), ModuleType::Broadcast)
        } else {
            let name = identifier[1..].to_string();
            let module_type = match first_char {
                '%' => ModuleType::Flipflip,
                '&' => ModuleType::Conjunction,
                _ => panic!(),
            };
            (name, module_type)
        };

        let status = false;
        let history = HashMap::new();

        let destinations = pcs
            .next()
            .unwrap()
            .trim()
            .split(",")
            .map(|dest| dest.trim().to_string())
            .collect::<Vec<_>>();

        Self {
            name,
            status,
            module_type,
            history,
            destinations,
        }
    }

    fn process_pulse(&mut self, source: &str, pulse: Pulse) -> Vec<(String, Pulse)> {
        match self.module_type {
            ModuleType::Broadcast => self
                .destinations
                .iter()
                .map(|dest| (dest.clone(), pulse))
                .collect(),
            ModuleType::Flipflip => {
                match pulse {
                    Pulse::High => {
                        return vec![]; // no-op
                    }
                    Pulse::Low => {
                        let to_send = if self.status { Pulse::Low } else { Pulse::High };
                        self.status = !self.status;
                        self.destinations
                            .iter()
                            .map(|dest| (dest.clone(), to_send))
                            .collect()
                    }
                }
            }
            ModuleType::Conjunction => {
                if self.history.contains_key(source) {
                    self.history.insert(source.to_string(), pulse);
                }

                let to_send = if self.history.values().all(|&v| v == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                self.destinations
                    .iter()
                    .map(|dest| (dest.clone(), to_send))
                    .collect()
            }
        }
    }
}

struct System {
    modules: HashMap<String, Module>,
}

impl System {
    fn from_str(s: &str) -> Self {
        let mut modules = s
            .trim()
            .lines()
            .map(|line| {
                let module = Module::from_str(line);
                (module.name.to_string(), module)
            })
            .collect::<HashMap<_, _>>();

        let conjs = modules
            .iter()
            .filter_map(|(name, module)| match module.module_type {
                ModuleType::Conjunction => {
                    let sources = modules
                        .iter()
                        .filter_map(|(other_name, other_mod)| {
                            if other_mod.destinations.iter().any(|d| d == name) {
                                Some(other_name.clone())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    Some((name.clone(), sources))
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        for (dest, srcs) in conjs.into_iter() {
            let history = srcs
                .into_iter()
                .map(|src| src.to_string())
                .collect::<HashSet<_>>()
                .into_iter()
                .map(|src| (src, Pulse::Low))
                .collect::<HashMap<_, _>>();
            if dest == "hb" {
                modules.get_mut(&dest).unwrap().history =
                    history.into_iter().collect::<HashMap<_, _>>();
            } else {
                modules.get_mut(&dest).unwrap().history = history;
            }
        }

        Self { modules }
    }

    fn press(&mut self, count: usize) -> usize {
        let mut to_press = VecDeque::new();
        let mut num_lows = 0;
        let mut num_highs = 0;

        for _ in 0..count {
            to_press.push_back((
                String::from("button"),
                String::from("broadcaster"),
                Pulse::Low,
            ));

            while let Some((src_name, dest_name, pulse)) = to_press.pop_front() {
                if pulse == Pulse::Low {
                    num_lows += 1;
                } else {
                    num_highs += 1;
                }

                if let Some(dest) = self.modules.get_mut(&dest_name) {
                    let output = dest.process_pulse(&src_name, pulse);

                    for (output_dest_name, output_pulse) in output.into_iter() {
                        to_press.push_back((dest_name.clone(), output_dest_name, output_pulse));
                    }
                }
            }
        }

        num_lows * num_highs
    }

    fn reset(&mut self) {
        for module in self.modules.values_mut() {
            module.status = false;
            module.history = module
                .history
                .keys()
                .map(|key| (key.to_string(), Pulse::Low))
                .collect();
        }
    }

    fn find_cycle(&mut self) -> u64 {
        // remarks: this depends quite a bit on the input
        let mut to_press = VecDeque::new();

        let mut preqs = self.modules.values().filter_map(|module| {
            if module.destinations.iter().any(|dest| dest == "rx") {
                Some(module.name.clone())
            } else {
                None
            }
        });

        let mut found = if let Some(preq) = preqs.next() {
            if self.modules[&preq].module_type != ModuleType::Conjunction {
                panic!("Can't handle this case");
            } else {
                self.modules[&preq]
                    .history
                    .keys()
                    .map(|dest| (dest.to_string(), None))
                    .collect::<HashMap<String, Option<u64>>>()
            }
        } else {
            panic!("Can't handle this case")
        };

        if let Some(_) = preqs.next() {
            panic!("Can't handle this case");
        }

        'step_loop: for i in 0.. {
            to_press.push_back((
                String::from("button"),
                String::from("broadcaster"),
                Pulse::Low,
            ));

            while let Some((src_name, dest_name, pulse)) = to_press.pop_front() {
                if pulse == Pulse::Low {
                    if dest_name == "rx" {
                        return i + 1;
                    }
                }

                if let Some(dest) = self.modules.get_mut(&dest_name) {
                    let output = dest.process_pulse(&src_name, pulse);

                    for (output_dest_name, output_pulse) in output.into_iter() {
                        to_press.push_back((dest_name.clone(), output_dest_name, output_pulse));
                    }
                }

                if let Some(src) = self.modules["hb"].history.iter().find_map(|(src, p)| {
                    if found[src].is_none() && *p == Pulse::High {
                        Some(src)
                    } else {
                        None
                    }
                }) {
                    found.insert(src.to_string(), Some(i + 1));

                    if found.values().all(|c| c.is_some()) {
                        break 'step_loop;
                    }
                }
            }
        }

        Self::lcm(found.values().map(|v| v.unwrap()).collect::<Vec<_>>())
    }

    fn lcm(nums: Vec<u64>) -> u64 {
        nums.into_iter()
            .fold(1, |res, num| res * num / Self::gcd(res, num))
    }

    fn gcd(a: u64, b: u64) -> u64 {
        let mut a = a;
        let mut b = b;
        while b > 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 20: Pulse Propagation");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");

    let mut system = System::from_str(&raw_input);

    let part_one = system.press(1000);
    system.reset();
    let part_two = system.find_cycle();

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
