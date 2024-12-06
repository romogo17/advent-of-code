use miette::miette;
use tracing::{debug};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u32> {
    debug!(?input);
    todo!("not implemented {{crate_name}}_part1");
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
    // fn input() -> miette::Result<()> {
    //     let input = include_str!("../inputs/input.txt");
    //     assert_eq!(100, process(input)?);
    //     Ok(())
    // }
}
