use std::fmt::Debug;

use advent_of_code::get_challenge_input_as_str;

#[derive(Clone, Copy, PartialEq, Eq)]
enum BlockType {
    File(i64),
    FreeSpace(i64),
}

impl Debug for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(value) => write!(f, "{}", value),
            Self::FreeSpace(value) => write!(f, "{}", ".".repeat(*value as usize)),
        }
    }
}

impl BlockType {
    pub fn checksum_value(&self, index: usize) -> i64 {
        match self {
            BlockType::File(value) => index as i64 * value,
            BlockType::FreeSpace(_) => 0,
        }
    }
}

fn parse_data(data: &str) -> (i64, Vec<BlockType>) {
    let mut blocks: Vec<BlockType> = Vec::new();
    let mut current_file_id: i64 = 0;

    for (index, block) in data.split("").enumerate() {
        if block == "" {
            continue;
        }
        let block_count: i64 = block.parse().unwrap_or(0);

        match index {
            i if i % 2 == 1 => {
                for _ in 0..block_count {
                    blocks.push(BlockType::File(current_file_id));
                }

                current_file_id += 1;
            }
            i if i % 2 == 0 => {
                for _ in 0..block_count {
                    blocks.push(BlockType::FreeSpace(1));
                }
            }
            _ => {}
        }
    }

    (current_file_id, blocks)
}

#[allow(unused)]
fn part_one(data: &str) -> i64 {
    let (_, mut blocks) = parse_data(data);

    let mut compacted_blocks = Vec::with_capacity(blocks.len());
    compacted_blocks.clone_from(&blocks);

    for (index, _) in blocks.iter().enumerate() {
        if matches!(compacted_blocks[index], BlockType::FreeSpace(_)) {
            if let Some(block_index) = compacted_blocks
                .iter()
                .rev()
                .position(|&block| !matches!(block, BlockType::FreeSpace(_)))
            {
                if index == (blocks.len() - block_index) {
                    break;
                }

                compacted_blocks.swap(blocks.len() - block_index - 1, index);
            }
        }
    }

    compacted_blocks
        .iter()
        .enumerate()
        .map(|(index, block)| block.checksum_value(index))
        .sum()
}

fn find_range_for_block(vec: &Vec<BlockType>, value: &BlockType) -> Option<std::ops::Range<usize>> {
    let mut start = None;

    for (index, block) in vec.iter().enumerate() {
        if block == value {
            if start.is_none() {
                start = Some(index);
            }
        } else if start.is_some() {
            return start.map(|s| s..index);
        }
    }

    start.map(|s| s..vec.len())
}

#[allow(unused)]
fn part_two(data: &str) -> i64 {
    let (mut current_file_id, mut blocks) = parse_data(data);

    while current_file_id >= 0 {
        if let Some(file_range) = find_range_for_block(&blocks, &BlockType::File(current_file_id)) {
            for (start_index, block) in blocks.clone().iter().enumerate() {
                if matches!(block, BlockType::FreeSpace(_)) {
                    let mut end_index = start_index;

                    for (index, block) in blocks.iter().enumerate().skip(start_index) {
                        if matches!(block, BlockType::File(_)) {
                            break;
                        }

                        end_index = index;
                    }

                    let free_space_range: std::ops::Range<usize> = start_index..end_index + 1;

                    if free_space_range.start > file_range.start {
                        break;
                    }

                    if free_space_range.len() == file_range.len() {
                        for i in 0..free_space_range.len() {
                            blocks.swap(free_space_range.start + i, file_range.start + i);
                        }
                        break;
                    } else if free_space_range.len() > file_range.len() {
                        for i in 0..file_range.len() {
                            blocks.swap(free_space_range.start + i, file_range.start + i);
                        }
                        break;
                    }
                }
            }
        }
        current_file_id -= 1;
    }

    blocks
        .iter()
        .enumerate()
        .map(|(index, block)| block.checksum_value(index))
        .sum()
}

pub fn main() {
    if let Ok(data) = get_challenge_input_as_str(2024, 9) {
        let result = part_one(&data);
        println!("part one: {}", result);
        let result = part_two(&data);
        println!("part two: {}", result);
    }
}

mod tests {
    #![allow(unused)]
    use super::*;

    const EXAMPLE_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part_one() {
        let result = part_one(EXAMPLE_INPUT);

        assert_eq!(result, 1928)
    }

    #[test]
    fn test_part_two() {
        let result = part_two(EXAMPLE_INPUT);

        assert_eq!(result, 2858)
    }
}
