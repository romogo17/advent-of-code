use std::collections::BTreeMap;

use crate::custom_error::AocError;
use tracing::{debug, info};

use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{self, alpha1, multispace0},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult, Parser,
};

#[derive(Debug)]
enum Operation<'a> {
    Remove { label: &'a str },
    Upsert { label: &'a str, focal_length: u8 },
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u8,
}

#[derive(Debug)]
struct LensBox {
    lenses: Vec<Lens>,
}

impl LensBox {
    fn new() -> Self {
        Self { lenses: Vec::new() }
    }

    fn upsert(&mut self, label: &str, focal_length: u8) {
        match self.lenses.iter_mut().find(|lens| lens.label == label) {
            Some(lens) => lens.focal_length = focal_length,
            None => self.lenses.push(Lens {
                label: label.to_string(),
                focal_length,
            }),
        }
    }

    fn remove(&mut self, label: &str) {
        self.lenses.retain(|lens| lens.label != label);
    }
}

fn hash(input: &str) -> u8 {
    input
        .trim()
        .chars()
        .fold(0u64, |acc, c| (acc + c as u64) * 17u64) as u8
}

fn parse_operations(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, output) = all_consuming(terminated(
        separated_list1(
            tag(","),
            tuple((alpha1, is_a("=-"), opt(complete::u8))).map(|(label, op_type, focal_length)| {
                match op_type {
                    "=" => Operation::Upsert {
                        label,
                        focal_length: focal_length.unwrap(),
                    },
                    "-" => Operation::Remove { label },
                    _ => panic!("unexpected operation type"),
                }
            }),
        ),
        multispace0,
    ))(input)?;

    Ok((input, output))
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, init_sequence) = parse_operations(input).expect("a valid init sequence parse");
    let mut boxes: BTreeMap<u8, LensBox> = BTreeMap::new();

    for op in init_sequence {
        match op {
            Operation::Upsert {
                label,
                focal_length,
            } => {
                let box_num = hash(label);
                info!(
                    "Box {}: upserting {:?} with focal length {}",
                    box_num, label, focal_length
                );
                boxes
                    .entry(box_num)
                    .and_modify(|lens_box| lens_box.upsert(label, focal_length))
                    .or_insert_with(|| {
                        let mut lens_box = LensBox::new();
                        lens_box.upsert(label, focal_length);
                        lens_box
                    });
            }
            Operation::Remove { label } => {
                let box_num = hash(label);
                info!("Box {}: removing {:?}", box_num, label);
                boxes
                    .entry(box_num)
                    .and_modify(|lens_box| lens_box.remove(label));
            }
        }
        debug!("{:?}", boxes);
    }

    let focusing_power: u64 = boxes
        .iter()
        .map(|(box_num, lens_box)| {
            lens_box
                .lenses
                .iter()
                .enumerate()
                .map(|(idx, lens)| {
                    // The focusing power of a single lens is the result of multiplying together:
                    // - One plus the box number of the lens in question.
                    // - The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
                    // - The focal length of the lens.
                    (*box_num as u64 + 1) * (idx as u64 + 1) * lens.focal_length as u64
                })
                .sum::<u64>()
        })
        .sum();

    info!("Focusing power: {}", focusing_power);

    Ok(focusing_power)
}

#[cfg(test)]
mod day_15_part2 {
    use super::*;

    #[test]
    fn example() -> miette::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(145, process(input)?);
        Ok(())
    }

    #[test]
    fn input1() -> miette::Result<()> {
        let input = include_str!("../inputs/input1.txt");
        assert_eq!(271384, process(input)?);
        Ok(())
    }
}
