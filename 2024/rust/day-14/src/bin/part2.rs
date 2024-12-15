use day_14::part2::process;

use glam::IVec2;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let input = include_str!("../../inputs/input.txt");
    let output = process(input, IVec2::new(101, 103)).context("process part 2")?;
    println!("Output is {output}");
    Ok(())
}
