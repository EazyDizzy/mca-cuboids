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
mod voxel_sequence;
mod voxel_stack;

#[derive(Clone, Default)]
pub struct ExportParams {
    start: Vec3,
    end: Vec3,
    skip_blocks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Default)]
pub struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}

pub fn vec3(x: isize, y: isize, z: isize) -> Vec3 {
    Vec3 { x, y, z }
}

pub fn detect_collisions(lvl_path: &str, params: ExportParams) -> Vec<VoxelSequence> {
    let lvl = read::read_level(lvl_path, params);
    let stack = VoxelStack::from(lvl);
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
                start: vec3(1, -63, 1),
                end: vec3(2, -63, 2),
                ..Default::default()
            },
        );

        assert_eq!(
            result,
            vec![VoxelSequence::new(vec3(1, -63, 1), vec3(2, -63, 2))]
        );
    }
    #[test]
    fn detect_collisions_1x3x1() {
        let result = detect_collisions(
            "./assets/test_lvl",
            ExportParams {
                start: vec3(1, -63, 5),
                end: vec3(2, -60, 6),
                ..Default::default()
            },
        );

        assert_eq!(
            result,
            vec![VoxelSequence::new(vec3(1, -63, 5), vec3(1, -61, 5))]
        );
    }
    #[test]
    fn detect_collisions_2x2x2() {
        let result = detect_collisions(
            "./assets/test_lvl",
            ExportParams {
                start: vec3(1, -63, 8),
                end: vec3(4, -60, 10),
                ..Default::default()
            },
        );

        assert_eq!(
            result,
            vec![VoxelSequence::new(vec3(1, -63, 8), vec3(2, -62, 9))]
        );
    }
    #[test]
    fn detect_collisions_tetris() {
        let result = detect_collisions(
            "./assets/test_lvl",
            ExportParams {
                start: vec3(1, -63, 11),
                end: vec3(4, -60, 12),
                ..Default::default()
            },
        );

        assert_eq!(
            result,
            vec![
                VoxelSequence::new(vec3(1, -63, 11), vec3(2, -63, 11)),
                VoxelSequence::new(vec3(1, -63, 12), vec3(1, -63, 12))
            ]
        );
    }
    #[test]
    fn detect_collisions_chess() {
        let result = detect_collisions(
            "./assets/test_lvl",
            ExportParams {
                start: vec3(1, -63, 13),
                end: vec3(4, -60, 15),
                ..Default::default()
            },
        );

        assert_eq!(
            result,
            vec![
                VoxelSequence::new(vec3(1, -63, 14), vec3(1, -63, 14)),
                VoxelSequence::new(vec3(3, -63, 14), vec3(3, -63, 14)),
                VoxelSequence::new(vec3(2, -63, 15), vec3(2, -63, 15)),
            ]
        );
    }
}
