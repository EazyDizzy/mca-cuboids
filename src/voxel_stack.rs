use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::Vec3;
use crate::voxel_plate::VoxelPlate;

#[derive(Default)]
pub struct VoxelStack {
    internal: HashMap<isize, VoxelPlate>,
}

impl VoxelStack {

    pub fn width(&self) -> usize {
        let biggest_plate = self
            .internal
            .iter()
            .max_by(|(.., plate1), (.., plate2)| plate1.width().cmp(&plate2.width()));
        let (.., plate) = biggest_plate.expect("Failed to find any rows in VoxelPlate");
        plate.width()
    }

    fn add_voxel(&mut self, voxel: Vec3) {
        let y = voxel.y as isize;
        self.internal
            .entry(y)
            .or_insert_with(VoxelPlate::default)
            .add_voxel(voxel);
    }

    pub fn plates(&self) -> Vec<(isize, &VoxelPlate)> {
        let mut keys: Vec<isize> = self.internal.keys().into_iter().map(|k| *k).collect();
        keys.sort_by(|y1, y2| y1.cmp(y2));

        keys.into_iter()
            .map(|key| (key, &self.internal[&key]))
            .collect()
    }
}

impl From<Vec<Vec3>> for VoxelStack {
    fn from(voxels: Vec<Vec3>) -> Self {
        let mut stack = VoxelStack::default();

        for voxel in voxels {
            stack.add_voxel(voxel);
        }

        stack
    }
}
