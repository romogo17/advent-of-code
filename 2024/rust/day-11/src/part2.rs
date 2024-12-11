use std::collections::HashMap;

use miette::miette;
use nom::{
    character::complete::{self, space1},
    multi::separated_list1,
    IResult,
};
use num::traits::Euclid;

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, complete::u64)(input)
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64> {
    let (_input, stones) = parse(input).map_err(|e| miette!("parse failed {}", e))?;

    // stone number, stone count
    let mut cache: HashMap<u64, u64> = HashMap::default();

    for stone in stones {
        cache
            .entry(stone)
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert(1);
    }

    for _ in 0..75 {
        let mut new_cache: HashMap<u64, u64> = HashMap::default();

        for (num, count) in cache.into_iter() {
            match num {
                0 => {
                    new_cache
                        .entry(1)
                        .and_modify(|v| {
                            *v += count;
                        })
                        .or_insert(count);
                }
                num if (num.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 => {
                    let num_digits = num.checked_ilog10().unwrap_or(0) + 1;
                    let (left, right) = num.div_rem_euclid(&10u64.pow(num_digits / 2));

                    new_cache
                        .entry(left)
                        .and_modify(|v| {
                            *v += count;
                        })
                        .or_insert(count);
                    new_cache
                        .entry(right)
                        .and_modify(|v| {
                            *v += count;
                        })
                        .or_insert(count);
                }
                num => {
                    new_cache
                        .entry(num * 2024)
                        .and_modify(|v| {
                            *v += count;
                        })
                        .or_insert(count);
                }
            }
        }
        cache = new_cache;
    }

    Ok(cache.values().sum())
}

#[cfg(test)]
mod day_11_2 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!(65601038650482, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(225253278506288, process(input)?);
        Ok(())
    }
}
