use std::{collections::HashMap, fmt::Debug, ops::Range};

use regex::{Match, Regex};

pub fn part_one(input: &str) -> isize {
    let program = Program::parse_program(input);

    program.run().iter().map(Part::sum).sum()
}

pub fn part_two(input: &str) -> usize {
    let program = Program::parse_program(input);
    let possible_checks = program.possible_checks();

    possible_checks
        .iter()
        .map(PartRange::number_of_possibilities)
        .sum()
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
                symbol_table.insert(workflow.name.clone(), workflow);
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
        let input = self.symbol_table.get("in").expect("No input workflow");

        let mut accepted = Vec::new();

        for part in &self.parts {
            self.execute_workflow(input, part, &mut accepted);
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
                    let next = self
                        .symbol_table
                        .get(workflow.as_str())
                        .unwrap_or_else(|| panic!("Workflow {workflow} not found"));
                    self.execute_workflow(next, part, accepted);
                    break;
                }
            };
        }
    }

    fn possible_checks(&self) -> Vec<PartRange> {
        let input = self.symbol_table.get("in").expect("No input workflow");

        let mut ranges = Vec::new();
        let mut range = PartRange::new();

        input.possible_checks(&self.symbol_table, &mut ranges, &mut range);

        ranges
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
}

#[derive(Debug, Clone)]
struct PartRange {
    x: Range<isize>,
    m: Range<isize>,
    a: Range<isize>,
    s: Range<isize>,
}

impl PartRange {
    fn new() -> Self {
        Self {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
    }

    fn get_mut(&mut self, prop: &PartProp) -> &mut Range<isize> {
        match prop {
            PartProp::X => &mut self.x,
            PartProp::M => &mut self.m,
            PartProp::A => &mut self.a,
            PartProp::S => &mut self.s,
        }
    }

    fn number_of_possibilities(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    instructions: Vec<Instruction>,
}

lazy_static! {
    static ref WORKFLOW_REGEX: Regex = Regex::new(r"(\w+)\{([^}]+)\}").unwrap();
    static ref GREATER_LESS_REGEX: Regex = Regex::new(r"(\w+)([><])(\d+):(.+)").unwrap();
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
        ranges: &mut Vec<PartRange>,
        range: &mut PartRange,
    ) {
        for instruction in &self.instructions {
            if instruction.possible_checks(symbol_table, ranges, range) {
                break;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ComparisonType {
    Greater,
    Less,
}

#[derive(Debug, Clone)]
enum Instruction {
    Accept,
    Reject,
    Comparison {
        prop: PartProp,
        kind: ComparisonType,
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
            Instruction::Comparison {
                prop,
                kind,
                to_compare,
                on_accept,
            } => match kind {
                ComparisonType::Greater if part.get(prop) > *to_compare => on_accept.apply(part),
                ComparisonType::Less if part.get(prop) < *to_compare => on_accept.apply(part),
                _ => NextStep::Continue,
            },
            Instruction::Goto { label } => NextStep::Goto(label.clone()),
        }
    }

    fn parse(text: &str) -> Option<Instruction> {
        if text == "A" {
            return Some(Instruction::Accept);
        }
        if text == "R" {
            return Some(Instruction::Reject);
        }

        if let Some(caps) = GREATER_LESS_REGEX.captures(text) {
            let prop = PartProp::parse(caps.get(1)?.as_str())?;
            let simbol = caps.get(2)?.as_str();
            let to_compare = caps.get(3)?.as_str().parse::<isize>().ok()?;
            let on_accept = Box::new(Instruction::parse(caps.get(4)?.as_str())?);

            let kind = match simbol {
                ">" => ComparisonType::Greater,
                "<" => ComparisonType::Less,
                _ => panic!("Invalid comparison sign {simbol}"),
            };

            return Some(Instruction::Comparison {
                prop,
                kind,
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
        symbol_table: &HashMap<String, Workflow>,
        ranges: &mut Vec<PartRange>,
        range: &mut PartRange,
    ) -> bool {
        match self {
            Instruction::Accept => {
                ranges.push(range.clone());
                return true;
            }
            Instruction::Reject => return true, // Break on Reject
            Instruction::Comparison {
                on_accept,
                kind,
                prop,
                to_compare,
            } => {
                let mut true_range = range.clone();
                let mut_true = true_range.get_mut(prop);
                let mut_false = range.get_mut(prop);

                match kind {
                    ComparisonType::Greater => {
                        mut_true.start = to_compare + 1;
                        mut_false.end = to_compare + 1;
                    }
                    ComparisonType::Less => {
                        mut_true.end = *to_compare;
                        mut_false.start = *to_compare;
                    }
                };

                on_accept.possible_checks(symbol_table, ranges, &mut true_range);
            }
            Instruction::Goto { label } => {
                let next = symbol_table
                    .get(label.as_str())
                    .unwrap_or_else(|| panic!("Workflow {label} not found"));
                next.possible_checks(symbol_table, ranges, range);
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
        assert_eq!(167409079868000, part_two(input));
    }

    #[test]
    fn real() {
        let input = include_str!("../res/day_19.txt");

        assert_eq!(575412, part_one(input));
        assert_eq!(126107942006821, part_two(input));
    }
}
