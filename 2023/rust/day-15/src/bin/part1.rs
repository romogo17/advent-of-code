use day_15::part1::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    let input = include_str!("../../inputs/input1.txt");
    let output = process(input).context("process part 1")?;
    println!("Output is {output}");
    Ok(())
}