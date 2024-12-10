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

    // move the right-most complete files to the left-most free space available
    let mut mem_end_idx = expanded_memory.len();

    loop {
        // get last file position and size
        let Some(last_file_end_idx) = expanded_memory[0..mem_end_idx]
            .iter()
            .rposition(|file_id| file_id.is_some())
        else {
            panic!("memory should have at least one file_id");
        };

        let Some(last_file_start_idx) = expanded_memory[0..last_file_end_idx]
            .iter()
            .rposition(|file_id| file_id != &expanded_memory[last_file_end_idx])
            .map(|idx| idx + 1)
        else {
            break;
        };

        let last_file_len = (last_file_start_idx..=last_file_end_idx).count();

        // find an empty space that is at least as big as the file
        let Some(empty_chunk) = expanded_memory
            .windows(last_file_len)
            .position(|slice| slice.iter().all(|file_id| file_id.is_none()))
        else {
            mem_end_idx = last_file_start_idx;
            continue;
        };

        if empty_chunk < last_file_start_idx {
            // split mutable access to left/right of the last file
            let (left, right) = expanded_memory.split_at_mut(last_file_start_idx);

            // copy chunk from right into left at empty location
            left[empty_chunk..(empty_chunk + last_file_len)]
                .copy_from_slice(&right[..last_file_len]);

            // clear the right side
            for i in 0..last_file_len {
                right[i] = None;
            }
        }

        mem_end_idx = last_file_start_idx;
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
mod day_09_part2 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!(2858, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(6359491814941, process(input)?);
        Ok(())
    }
}
