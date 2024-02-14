use std::{collections::HashMap, fmt::Debug};

use num::BigInt;
use regex::{Match, Regex};

pub fn part_one(input: &str) -> isize {
    let program = Program::parse_program(input);

    program.run().iter().map(Part::sum).sum()
}

pub fn part_two(input: &str) -> BigInt {
    let program = Program::parse_program(input);
    let mut possible_checks = Vec::new();
    program.possible_checks(&mut possible_checks);

    possible_checks
        .into_iter()
        .map(|checks| {
            let mut ranges = vec![1..4001, 1..4001, 1..4001, 1..4001];

            for check in checks {
                if let Instruction::Less {
                    prop, to_compare, ..
                } = check
                {
                    ranges[prop.index()].end = to_compare
                } else if let Instruction::Greater {
                    prop, to_compare, ..
                } = check
                {
                    ranges[prop.index()].start = to_compare + 1
                }
            }

            dbg!(&ranges);

            ranges
        })
        .reduce(|mut acc, cur| {
            for (a, c) in acc.iter_mut().zip(cur.iter()) {
                if c.start > a.start && c.start < a.end {
                    a.start = c.start;
                }
                if c.end < a.end && c.end > a.start {
                    a.end = c.end;
                }
            }

            acc
        })
        .unwrap_or_default()
        .into_iter()
        .map(|range| {
            dbg!(&range);
            BigInt::from(range.len())
        })
        .product::<BigInt>()
}

#[derive(Debug)]
struct Program {
    symbol_table: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl Program {
    fn parse_program(string: &str) -> Self {
        let mut symbol_table = HashMap::new();
        let mut parts = Vec::new();

        for line in string.lines() {
            let line = line.trim();

            if let Some(workflow) = Workflow::parse(line) {
                symbol_table
                    .entry(workflow.name.clone())
                    .or_insert(workflow);
            } else if let Some(part) = Part::parse(line) {
                parts.push(part);
            }
        }

        Self {
            symbol_table,
            parts,
        }
    }

    fn run(&self) -> Vec<Part> {
        let mut accepted = Vec::new();

        if let Some(input) = self.symbol_table.get("in") {
            for part in &self.parts {
                self.execute_workflow(input, part, &mut accepted);
            }
        }

        accepted
    }

    fn execute_workflow(&self, current_workflow: &Workflow, part: &Part, accepted: &mut Vec<Part>) {
        for rule in &current_workflow.instructions {
            let next_step = rule.apply(part);

            match next_step {
                NextStep::Continue => (),
                NextStep::Accept => {
                    accepted.push(part.clone());
                    break;
                }
                NextStep::Reject => break,
                NextStep::Goto(workflow) => {
                    if let Some(next) = self.symbol_table.get(workflow.as_str()) {
                        self.execute_workflow(next, part, accepted)
                    }
                    break;
                }
            };
        }
    }

    fn possible_checks(&self, possible_checks: &mut Vec<Vec<Instruction>>) {
        if let Some(input) = self.symbol_table.get("in") {
            let mut current_checks = Vec::new();

            input.possible_checks(&self.symbol_table, possible_checks, &mut current_checks);
        }

        dbg!(&possible_checks.len());
    }
}

#[derive(Debug)]
enum NextStep {
    Continue,
    Accept,
    Reject,
    Goto(String),
}

#[derive(Debug, Clone)]
struct Part {
    x: isize,
    m: isize,
    a: isize,
    s: isize,
}

lazy_static! {
    static ref PART_REGEX: Regex = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
}

impl Part {
    fn parse(string: &str) -> Option<Self> {
        PART_REGEX.captures(string).and_then(|caps| {
            let mut props = caps.iter().skip(1).flatten();

            let x = Self::parse_prop(&mut props)?;
            let m = Self::parse_prop(&mut props)?;
            let a = Self::parse_prop(&mut props)?;
            let s = Self::parse_prop(&mut props)?;

            Some(Self { x, m, a, s })
        })
    }

    fn parse_prop<'a>(props: &mut impl Iterator<Item = Match<'a>>) -> Option<isize> {
        props.next()?.as_str().parse::<isize>().ok()
    }

    fn get(&self, prop: &PartProp) -> isize {
        match prop {
            PartProp::X => self.x,
            PartProp::M => self.m,
            PartProp::A => self.a,
            PartProp::S => self.s,
        }
    }

    fn sum(&self) -> isize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone)]
enum PartProp {
    X,
    M,
    A,
    S,
}

impl PartProp {
    fn parse(name: &str) -> Option<Self> {
        match name {
            "x" => Some(Self::X),
            "m" => Some(Self::M),
            "a" => Some(Self::A),
            "s" => Some(Self::S),
            _ => None,
        }
    }

