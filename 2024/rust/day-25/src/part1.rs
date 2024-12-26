use itertools::Itertools;
use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::{iterator, opt, peek},
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
enum DeviceType {
    Lock,
    Key,
}

#[derive(Debug)]
struct Device {
    r#type: DeviceType,
    pins: [i32; 5],
}

fn pins(mut pins: [i32; 5]) -> impl FnMut(&str) -> IResult<&str, [i32; 5]> {
    move |input| {
        let mut it = iterator(
            input,
            terminated(alt((tag("#"), tag("."))), opt(line_ending)),
        );

        for (i, value) in it.enumerate() {
            pins[i % 5] += match value {
                "#" => 1,
                _ => 0,
            };
        }
        let res: IResult<_, _> = it.finish();
        res.map(|(input, _)| (input, pins))
    }
}

fn lock(input: &str) -> IResult<&str, Device> {
    let (input, _) = tag("#####")(input)?;
    let (input, pins) = preceded(
        tuple((line_ending, peek(alt((tag("."), tag("#")))))),
        pins([0i32; 5]),
    )(input)?;

    Ok((
        input,
        Device {
            r#type: DeviceType::Lock,
            pins,
        },
    ))
}

fn key(input: &str) -> IResult<&str, Device> {
    let (input, _) = tag(".....")(input)?;
    let (input, pins) = preceded(
        tuple((line_ending, peek(alt((tag("."), tag("#")))))),
        pins([-1i32; 5]),
    )(input)?;

    Ok((
        input,
        Device {
            r#type: DeviceType::Key,
            pins,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Device>> {
    separated_list1(line_ending, alt((key, lock)))(input)
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u32> {
    let (_input, mut data) = parse(input).map_err(|e| miette!("parse failed {}", e))?;

    let locks = data
        .extract_if(.., |device| device.r#type == DeviceType::Lock)
        .collect::<Vec<Device>>();
    let keys = data;

    let count = locks
        .iter()
        .cartesian_product(keys.iter())
        .filter(|(lock, key)| std::iter::zip(lock.pins, key.pins).all(|(a, b)| a + b <= 5))
        .count();

    Ok(count as u32)
}

#[cfg(test)]
mod day_25_part1 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        assert_eq!(3, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(3090, process(input)?);
        Ok(())
    }
}
