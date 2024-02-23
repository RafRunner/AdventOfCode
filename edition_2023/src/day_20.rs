use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    vec,
};

pub fn part_one(input: &str) -> usize {
    let mut pulse_count = (0, 0);

    let modules = parse_modules(input);

    for _ in 0..1000 {
        press_button(&modules, &mut pulse_count);
    }

    pulse_count.0 * pulse_count.1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, PartialEq, Eq)]
enum ModuleType {
    FlipFlop { on: bool },
    Conjunction { memory: HashMap<String, Pulse> },
    Broadcast,
}

#[derive(Debug)]
struct Module {
    name: String,
    kind: ModuleType,
    targets: Vec<String>,
}

impl Module {
    fn parse(line: &str) -> Self {
        let parts = line.trim().split(" -> ").collect::<Vec<_>>();
        let name = String::from(parts[0]);
        let targets = parts[1].split(", ").map(String::from).collect::<Vec<_>>();

        if name == "broadcaster" {
            Module {
                name,
                kind: ModuleType::Broadcast,
                targets,
            }
        } else {
            let kind = if name.starts_with('%') {
                ModuleType::FlipFlop { on: false }
            } else {
                ModuleType::Conjunction {
                    memory: HashMap::new(),
                }
            };

            Module {
                name: String::from(&name[1..]),
                kind,
                targets,
            }
        }
    }

    fn process_pulse(&mut self, sender: &str, pulse: &Pulse) -> Option<Pulse> {
        match &mut self.kind {
            ModuleType::FlipFlop { on } => {
                match pulse {
                    Pulse::High => None,
                    Pulse::Low => {
                        *on = !*on; // Toggle the state
                        Some(if *on { Pulse::High } else { Pulse::Low })
                    }
                }
            }
            ModuleType::Conjunction { memory } => {
                memory.insert(sender.to_string(), *pulse); // Insert or update the sender's pulse

                let output_pulse = if memory.values().all(|p| *p == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                Some(output_pulse)
            }
            ModuleType::Broadcast => Some(*pulse), // Directly pass through the pulse
        }
    }
}

fn parse_modules(input: &str) -> HashMap<String, RefCell<Module>> {
    let modules: HashMap<String, RefCell<Module>> = input
        .lines()
        .map(Module::parse)
        .map(|m| (m.name.clone(), m))
        .map(|(key, val)| (key, RefCell::new(val)))
        .collect();

    for (name, module) in modules.iter() {
        let module = &mut module.borrow_mut();

        if let ModuleType::Conjunction { memory } = &mut module.kind {
            for input in modules.values().filter(|it| {
                it.try_borrow()
                    .map(|m| m.targets.contains(name))
                    .unwrap_or(false)
            }) {
                memory.insert(input.borrow().name.clone(), Pulse::Low);
            }
        }
    }

    modules
}

fn press_button(modules: &HashMap<String, RefCell<Module>>, pulse_count: &mut (usize, usize)) {
    let button = RefCell::new(Module {
        name: "button".to_string(),
        kind: ModuleType::Broadcast,
        targets: vec!["broadcaster".to_string()],
    });

    let mut queue = VecDeque::new();
    queue.push_back((&button, "elf".to_string(), Pulse::Low));

    while let Some((current_module, current_sender, current_pulse)) = queue.pop_front() {
        let mut module = current_module.borrow_mut();

        if let Some(pulse) = module.process_pulse(current_sender.as_str(), &current_pulse) {
            for target in &module.targets {
                // println!("{} -> -{:?} {}", module.name, pulse, target);
                match pulse {
                    Pulse::High => pulse_count.0 += 1,
                    Pulse::Low => pulse_count.1 += 1,
                }

                if let Some(target_module) = modules.get(target) {
                    queue.push_back((target_module, module.name.clone(), pulse));
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "\
        broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a";

        assert_eq!(32_000_000, part_one(input));

        let input = "\
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output";

        assert_eq!(11_687_500, part_one(input));
    }

    #[test]
    fn real() {
        let input = include_str!("../res/day_20.txt");

        assert_eq!(867118762, part_one(input));
    }
}
