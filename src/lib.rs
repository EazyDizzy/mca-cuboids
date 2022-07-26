use crate::merge::merge_voxels;
use crate::voxel_stack::VoxelStack;

mod convert;
mod voxel_plate;
mod voxel_stack;
mod merge;
mod voxel_sequence;

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
    let lvl = convert::read_level(lvl_path);
    let stack = VoxelStack::from(lvl);
    let collisions = merge_voxels(&stack);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        parse("./assets/simple_lvl")
    }
}
