use crate::custom_error::AocError;
use tracing::{debug, info};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    info!("running not implemented {{crate_name}}_part2");
    debug!(input, "with");
    Ok(100)
}

#[cfg(test)]
mod {{crate_name}}_part2 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "";
        assert_eq!(100, process(input)?);
        Ok(())
    }

    // #[test_log::test]
    // fn input1() -> miette::Result<()> {
    //     let input = include_str!("../inputs/input1.txt");
    //     assert_eq!(100, process(input)?);
    //     Ok(())
    // }
}
