#![feature(test)]
#[cfg(test)]
extern crate test;
use crate::merge::merge_voxels;
use crate::voxel_stack::VoxelStack;

mod merge;
pub mod read;
mod voxel_plate;
mod voxel_sequence;
mod voxel_stack;

#[derive(Debug)]
pub struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}

fn vec3(x: isize, y: isize, z: isize) -> Vec3 {
    Vec3 { x, y, z }
}

pub fn parse(lvl_path: &str) {
    let lvl = read::read_level(lvl_path);
    let stack = VoxelStack::from(lvl);
    let collisions = merge_voxels(&stack);
}
