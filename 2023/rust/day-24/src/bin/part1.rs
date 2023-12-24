use day_24::part1::process;

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
    let bounds = 200000000000000f64..=400000000000000f64;
    let output = process(input, bounds).context("process part 1")?;
    println!("Output is {output}");
    Ok(())
}
