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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}

fn vec3(x: isize, y: isize, z: isize) -> Vec3 {
    Vec3 { x, y, z }
}

pub fn detect_collisions(lvl_path: &str) -> Vec<VoxelSequence> {
    let lvl = read::read_level(lvl_path);
    let stack = VoxelStack::from(lvl);
    merge_voxels(stack)
}
