#![deny(clippy::all, clippy::pedantic, clippy::cognitive_complexity)]
#![allow(clippy::cast_possible_wrap)]
#![feature(test)]
#[cfg(test)]
extern crate test;
use crate::block_sequence::BlockSequence;
use crate::block_stack::BlockStack;
use crate::merge::merge_blocks;
use anyhow::Result;
use serde::{Deserialize, Serialize};

mod block_plate;
pub mod block_sequence;
mod block_stack;
mod merge;
mod read;

#[derive(Clone, Default)]
pub struct ExportParams {
    pub start: BlockCoordinates,
    pub end: BlockCoordinates,
    pub skip_blocks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Default)]
pub struct BlockCoordinates {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl BlockCoordinates {
    #[must_use]
    pub fn new(x: isize, y: isize, z: isize) -> BlockCoordinates {
        BlockCoordinates { x, y, z }
    }
}
/// # Errors
///
/// Will return `Err` if `lvl_path` does not exist or the user does not have
/// permission to read it.
pub fn detect_collisions(lvl_path: &str, params: ExportParams) -> Result<Vec<BlockSequence>> {
    let stack = read::read_level(lvl_path, params)?;

    Ok(merge_blocks(stack))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_collisions_2x2x1() {
        let result = detect_collisions(
            "./assets/test_lvl",
            ExportParams {
                start: BlockCoordinates::new(1, -63, 1),
                end: BlockCoordinates::new(2, -63, 2),
                ..Default::default()
            },
        );

        assert_eq!(
            result.unwrap(),
            vec![BlockSequence::new(
                BlockCoordinates::new(1, -63, 1),
                BlockCoordinates::new(2, -63, 2)
            )]
        );
    }
    #[test]
    fn detect_collisions_1x3x1() {
        let result = detect_collisions(
            "./assets/test_lvl",
            ExportParams {
                start: BlockCoordinates::new(1, -63, 5),
                end: BlockCoordinates::new(2, -60, 6),
                ..Default::default()
            },
        );

        assert_eq!(
            result.unwrap(),
            vec![BlockSequence::new(
                BlockCoordinates::new(1, -63, 5),
                BlockCoordinates::new(1, -61, 5)
            )]
        );
    }
    #[test]
    fn detect_collisions_2x2x2() {
        let result = detect_collisions(
            "./assets/test_lvl",
            ExportParams {
                start: BlockCoordinates::new(1, -63, 8),
                end: BlockCoordinates::new(4, -60, 10),
                ..Default::default()
            },
        );

        assert_eq!(
            result.unwrap(),
            vec![BlockSequence::new(
                BlockCoordinates::new(1, -63, 8),
                BlockCoordinates::new(2, -62, 9)
            )]
        );
    }
    #[test]
    fn detect_collisions_tetris() {
        let result = detect_collisions(
            "./assets/test_lvl",
            ExportParams {
                start: BlockCoordinates::new(1, -63, 11),
                end: BlockCoordinates::new(4, -60, 12),
                ..Default::default()
            },
        );

        assert_eq!(
            result.unwrap(),
            vec![
                BlockSequence::new(
                    BlockCoordinates::new(1, -63, 11),
                    BlockCoordinates::new(2, -63, 11)
                ),
                BlockSequence::new(
                    BlockCoordinates::new(1, -63, 12),
                    BlockCoordinates::new(1, -63, 12)
                )
            ]
        );
    }
}
