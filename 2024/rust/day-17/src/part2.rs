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

#[derive(Debug, Clone, Copy)]
struct Register {
    a: i64,
    b: i64,
    c: i64,
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Adv(i64),
    Bxl(i64),
    Bst(i64),
    Jnz(i64),
    Bxc(i64),
    Out(i64),
    Bdv(i64),
    Cdv(i64),
}

fn parse_register(input: &str) -> IResult<&str, Register> {
    let (input, a) = delimited(tag("Register A: "), complete::i64, line_ending)(input)?;
    let (input, b) = delimited(tag("Register B: "), complete::i64, line_ending)(input)?;
    let (input, c) = delimited(tag("Register C: "), complete::i64, line_ending)(input)?;

    Ok((input, Register { a, b, c }))
}

fn program_from_nums(num_repr: &Vec<i64>) -> Vec<Instruction> {
    if num_repr.len() % 2 != 0 {
        return Vec::new();
    }
    num_repr
        .into_iter()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let (a, b) = (chunk.next().unwrap(), chunk.next().unwrap());
            match a {
                0 => Instruction::Adv(*b),
                1 => Instruction::Bxl(*b),
                2 => Instruction::Bst(*b),
                3 => Instruction::Jnz(*b),
                4 => Instruction::Bxc(*b),
                5 => Instruction::Out(*b),
                6 => Instruction::Bdv(*b),
                7 => Instruction::Cdv(*b),
                _ => panic!("invalid instruction"),
            }
        })
        .collect()
}

fn parse(input: &str) -> IResult<&str, (Register, Vec<i64>)> {
    let (input, register) = parse_register(input)?;
    let (input, _) = multispace1(input)?;

    let (input, _) = tag("Program: ")(input)?;
    let (input, program) = separated_list1(tag(","), complete::i64)(input)?;
    let (input, _) = opt(line_ending)(input)?;

    Ok((input, (register, program)))
}

fn combo_op_value(register: &Register, combo_op: i64) -> i64 {
    match combo_op {
        0..=3 => combo_op,
        4 => register.a,
        5 => register.b,
        6 => register.c,
        7 => unreachable!("7 is reserved"),
        _ => panic!("combo operand out of range"),
    }
}

fn run_program(register: &mut Register, program: &Vec<Instruction>) -> Vec<i64> {
    let mut output: Vec<i64> = Vec::new();
    let mut i = 0;

    while i < program.len() {
        // debug!("register={:?}, i={}, program={:?}", register, i, program[i]);
        match program[i] {
            Instruction::Adv(combo_op) => {
                register.a = register.a / 2i64.pow(combo_op_value(&register, combo_op) as u32);
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
                register.b = register.a / 2i64.pow(combo_op_value(&register, combo_op) as u32);
                i += 1;
            }
            Instruction::Cdv(combo_op) => {
                register.c = register.a / 2i64.pow(combo_op_value(&register, combo_op) as u32);
                i += 1;
            }
        }
    }

    // debug!(?register, ?output);

    output
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<i64> {
    let (_input, (mut register, prog_nums)) =
        all_consuming(parse)(input).map_err(|e| miette!("parse failed {}", e))?;

    let prog_inst = program_from_nums(&prog_nums);
    debug!(?register, ?prog_inst);

    let mut pow;
    register.a = 8i64.pow(prog_nums.len() as u32 - 1);

    loop {
        debug!("Trying with register.a={}", register.a);
        let output = run_program(&mut register.clone(), &prog_inst);
        if output == prog_nums {
            debug!(?output, ?prog_nums);
            break;
        }

        pow = register.a.ilog(8) as i64;

        for (i, &v) in output[1..].iter().rev().enumerate() {
            if prog_nums[prog_nums.len() - 1 - i] != v {
                break;
            }
            pow -= 1;
        }

        register.a += 8i64.pow(pow as u32);
    }

    Ok(register.a)
}

#[cfg(test)]
mod day_17_part2 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!(117440, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(236581108670061, process(input)?);
        Ok(())
    }
}
