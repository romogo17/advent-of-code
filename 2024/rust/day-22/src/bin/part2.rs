use day_22::part2::process;

use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let input = include_str!("../../inputs/input.txt");
    let output = process(input).context("process part 2")?;
    println!("Output is {output}");
    Ok(())
}
