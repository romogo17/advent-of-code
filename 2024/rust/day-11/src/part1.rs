use miette::miette;
use nom::{
    character::complete::{self, space1},
    combinator::map,
    multi::separated_list1,
    IResult,
};
use num::traits::Euclid;
use tracing::debug;

#[derive(Debug, PartialEq, Eq)]
enum Stone {
    Zero,
    EvenDigits(u64),
    Default(u64),
}

impl Stone {
    fn blink(&self) -> Vec<Stone> {
        match self {
            Stone::Zero => vec![Stone::Default(1)],
            Stone::EvenDigits(num) => {
                let num_digits = num.checked_ilog10().unwrap_or(0) + 1;
                let (left, right) = num.div_rem_euclid(&10u64.pow(num_digits / 2));

                let left = match left {
                    0 => Stone::Zero,
                    num if (num.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 => {
                        Stone::EvenDigits(num)
                    }
                    num => Stone::Default(num),
                };
                let right = match right {
                    0 => Stone::Zero,
                    num if (num.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 => {
                        Stone::EvenDigits(num)
                    }
                    num => Stone::Default(num),
                };

                vec![left, right]
            }
            Stone::Default(num) => {
                let new_num = num * 2024;

                if (new_num.ilog10() + 1) % 2 == 0 {
                    vec![Stone::EvenDigits(new_num)]
                } else {
                    vec![Stone::Default(new_num)]
                }
            }
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Stone>> {
    separated_list1(
        space1,
        map(complete::u64, |num| match num {
            0 => Stone::Zero,
            num if (num.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 => Stone::EvenDigits(num),
            num => Stone::Default(num),
        }),
    )(input)
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64> {
    let (_input, mut stones) = parse(input).map_err(|e| miette!("parse failed {}", e))?;
    debug!(?stones);

    for blink in 0..25 {
        debug!("Blink {}", blink);
        stones = stones.iter().flat_map(|stone| stone.blink()).collect();
        debug!(?stones)
    }

    Ok(stones.len() as u64)
}

#[cfg(test)]
mod day_11_part1 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!(55312, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(189167, process(input)?);
        Ok(())
    }
}
