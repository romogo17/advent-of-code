use crate::custom_error::AocError;
use glam::IVec2;
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
            [
                block.pos + IVec2::new(-1, 0),
                block.pos + IVec2::new(1, 0),
                block.pos + IVec2::new(0, -1),
                block.pos + IVec2::new(0, 1),
            ]
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
                if prevs.len() == 5 {
                    let dir = prevs[1] - prevs[0];

                    // moving in the same direction four times
                    let moving_in_same_dir = [
                        prevs[2] - prevs[1],
                        prevs[3] - prevs[2],
                        prevs[4] - prevs[3],
                    ]
                    .iter()
                    .all(|a_dir| a_dir == &dir);

                    if moving_in_same_dir {
                        debug!(%candidate_pos, prev = ?block.prev, "going in the same direction four times");
                        return None;
                    } else {
                        prevs.pop_back();
                        return Some(Block::new(candidate_pos, prevs));
                    }
                }
                Some(Block::new(candidate_pos, prevs))
            })
            .map(|block| {
                let cost = *grid.get(&block.pos).unwrap();
                (block, cost)
            })
            .collect::<Vec<(Block, u64)>>()
        },
        |block| block.pos == goal,
    )
    .expect("should have a valid path");

    Ok(result.1)
}

#[cfg(test)]
mod day_17_part1 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
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
        assert_eq!(102, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input1() -> miette::Result<()> {
        let input = include_str!("../inputs/input1.txt");
        assert_eq!(851, process(input)?);
        Ok(())
    }
}
