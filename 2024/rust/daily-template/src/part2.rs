use miette::miette;
use nom::IResult;
use tracing::debug;

fn parse(input: &str) -> IResult<&str, ()> {
    debug!(?input);

    todo!("not implemented {{crate_name}}_part2 parsing");
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u32> {
    let (_input, data) = parse(input).map_err(|e| miette!("parse failed {}", e))?;
    debug!(?data);
    todo!("not implemented {{crate_name}}_part2");
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
