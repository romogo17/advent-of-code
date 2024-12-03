use day_01::part2::process;

use miette::Context;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().compact())
        .init();

    let input = include_str!("../../inputs/input.txt");
    let output = process(input).context("process part 2")?;
    println!("Output is {output}");
    Ok(())
}
