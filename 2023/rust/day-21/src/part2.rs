use crate::custom_error::AocError;
use crate::nom_locate_utils::*;

use glam::{IVec2, Mat3, Vec3};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::all_consuming,
    multi::many1, sequence::terminated, IResult, Parser,
};
use std::collections::{HashMap, HashSet};
use tracing::debug;

#[derive(Debug, Eq, PartialEq)]
enum MapTile {
    Start,
    GardenPlot,
    Rock,
}

#[derive(Debug, Eq, PartialEq)]
struct MapTileInfo<'a> {
    span: SpanIVec2<'a>,
    tile_type: MapTile,
}

fn parse_grid(input: Span) -> IResult<Span, HashMap<IVec2, MapTile>> {
    let (input, output) = all_consuming(many1(terminated(
        alt((
            tag(".").map(with_xy).map(|span| MapTileInfo {
                span,
                tile_type: MapTile::GardenPlot,
            }),
            tag("S").map(with_xy).map(|span| MapTileInfo {
                span,
                tile_type: MapTile::Start,
            }),
            tag("#").map(with_xy).map(|span| MapTileInfo {
                span,
                tile_type: MapTile::Rock,
            }),
        )),
        multispace0,
    )))(input)?;

    Ok((
        input,
        output
            .into_iter()
            .filter_map(|map_tile_info| Some((map_tile_info.span.extra, map_tile_info.tile_type)))
            .collect(),
    ))
}

#[tracing::instrument(skip(input, total_steps))]
pub fn process(input: &str, total_steps: u64) -> miette::Result<u64, AocError> {
    let (_, grid) = parse_grid(Span::new(input)).expect("a valid grid parse");

    let diameter = input.lines().count() as i64;
    let radius: i64 = diameter / 2;
    let remainder: i64 = total_steps as i64 % diameter;
    let n: i64 = (total_steps as i64 - remainder) / diameter;

    let bounds = IVec2::new(diameter as i32, diameter as i32);

    let x_values = vec![radius, radius + 1 * diameter, radius + 2 * diameter];

    debug!(?radius, ?diameter, ?remainder, ?n, "grid properties");
    debug!(?x_values);

    let start = grid
        .iter()
        .find_map(|(pos, tile_type)| match tile_type {
            MapTile::Start => Some(pos),
            _ => None,
        })
        .expect("grid should have a start position");

    let mut steps_at: HashMap<u64, HashSet<IVec2>> =
        HashMap::from([(0u64, HashSet::from([*start]))]);
    for steps in 1..=*x_values.last().unwrap() as u64 {
        let mut pos_to_check = steps_at
            .get(&(steps - 1))
            .unwrap()
            .iter()
            .flat_map(|pos| {
                vec![
                    *pos + IVec2::X,
                    *pos + IVec2::NEG_X,
                    *pos + IVec2::Y,
                    *pos + IVec2::NEG_Y,
                ]
            })
            .collect::<Vec<IVec2>>();

        while let Some(pos) = pos_to_check.pop() {
            match grid.get(&(pos.rem_euclid(bounds))) {
                Some(MapTile::Rock) | None => {}
                Some(MapTile::GardenPlot) | Some(MapTile::Start) => {
                    // debug!(?pos, ?steps);
                    steps_at
                        .entry(steps)
                        .and_modify(|pos_set| {
                            pos_set.insert(pos);
                        })
                        .or_insert(HashSet::from([pos]));
                }
            }
        }
    }

    let y_values = x_values
        .iter()
        .map(|s| {
            let s = *s as u64;
            steps_at.get(&s).unwrap().len() as i64
        })
        .collect::<Vec<i64>>();
    debug!(?y_values);

    // Vandermonde matrix is built from the sequence number input
    // so x=0, y=1, z=2
    // x.pow(0), x.pow(1), x.pow(2)
    // y.pow(0), y.pow(1), y.pow(2)
    // z.pow(0), z.pow(1), z.pow(2)
    //
    // 1, 0, 0
    // 1, 1, 1
    // 1, 2, 4
    let x: f32 = 0.;
    let y: f32 = 1.;
    let z: f32 = 2.;
    let vandermonde = Mat3::from_cols(
        Vec3::new(x.powf(0.), y.powf(0.), z.powf(0.)),
        Vec3::new(x.powf(1.), y.powf(1.), z.powf(1.)),
        Vec3::new(x.powf(2.), y.powf(2.), z.powf(2.)),
    );
    // multiplying vandermonde by the sequence numbers
    // yields our a, b, c values
    let [c, b, a] = (vandermonde.inverse()
        * Vec3::new(y_values[0] as f32, y_values[1] as f32, y_values[2] as f32))
    .to_array();
    let a = a as i64;
    let b = b as i64;
    let c = c as i64;

    debug!(a, b, c, "coefficients");
    debug!(n, "solving for");

    Ok((a * n.pow(2) + b * n + c) as u64)
}
