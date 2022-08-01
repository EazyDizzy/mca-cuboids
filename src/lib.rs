#![feature(test)]
#[cfg(test)]
extern crate test;
use crate::merge::merge_voxels;
use crate::voxel_sequence::VoxelSequence;
use crate::voxel_stack::VoxelStack;
use serde::{Deserialize, Serialize};

mod merge;
mod read;
mod voxel_plate;
pub mod voxel_sequence;
mod voxel_stack;

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
    pub fn new(x: isize, y: isize, z: isize) -> BlockCoordinates {
        BlockCoordinates { x, y, z }
    }
}

pub fn detect_collisions(lvl_path: &str, params: ExportParams) -> Vec<VoxelSequence> {
    let stack = read::read_level(lvl_path, params);
    merge_voxels(stack)
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
            result,
            vec![VoxelSequence::new(
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
            result,
            vec![VoxelSequence::new(
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
            result,
            vec![VoxelSequence::new(
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
            result,
            vec![
                VoxelSequence::new(
                    BlockCoordinates::new(1, -63, 11),
                    BlockCoordinates::new(2, -63, 11)
                ),
                VoxelSequence::new(
                    BlockCoordinates::new(1, -63, 12),
                    BlockCoordinates::new(1, -63, 12)
                )
            ]
        );
    }
}
