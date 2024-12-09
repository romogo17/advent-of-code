use glam::IVec2;
use itertools::Itertools;
use miette::miette;
use nom::{
    bytes::complete::take_till, character::complete::satisfy, multi::many1, sequence::preceded,
    AsChar, IResult,
};
use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str>;

fn token(input: Span) -> IResult<Span, (IVec2, char)> {
    let x = input.get_column();
    let y = input.location_line();
    let (input, token) = satisfy(|c| c.is_alphanum())(input)?;
    Ok((input, (IVec2::new(x as i32 - 1, y as i32 - 1), token)))
}

fn parse(input: Span) -> IResult<Span, Vec<(IVec2, char)>> {
    many1(preceded(take_till(|c: char| c.is_alphanum()), token))(input)
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u32> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let y_bound = 0i32..height as i32;
    let x_bound = 0i32..width as i32;

    let (_input, mut data) = parse(Span::new(input)).map_err(|e| miette!("parse failed {}", e))?;
    data.sort_by(|(_, a), (_, b)| a.cmp(&b));

    let results = data
        .chunk_by(|(_, a), (_, b)| a == b)
        .flat_map(|chunk| {
            chunk.iter().combinations(2).flat_map(|antenas| {
                let diff = antenas[0].0 - antenas[1].0;
                [antenas[0].0 + diff, antenas[1].0 - diff]
            })
        })
        .filter(|pos| x_bound.contains(&pos.x) && y_bound.contains(&pos.y))
        .unique()
        .count();

    Ok(results as u32)
}

#[cfg(test)]
mod day_08_part1 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(14, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(289, process(input)?);
        Ok(())
    }
}
