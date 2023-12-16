use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<u64, AocError> {
    todo!("{{project-name}}_part1");
}

#[cfg(test)]
mod {{crate_name}}_part2 {
    use super::*;

    #[test]
    fn example() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!(100, process(input)?);
        Ok(())
    }
}
