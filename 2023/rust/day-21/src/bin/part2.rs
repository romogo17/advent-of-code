use day_21::part2::process;

use miette::Context;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().compact())
        .init();

    let input = include_str!("../../inputs/input1.txt");
    let output = process(input, 26501365).context("process part 2")?;
    println!("Output is {output}");
    Ok(())
}
