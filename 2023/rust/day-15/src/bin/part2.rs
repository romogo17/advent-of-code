use day_15::part2::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    let input = include_str!("../../inputs/input1.txt");
    let output = process(input).context("process part 2")?;
    println!("Output is {output}");
    Ok(())
}