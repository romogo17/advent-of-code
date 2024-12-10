use itertools::Itertools;
use tracing::debug;

fn print_memory(memory: &[Option<u64>]) {
    let memory = memory
        .iter()
        .map(|mem_block| {
            if let Some(file_id) = mem_block {
                format!("{}", file_id)
            } else {
                format!(".")
            }
        })
        .join("");
    debug!("{}", memory);
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64> {
    // expand the memory to the full layout
    let mut expanded_memory = input
        .trim()
        .chars()
        .enumerate()
        .map(|(idx, num_blocks)| {
            std::iter::repeat(if idx % 2 == 0 {
                Some((idx / 2) as u64) // this is the file ID
            } else {
                None
            })
            .take(num_blocks.to_digit(10).unwrap() as usize)
        })
        .flatten()
        .collect::<Vec<_>>();

    print_memory(&expanded_memory);

    // move the right-most memory blocks to the left-most free space available
    let file_blocks = expanded_memory
        .iter()
        .filter(|file_id| file_id.is_some())
        .count();
    let mut idx = 0;
    while idx < file_blocks {
        match expanded_memory[idx] {
            Some(_file_id) => {}
            None => {
                let last_file_idx = expanded_memory.iter().rposition(|v| v.is_some()).unwrap();
                expanded_memory.swap(idx, last_file_idx);
            }
        }
        idx += 1;
    }
    print_memory(&expanded_memory);

    // compute the file system checksum
    let result = expanded_memory
        .iter()
        .enumerate()
        .fold(0u64, |acc, (idx, file_id)| {
            if let Some(file_id) = file_id {
                acc + idx as u64 * file_id
            } else {
                acc
            }
        });

    Ok(result)
}

#[cfg(test)]
mod day_09_part1 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!(1928, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(6330095022244, process(input)?);
        Ok(())
    }
}
