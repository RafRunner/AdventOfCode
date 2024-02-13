use std::{collections::HashMap, fmt::Debug};

use regex::{Match, Regex};

pub fn part_one(input: &str) -> isize {
    let program = Program::parse_program(input);

    program.run().iter().map(|part| part.sum()).sum()
}

#[derive(Debug)]
struct Program {
    workflows: Vec<Workflow>,
    parts: Vec<Part>,
}

impl Program {
    fn parse_program(string: &str) -> Self {
        let mut workflows = Vec::new();
        let mut parts = Vec::new();

        for line in string.lines() {
            let line = line.trim();

            if let Some(workflow) = Workflow::parse(line) {
                workflows.push(workflow);
            } else if let Some(part) = Part::parse(line) {
                parts.push(part);
            }
        }

        Self { workflows, parts }
    }

    fn run(&self) -> Vec<Part> {
        let mut accepted = Vec::new();
        let symbol_table: HashMap<&str, &Workflow> = self.workflows.iter().fold(
            HashMap::new(),
            |mut acc: HashMap<&str, &Workflow>, item| {
                acc.entry(&item.name).or_insert(item);
                acc
            },
        );

        if let Some(input) = self.workflows.iter().find(|w| w.name == "in") {
            for part in &self.parts {
                execute_workflow(input, part, &mut accepted, &symbol_table);
            }
        }

        accepted
    }
}

fn execute_workflow(
    current_workflow: &Workflow,
    part: &Part,
    accepted: &mut Vec<Part>,
    symbol_table: &HashMap<&str, &Workflow>,
) {
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
                if let Some(&next) = symbol_table.get(workflow.as_str()) {
                    execute_workflow(next, part, accepted, symbol_table)
                }
                break;
            }
        }
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

#[derive(Debug)]
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

#[derive(Debug)]
struct Workflow {
    name: String,
    instructions: Vec<Box<dyn Instruction>>,
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
                instructions: instruction_text.flat_map(parse_instruction).collect(),
            })
        } else {
            None
        }
    }
}

trait Instruction: Debug {
    fn apply(&self, part: &Part) -> NextStep;
    fn parse(string: &str) -> Option<Box<dyn Instruction>>
    where
        Self: Sized;
}

fn parse_instruction(text: &str) -> Option<Box<dyn Instruction>> {
    AcceptInstruction::parse(text)
        .or_else(|| RejectInstruction::parse(text))
        .or_else(|| GreaterInstruction::parse(text))
        .or_else(|| LessInstruction::parse(text))
        .or_else(|| GotoInstruction::parse(text))
}

#[derive(Debug)]
struct AcceptInstruction {}

impl Instruction for AcceptInstruction {
    fn apply(&self, _: &Part) -> NextStep {
        NextStep::Accept
    }

    fn parse(string: &str) -> Option<Box<dyn Instruction>>
    where
        Self: Sized,
    {
        if string == "A" {
            Some(Box::new(AcceptInstruction {}))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct RejectInstruction {}

impl Instruction for RejectInstruction {
    fn apply(&self, _: &Part) -> NextStep {
        NextStep::Reject
    }

    fn parse(string: &str) -> Option<Box<dyn Instruction>>
    where
        Self: Sized,
    {
        if string == "R" {
            Some(Box::new(RejectInstruction {}))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct GreaterInstruction {
    prop: PartProp,
    to_compare: isize,
    on_accept: Box<dyn Instruction>,
}

impl Instruction for GreaterInstruction {
    fn apply(&self, part: &Part) -> NextStep {
        if part.get(&self.prop) > self.to_compare {
            self.on_accept.apply(part)
        } else {
            NextStep::Continue
        }
    }

    fn parse(string: &str) -> Option<Box<dyn Instruction>>
    where
        Self: Sized,
    {
        if let Some(caps) = GREATER_REGEX.captures(string) {
            let prop = PartProp::parse(caps.get(1)?.as_str())?;
            let to_compare = caps.get(2)?.as_str().parse::<isize>().ok()?;
            let on_accept = caps
                .get(3)
                .and_then(|match_| parse_instruction(match_.as_str()))?;

            Some(Box::new(GreaterInstruction {
                prop,
                to_compare,
                on_accept,
            }))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct LessInstruction {
    prop: PartProp,
    to_compare: isize,
    on_accept: Box<dyn Instruction>,
}

impl Instruction for LessInstruction {
    fn apply(&self, part: &Part) -> NextStep {
        if part.get(&self.prop) < self.to_compare {
            self.on_accept.apply(part)
        } else {
            NextStep::Continue
        }
    }

    fn parse(string: &str) -> Option<Box<dyn Instruction>>
    where
        Self: Sized,
    {
        if let Some(caps) = LESS_REGEX.captures(string) {
            let prop = PartProp::parse(caps.get(1)?.as_str())?;
            let to_compare = caps.get(2)?.as_str().parse::<isize>().ok()?;
            let on_accept = caps
                .get(3)
                .and_then(|match_| parse_instruction(match_.as_str()))?;

            Some(Box::new(LessInstruction {
                prop,
                to_compare,
                on_accept,
            }))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct GotoInstruction {
    label: String,
}

impl Instruction for GotoInstruction {
    fn apply(&self, _: &Part) -> NextStep {
        NextStep::Goto(self.label.clone())
    }

    fn parse(string: &str) -> Option<Box<dyn Instruction>>
    where
        Self: Sized,
    {
        Some(Box::new(GotoInstruction {
            label: String::from(string),
        }))
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
    }

    #[test]
    fn real() {
        let input = include_str!("../res/day_19.txt");

        assert_eq!(575412, part_one(input));
    }
}