    fn index(&self) -> usize {
        match self {
            PartProp::X => 0,
            PartProp::M => 1,
            PartProp::A => 2,
            PartProp::S => 3,
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    instructions: Vec<Instruction>,
}

lazy_static! {
    static ref WORKFLOW_REGEX: Regex = Regex::new(r"(\w+)\{([^}]+)\}").unwrap();
    static ref GREATER_REGEX: Regex = Regex::new(r"(\w+)>(\d+):(.+)").unwrap();
    static ref LESS_REGEX: Regex = Regex::new(r"(\w+)<(\d+):(.+)").unwrap();
}

impl Workflow {
    fn parse(string: &str) -> Option<Self> {
        if let Some(caps) = WORKFLOW_REGEX.captures(string) {
            let name = caps.get(1)?.as_str();
            let instruction_text = caps.get(2)?.as_str().split(',');

            Some(Self {
                name: String::from(name),
                instructions: instruction_text.flat_map(Instruction::parse).collect(),
            })
        } else {
            None
        }
    }

    fn possible_checks(
        &self,
        symbol_table: &HashMap<String, Workflow>,
        possible_checks: &mut Vec<Vec<Instruction>>,
        current_checks: &mut Vec<Instruction>,
    ) {
        for instruction in &self.instructions {
            if instruction.possible_checks(
                instruction,
                symbol_table,
                possible_checks,
                current_checks,
            ) {
                break;
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Accept,
    Reject,
    Greater {
        prop: PartProp,
        to_compare: isize,
        on_accept: Box<Instruction>,
    },
    Less {
        prop: PartProp,
        to_compare: isize,
        on_accept: Box<Instruction>,
    },
    Goto {
        label: String,
    },
}

impl Instruction {
    fn apply(&self, part: &Part) -> NextStep {
        match self {
            Instruction::Accept => NextStep::Accept,
            Instruction::Reject => NextStep::Reject,
            Instruction::Greater {
                prop,
                to_compare,
                on_accept,
            } => {
                if part.get(prop) > *to_compare {
                    on_accept.apply(part)
                } else {
                    NextStep::Continue
                }
            }
            Instruction::Less {
                prop,
                to_compare,
                on_accept,
            } => {
                if part.get(prop) < *to_compare {
                    on_accept.apply(part)
                } else {
                    NextStep::Continue
                }
            }
            Instruction::Goto { label } => NextStep::Goto(label.clone()),
        }
    }

    fn parse(text: &str) -> Option<Instruction> {
        if text == "A" {
            return Some(Instruction::Accept);
        } else if text == "R" {
            return Some(Instruction::Reject);
        }

        if let Some(caps) = GREATER_REGEX.captures(text) {
            let prop = PartProp::parse(caps.get(1)?.as_str())?;
            let to_compare = caps.get(2)?.as_str().parse::<isize>().ok()?;
            let on_accept = Box::new(Instruction::parse(caps.get(3)?.as_str())?);

            return Some(Instruction::Greater {
                prop,
                to_compare,
                on_accept,
            });
        }

        if let Some(caps) = LESS_REGEX.captures(text) {
            let prop = PartProp::parse(caps.get(1)?.as_str())?;
            let to_compare = caps.get(2)?.as_str().parse::<isize>().ok()?;
            let on_accept = Box::new(Instruction::parse(caps.get(3)?.as_str())?);

            return Some(Instruction::Less {
                prop,
                to_compare,
                on_accept,
            });
        }

        Some(Instruction::Goto {
            label: String::from(text),
        })
    }

    fn possible_checks(
        &self,
        instruction: &Instruction,
        symbol_table: &HashMap<String, Workflow>,
        possible_checks: &mut Vec<Vec<Instruction>>,
        current_checks: &mut Vec<Instruction>,
    ) -> bool {
        match self {
            Instruction::Accept => {
                possible_checks.push(current_checks.clone());
                return true;
            }
            Instruction::Reject => (), // Do nothing for Reject
            Instruction::Greater { on_accept, .. } | Instruction::Less { on_accept, .. } => {
                let mut divergent_checks = current_checks.clone();
                divergent_checks.push(instruction.clone());
                on_accept.possible_checks(
                    &on_accept,
                    symbol_table,
                    possible_checks,
                    &mut divergent_checks,
                );
            }
            Instruction::Goto { label } => {
                if let Some(next) = symbol_table.get(label.as_str()) {
                    next.possible_checks(symbol_table, possible_checks, current_checks)
                }
            }
        };

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\\
        px{a<2006:qkq,m>2090:A,rfg}
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

        assert_eq!(19114, part_one(input));
        assert_eq!(
            BigInt::parse_bytes(b"167409079868000", 10).unwrap(),
            part_two(input)
        );
    }

    #[test]
    fn real() {
        let input = include_str!("../res/day_19.txt");

        assert_eq!(575412, part_one(input));
        assert_eq!(
            BigInt::parse_bytes(b"167409079868000", 10).unwrap(),
            part_two(input)
        );
    }
}
