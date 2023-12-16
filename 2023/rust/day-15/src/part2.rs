use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i64, AocError> {
    todo!("day-15_part1");
}

#[cfg(test)]
mod day_15_part2 {
    use super::*;

    #[test]
    fn example() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!(100, process(input)?);
        Ok(())
    }
}
