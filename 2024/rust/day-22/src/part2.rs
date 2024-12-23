use std::{collections::HashMap, iter::successors};

use itertools::Itertools;
use miette::IntoDiagnostic;

// To mix a value into the secret number, calculate the bitwise XOR of the given value and the secret number.
// Then, the secret number becomes the result of that operation.
fn mix(secret: usize, value: usize) -> usize {
    secret ^ value
}
// To prune the secret number, calculate the value of the secret number modulo 16777216.
// Then, the secret number becomes the result of that operation.
fn prune(secret: usize) -> usize {
    secret.rem_euclid(16777216)
}

fn process_secret(secret: &str) -> miette::Result<impl Iterator<Item = usize>> {
    let secret = secret.parse::<usize>().into_diagnostic()?;

    Ok(successors(Some(secret), |secret| {
        // Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
        let value = secret * 64;
        let secret = prune(mix(*secret, value));
        // Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
        let value = secret / 32;
        let secret = prune(mix(secret, value));
        // Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.
        let value = secret * 2048;
        let secret = prune(mix(secret, value));

        Some(secret)
    }))
}

fn cost_and_delta(secret: &str) -> miette::Result<impl Iterator<Item = (usize, i32)>> {
    Ok(process_secret(secret)
        .unwrap()
        .map(|num| num % 10)
        .tuple_windows()
        .map(|(a, b)| (b, b as i32 - a as i32)))
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let hashmap = input
        .lines()
        .fold(HashMap::<[i32; 4], usize>::new(), |mut map, line| {
            let inner_map = cost_and_delta(line)
                .unwrap()
                .take(2000)
                .tuple_windows()
                .fold(
                    HashMap::<[i32; 4], usize>::with_capacity(1994),
                    |mut map, (a, b, c, d)| {
                        let key = [a.1, b.1, c.1, d.1];
                        map.entry(key).or_insert(d.0);
                        map
                    },
                );

            for (key, inner_value) in inner_map.into_iter() {
                map.entry(key)
                    .and_modify(|value| *value += inner_value)
                    .or_insert(inner_value);
            }

            map
        });

    let result: &usize = &hashmap.values().max().unwrap();

    Ok(result.to_string())
}
#[cfg(test)]
mod day_22_part2 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "1
2
3
2024";
        assert_eq!("23", process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!("1808", process(input)?);
        Ok(())
    }
}
