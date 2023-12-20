use std::cmp::Ordering;
use std::collections::HashMap;

use crate::custom_error::AocError;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, multispace1},
    combinator::opt,
    multi::{fold_many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

// The puzzle uses only less than and greater than. Otherwise we would use Ordering
#[derive(Debug, PartialEq, Eq)]
enum Condition {
    GreaterThan,
    LessThan,
}

#[derive(Debug, PartialEq, Eq)]
enum Target<'a> {
    Workflow(&'a str),
    Accepted,
    Rejected,
}

#[derive(Debug, PartialEq, Eq)]
enum Rule<'a> {
    Test {
        attribute: &'a str,
        condition: Condition,
        value: u64,
        target: Target<'a>,
    },
    Target(Target<'a>),
}

impl<'a> Rule<'a> {
    fn next_target(&self, part: &Part) -> Option<&Target> {
        match self {
            Rule::Test {
                attribute,
                condition,
                value,
                target,
            } => {
                let attribute_value = match *attribute {
                    "x" => part.x,
                    "m" => part.m,
                    "a" => part.a,
                    "s" => part.s,
                    _ => panic!("unknown attribute {}", attribute),
                };
                let ordering = match condition {
                    Condition::LessThan => Ordering::Less,
                    Condition::GreaterThan => Ordering::Greater,
                };
                (attribute_value.cmp(value) == ordering).then_some(target)
            }
            Rule::Target(target) => Some(target),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Workflow<'a> {
    id: &'a str,
    rules: Vec<Rule<'a>>,
}

#[derive(Debug, PartialEq, Eq, Default)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

fn target(input: &str) -> IResult<&str, Target> {
    alt((
        tag("A").map(|_| Target::Accepted),
        tag("R").map(|_| Target::Rejected),
        alpha1.map(|workflow_id| Target::Workflow(workflow_id)),
    ))(input)
}

fn rule_test(input: &str) -> IResult<&str, Rule> {
    let (input, attribute) = alpha1(input)?;
    let (input, condition) = alt((
        tag("<").map(|_| Condition::LessThan),
        tag(">").map(|_| Condition::GreaterThan),
    ))(input)?;
    let (input, value) = complete::u64(input)?;
    let (input, _) = complete::char(':')(input)?;
    let (input, target) = target(input)?;
    Ok((
        input,
        Rule::Test {
            attribute,
            condition,
            value,
            target,
        },
    ))
}

fn workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, id) = alpha1(input)?;
    let (input, rules) = delimited(
        complete::char('{'),
        separated_list1(
            complete::char(','),
            alt((rule_test, target.map(Rule::Target))),
        ),
        complete::char('}'),
    )(input)?;
    Ok((input, Workflow { id, rules }))
}

fn workflows(input: &str) -> IResult<&str, HashMap<&str, Workflow>> {
    let (input, workflows) = separated_list1(line_ending, workflow)(input)?;
    Ok((input, workflows.into_iter().map(|w| (w.id, w)).collect()))
}

fn part(input: &str) -> IResult<&str, Part> {
    delimited(
        complete::char('{'),
        fold_many1(
            terminated(
                separated_pair(alpha1, complete::char('='), complete::u64),
                opt(tag(",")),
            ),
            Part::default,
            |mut part, (next_attribute, count)| {
                match next_attribute {
                    "x" => part.x = count,
                    "m" => part.m = count,
                    "a" => part.a = count,
                    "s" => part.s = count,
                    _ => panic!("unknown attribute {}", next_attribute),
                }
                part
            },
        ),
        complete::char('}'),
    )(input)
}

fn parts(input: &str) -> IResult<&str, Vec<Part>> {
    separated_list1(line_ending, part)(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<Part>, HashMap<&str, Workflow>)> {
    let (input, workflows) = workflows(input)?;
    let (input, _) = multispace1(input)?;
    let (input, parts) = parts(input)?;
    Ok((input, (parts, workflows)))
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_input, (parts, workflows)) = parse(input).expect("should parse the puzzle input");

    let results = parts
        .iter()
        .filter_map(|part| {
            let mut workflow_id = "in";
            let last_target: Target = 'workflow_loop: loop {
                let workflow = workflows.get(workflow_id).expect("should have a workflow");

                'rule_loop: for rule in workflow.rules.iter() {
                    match rule.next_target(part) {
                        Some(Target::Accepted) => {
                            break 'workflow_loop Target::Accepted;
                        }
                        Some(Target::Rejected) => {
                            break 'workflow_loop Target::Rejected;
                        }
                        Some(Target::Workflow(next_workflow_id)) => {
                            workflow_id = next_workflow_id;
                            break 'rule_loop;
                        }
                        None => {}
                    }
                }
            };
            match last_target {
                Target::Workflow(_) => panic!("should not have a workflow as last target"),
                Target::Accepted => Some(part.x + part.m + part.a + part.s),
                Target::Rejected => None,
            }
        })
        .sum::<u64>();

    Ok(results)
}

#[cfg(test)]
mod day_19_part1 {
    use super::*;

    #[test_log::test]
    fn workflow_parser() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}";
        assert_eq!(
            Workflow {
                id: "px",
                rules: vec![
                    Rule::Test {
                        attribute: "a",
                        condition: Condition::LessThan,
                        value: 2006,
                        target: Target::Workflow("qkq")
                    },
                    Rule::Test {
                        attribute: "m",
                        condition: Condition::GreaterThan,
                        value: 2090,
                        target: Target::Accepted
                    },
                    Rule::Target(Target::Workflow("rfg"))
                ]
            },
            workflow(input).expect("should parse a workflow").1
        )
    }

    #[test_log::test]
    fn part_parser() {
        let input = "{x=787,m=2655,a=1222,s=2876}";
        assert_eq!(
            Part {
                x: 787,
                m: 2655,
                a: 1222,
                s: 2876
            },
            part(input).expect("should parse a part").1
        )
    }

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
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
        assert_eq!(19114, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input1() -> miette::Result<()> {
        let input = include_str!("../inputs/input1.txt");
        assert_eq!(509597, process(input)?);
        Ok(())
    }
}
