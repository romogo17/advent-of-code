use day_14::part1::process;

use glam::IVec2;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let input = include_str!("../../inputs/input.txt");
    let output = process(input, IVec2::new(101, 103)).context("process part 1")?;
    println!("Output is {output}");
    Ok(())
}
