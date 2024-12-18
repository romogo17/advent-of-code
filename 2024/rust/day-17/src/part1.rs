use itertools::Itertools;
use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace1},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};
use tracing::debug;

#[derive(Debug)]
struct Register {
    a: i32,
    b: i32,
    c: i32,
}

#[derive(Debug)]
enum Instruction {
    Adv(i32),
    Bxl(i32),
    Bst(i32),
    Jnz(i32),
    Bxc(i32),
    Out(i32),
    Bdv(i32),
    Cdv(i32),
}

fn parse_register(input: &str) -> IResult<&str, Register> {
    let (input, a) = delimited(tag("Register A: "), complete::i32, line_ending)(input)?;
    let (input, b) = delimited(tag("Register B: "), complete::i32, line_ending)(input)?;
    let (input, c) = delimited(tag("Register C: "), complete::i32, line_ending)(input)?;

    Ok((input, Register { a, b, c }))
}

fn parse(input: &str) -> IResult<&str, (Register, Vec<Instruction>)> {
    let (input, register) = parse_register(input)?;
    let (input, _) = multispace1(input)?;

    let (input, _) = tag("Program: ")(input)?;
    let (input, program) = separated_list1(tag(","), complete::i32)(input)?;
    let (input, _) = opt(line_ending)(input)?;

    let program = program
        .into_iter()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let (a, b) = (chunk.next().unwrap(), chunk.next().unwrap());
            match a {
                0 => Instruction::Adv(b),
                1 => Instruction::Bxl(b),
                2 => Instruction::Bst(b),
                3 => Instruction::Jnz(b),
                4 => Instruction::Bxc(b),
                5 => Instruction::Out(b),
                6 => Instruction::Bdv(b),
                7 => Instruction::Cdv(b),
                _ => panic!("invalid instruction"),
            }
        })
        .collect();

    Ok((input, (register, program)))
}

fn combo_op_value(register: &Register, combo_op: i32) -> i32 {
    match combo_op {
        0..=3 => combo_op,
        4 => register.a,
        5 => register.b,
        6 => register.c,
        7 => unreachable!("7 is reserved"),
        _ => panic!("combo operand out of range"),
    }
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, (mut register, program)) =
        all_consuming(parse)(input).map_err(|e| miette!("parse failed {}", e))?;
    debug!(?register, ?program);

    let mut output: Vec<i32> = Vec::new();
    let mut i = 0;

    while i < program.len() {
        debug!("register={:?}, i={}, program={:?}", register, i, program[i]);
        match program[i] {
            Instruction::Adv(combo_op) => {
                register.a = register.a / 2i32.pow(combo_op_value(&register, combo_op) as u32);
                i += 1;
            }
            Instruction::Bxl(literal_op) => {
                register.b = register.b ^ literal_op;
                i += 1;
            }
            Instruction::Bst(combo_op) => {
                register.b = combo_op_value(&register, combo_op) % 8;
                i += 1;
            }
            Instruction::Jnz(literal_op) => match register.a {
                0 => {
                    i += 1;
                }
                _ => {
                    i = (literal_op / 2) as usize;
                }
            },
            Instruction::Bxc(_combo_op) => {
                register.b = register.b ^ register.c;
                i += 1;
            }
            Instruction::Out(combo_op) => {
                output.push(combo_op_value(&register, combo_op) % 8);
                i += 1;
            }
            Instruction::Bdv(combo_op) => {
                register.b = register.a / 2i32.pow(combo_op_value(&register, combo_op) as u32);
                i += 1;
            }
            Instruction::Cdv(combo_op) => {
                register.c = register.a / 2i32.pow(combo_op_value(&register, combo_op) as u32);
                i += 1;
            }
        }
    }

    debug!(?register, ?output);

    Ok(output.iter().join(","))
}

#[cfg(test)]
mod day_17_part1 {
    use super::*;

    #[test_log::test]
    fn small_example1() -> miette::Result<()> {
        let input = "Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4";
        assert_eq!("0,1,2", process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn small_example2() -> miette::Result<()> {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("4,2,5,6,7,7,7,7,3,1,0", process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("4,6,3,5,6,3,5,2,1,0", process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!("1,5,0,5,2,0,1,3,5", process(input)?);
        Ok(())
    }
}
