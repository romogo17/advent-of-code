use crate::custom_error::AocError;

fn hash(input: &str) -> u8 {
    input
        .trim()
        .chars()
        .fold(0u64, |acc, c| (acc + c as u64) * 17u64) as u8
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let results: u64 = input.trim().split(",").map(|step| hash(step) as u64).sum();
    Ok(results)
}

#[cfg(test)]
mod day_15_part1 {
    use super::*;

    #[test]
    fn example() -> miette::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(1320, process(input)?);
        Ok(())
    }

    #[test]
    fn input1() -> miette::Result<()> {
        let input = include_str!("../inputs/input1.txt");
        assert_eq!(506869, process(input)?);
        Ok(())
    }
}
