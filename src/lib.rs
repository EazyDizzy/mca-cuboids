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

pub struct ExportParams {
    start: Vec3,
    end: Vec3,
    skip_blocks: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn combine_2x2x1() {
//         let result = detect_collisions(
//             "./assets/test_lvl",
//             ExportParams {
//                 start: vec3(1, 1, 1),
//                 end: vec3(2, 1, 2),
//                 skip_blocks: None,
//             },
//         );
//
//         dbg!(result);
//     }
// }
