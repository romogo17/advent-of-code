use crate::custom_error::AocError;
use glam::IVec3;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Brick {
    start: IVec3,
    end: IVec3,
    supports: Vec<usize>,
    supported_by: Vec<usize>,
}

impl Brick {
    fn min_z(&self) -> i32 {
        self.start.z.min(self.end.z)
    }
    fn max_z(&self) -> i32 {
        self.start.z.max(self.end.z)
    }
    fn overlaps(&self, other: &Self) -> bool {
        self.start.x.max(other.start.x) <= self.end.x.min(other.end.x)
            && self.start.y.max(other.start.y) <= self.end.y.min(other.end.y)
            && self.start.z.max(other.start.z) <= self.end.z.min(other.end.z)
    }
    fn move_down(&mut self) {
        self.start.z -= 1;
        self.end.z -= 1;
    }
    fn move_up(&mut self) {
        self.start.z += 1;
        self.end.z += 1;
    }
}

fn ivec3(input: &str) -> IResult<&str, IVec3> {
    let (input, a) = complete::i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, b) = complete::i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, c) = complete::i32(input)?;

    Ok((input, IVec3::from([a, b, c])))
}

fn parse_bricks(input: &str) -> IResult<&str, Vec<Brick>> {
    let (input, bricks) =
        separated_list1(line_ending, separated_pair(ivec3, tag("~"), ivec3))(input)?;

    Ok((
        input,
        bricks
            .iter()
            .map(|(a, b)| Brick {
                start: *a,
                end: *b,
                supports: vec![],
                supported_by: vec![],
            })
            .collect(),
    ))
}

fn settle_bricks(bricks: &mut Vec<Brick>) -> Vec<Brick> {
    bricks
        .iter_mut()
        .sorted_by_key(|brick| brick.min_z())
        .fold(
            (Vec::<Brick>::new(), 1),
            |(mut result, curr_max_z), brick| {
                // position brick just above curr_max_z
                let offset = IVec3::new(0, 0, curr_max_z - brick.min_z() + 1);
                brick.start += offset;
                brick.end += offset;

                loop {
                    let mut can_move_down = true;
                    brick.move_down();

                    for i in (0..result.len()).rev() {
                        if result[i].max_z() < brick.min_z() {
                            continue;
                        }

                        if brick.overlaps(&result[i]) {
                            can_move_down = false;
                            let len = result.len();
                            result[i].supports.push(len);
                            brick.supported_by.push(i);
                        }
                    }

                    // cannot move down anymore because of overlap
                    if !can_move_down {
                        brick.move_up();
                        break;
                    }

                    // reached bottom
                    if brick.min_z() == 1 {
                        break;
                    }
                }
                result.push(brick.clone());

                (result.clone(), curr_max_z.max(brick.max_z()))
            },
        )
        .0
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, mut bricks) = parse_bricks(input).expect("should parse bricks");
    let bricks = settle_bricks(&mut bricks);

    let mut sum = 0;

    for i in (0..bricks.len()).rev() {
        if bricks[i].supports.is_empty() {
            continue;
        }

        let mut fallen_bricks: Vec<usize> = vec![];
        let mut stack = vec![i];

        while let Some(idx) = stack.pop() {
            fallen_bricks.push(idx);

            let brick = &bricks[idx];

            for supported_brick_idx in brick.supports.iter() {
                let supported_brick = &bricks[*supported_brick_idx];

                if supported_brick
                    .supported_by
                    .iter()
                    .all(|idx| fallen_bricks.contains(idx))
                {
                    stack.push(*supported_brick_idx);
                }
            }
        }
        sum += (fallen_bricks.len() - 1) as u64;
    }

    Ok(sum)
}

#[cfg(test)]
mod day_22_part2 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!(7, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input1() -> miette::Result<()> {
        let input = include_str!("../inputs/input1.txt");
        assert_eq!(93292, process(input)?);
        Ok(())
    }
}
