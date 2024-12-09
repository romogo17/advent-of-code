use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};
use tracing::info;

fn parse(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(
        line_ending,
        nom::sequence::separated_pair(
            complete::u64,
            tag(": "),
            separated_list1(space1, complete::u64),
        ),
    )(input)
}

fn is_valid((test_value, equation): &(u64, Vec<u64>)) -> bool {
    let tree_size = (3usize.pow(equation.len() as u32) - 1) / 2usize;
    let max_rank = equation.len() - 1;

    let mut queue = vec![0];
    let mut results = vec![0; tree_size];
    results[0] = equation[0];

    // BFS traversal of the tree, calculating the result of each node
    while !queue.is_empty() {
        let index = queue.pop().unwrap();
        let rank = ((2.0 * index as f32 + 1.0).ln() / 3.0f32.ln()).floor() as usize;

        if index < tree_size {
            if rank < max_rank {
                let left = 3 * index + 1;
                let center = 3 * index + 2;
                let right = 3 * index + 3;

                if left < tree_size {
                    queue.push(left);
                    results[left] = results[index] + equation[rank + 1];
                }

                if center < tree_size {
                    queue.push(center);
                    results[center] = format!("{}{}", results[index], equation[rank + 1])
                        .parse::<u64>()
                        .unwrap();
                }

                if right < tree_size {
                    queue.push(right);
                    results[right] = results[index] * equation[rank + 1];
                }
            } else {
                // we already calculated the results for the nodes on the last rank
                if results[index] == *test_value {
                    return true;
                }
            }
        }
    }

    false
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64> {
    let (_input, equations) = parse(input).map_err(|e| miette!("parse failed {}", e))?;

    let result: u64 = equations
        .iter()
        .filter_map(|equation| {
            return match is_valid(equation) {
                true => {
                    info!("valid {:?}", equation);
                    Some(equation.0)
                }
                false => None,
            };
        })
        .sum();

    Ok(result)
}

#[cfg(test)]
mod day_07_part2 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(11387, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(104824810233437, process(input)?);
        Ok(())
    }
}
