use day_20::part1::process;

use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let input = include_str!("../../inputs/input.txt");
    let output = process(input, 100).context("process part 1")?;
    println!("Output is {output}");
    Ok(())
}