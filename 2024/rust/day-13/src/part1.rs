use glam::IVec2;
use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};
use tracing::debug;

#[derive(Debug)]
struct ClawMachine {
    a: IVec2,
    b: IVec2,
    prize: IVec2,
}

fn a_button(input: &str) -> IResult<&str, IVec2> {
    preceded(
        tag("Button A: X+"),
        separated_pair(complete::i32, tag(", Y+"), complete::i32).map(|(x, y)| IVec2::new(x, y)),
    )(input)
}

fn b_button(input: &str) -> IResult<&str, IVec2> {
    preceded(
        tag("Button B: X+"),
        separated_pair(complete::i32, tag(", Y+"), complete::i32).map(|(x, y)| IVec2::new(x, y)),
    )(input)
}

fn prize(input: &str) -> IResult<&str, IVec2> {
    preceded(
        tag("Prize: X="),
        separated_pair(complete::i32, tag(", Y="), complete::i32).map(|(x, y)| IVec2::new(x, y)),
    )(input)
}

fn claw_machine(input: &str) -> IResult<&str, ClawMachine> {
    let (input, (a, b, prize)) = tuple((
        terminated(a_button, line_ending),
        terminated(b_button, line_ending),
        prize,
    ))(input)?;

    Ok((input, ClawMachine { a, b, prize }))
}

fn parse(input: &str) -> IResult<&str, Vec<ClawMachine>> {
    separated_list1(tuple((line_ending, line_ending)), claw_machine)(input)
}

fn solve(claw_machine: &ClawMachine) -> (i32, i32) {
    let (x1, x2) = claw_machine.a.into();
    let (y1, y2) = claw_machine.b.into();
    let (z1, z2) = claw_machine.prize.into();

    let b = (z2 * x1 - z1 * x2) / (y2 * x1 - y1 * x2);
    let a = (z1 - b * y1) / x1;
    if (x1 * a + y1 * b, x2 * a + y2 * b) != (z1, z2) {
        return (0, 0);
    }
    debug!("{}a + {}b = {}; {}a + {}b = {}", x1, y1, z1, x2, y2, z2);
    debug!("a={}, b={}", a, b);
    (a, b)
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<i32> {
    let (_input, claw_machines) = parse(input).map_err(|e| miette!("parse failed {}", e))?;
    let result = claw_machines
        .iter()
        .map(|claw_machine| {
            let (a, b) = solve(&claw_machine);
            a * 3 + b
        })
        .sum::<i32>();
    Ok(result)
}

#[cfg(test)]
mod day_13_part1 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(480, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(33481, process(input)?);
        Ok(())
    }
}
