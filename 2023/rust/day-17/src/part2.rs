use crate::custom_error::AocError;
use glam::IVec2;
use itertools::Itertools;
use nom::{
    character::complete::{line_ending, multispace0, one_of},
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};
use nom_locate::{position, LocatedSpan};
use pathfinding::prelude::dijkstra;
use std::collections::{HashMap, VecDeque};
use tracing::debug;

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Block {
    pos: IVec2,
    prev: VecDeque<IVec2>,
}

impl Block {
    fn new(pos: IVec2, prev: VecDeque<IVec2>) -> Self {
        Self { pos, prev }
    }
}

fn a_num(input: Span) -> IResult<Span, (IVec2, u64)> {
    let (input, pos) = position(input)?;
    let (input, num) = one_of("0123456789")(input)?;

    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;

    Ok((input, (IVec2::new(x, y), num.to_digit(10).unwrap().into())))
}

fn parse_grid(input: Span) -> IResult<Span, HashMap<IVec2, u64>> {
    let (input, output) = all_consuming(terminated(
        separated_list1(line_ending, many1(a_num)),
        multispace0,
    ))(input)?;

    Ok((
        input,
        output
            .into_iter()
            .flatten()
            .collect::<HashMap<IVec2, u64>>(),
    ))
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let row_count = input.lines().count() as i32;
    let col_count = input.lines().next().unwrap().chars().count() as i32;
    let (_, grid) = parse_grid(Span::new(input)).expect("a valid grid parse");

    let goal = IVec2::new(col_count as i32 - 1, row_count as i32 - 1);
    let result: (Vec<Block>, u64) = dijkstra(
        &Block::new(IVec2::splat(0), VecDeque::from([IVec2::splat(0)])),
        |block| {
            let diffs: Vec<IVec2> = block
                .prev
                .iter()
                .tuple_windows()
                .map(|(a, b)| *a - *b)
                .collect();
            let last_diff = diffs.get(0);
            let maybe_first_diff_count = diffs.iter().dedup_with_count().next();
            let options = if let Some(diff_count) = maybe_first_diff_count {
                let num_consecutive_straight_diffs = diff_count.0;
                let must_turn = num_consecutive_straight_diffs == 10;
                let must_go_straight = num_consecutive_straight_diffs < 4;

                if must_turn {
                    [
                        IVec2::new(-1, 0),
                        IVec2::new(1, 0),
                        IVec2::new(0, -1),
                        IVec2::new(0, 1),
                    ]
                    .into_iter()
                    .filter(|option| option != last_diff.unwrap())
                    .map(|option| block.pos + option)
                    .collect::<Vec<IVec2>>()
                } else if must_go_straight {
                    vec![block.pos + *last_diff.unwrap()]
                } else {
                    vec![
                        block.pos + IVec2::new(-1, 0),
                        block.pos + IVec2::new(1, 0),
                        block.pos + IVec2::new(0, -1),
                        block.pos + IVec2::new(0, 1),
                    ]
                }
            } else {
                debug!("kickstarting the search options");
                vec![
                    block.pos + IVec2::new(-1, 0),
                    block.pos + IVec2::new(1, 0),
                    block.pos + IVec2::new(0, -1),
                    block.pos + IVec2::new(0, 1),
                ]
            };

            options
                .into_iter()
                .filter_map(|candidate_pos| {
                    // Canditate position not in the grid
                    if !(0..col_count).contains(&candidate_pos.x)
                        || !(0..row_count).contains(&candidate_pos.y)
                    {
                        debug!(%candidate_pos, "candidate position not in the grid");
                        return None;
                    }

                    // Going backwards
                    if block.prev.len() > 2 && block.prev[1] == candidate_pos {
                        debug!(%candidate_pos, prev = ?block.prev, "going backwards");
                        return None;
                    }

                    let mut prevs = block.prev.clone();
                    prevs.push_front(candidate_pos);
                    if prevs.len() > 14 {
                        prevs.pop_back();
                    }
                    Some(Block::new(candidate_pos, prevs))
                })
                .map(|block| {
                    let cost = *grid.get(&block.pos).unwrap();
                    (block, cost)
                })
                .collect::<Vec<(Block, u64)>>()
        },
        |block| {
            let diffs: Vec<IVec2> = block
                .prev
                .iter()
                .tuple_windows()
                .map(|(a, b)| *a - *b)
                .collect();
            let maybe_first_diff_count = diffs.iter().dedup_with_count().next();

            maybe_first_diff_count.is_some_and(|(count, _)| count >= 4) && block.pos == goal
        },
    )
    .expect("should have a valid path");

    Ok(result.1)
}

#[cfg(test)]
mod day_17_part2 {
    use super::*;

    #[test_log::test]
    fn example1() -> miette::Result<()> {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(94, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn example2() -> miette::Result<()> {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";
        assert_eq!(71, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input1() -> miette::Result<()> {
        let input = include_str!("../inputs/input1.txt");
        assert_eq!(982, process(input)?);
        Ok(())
    }
}
