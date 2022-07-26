use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::Vec3;

#[derive(Default)]
pub struct VoxelPlate {
    internal: HashMap<isize, Vec<Vec3>>,
}

impl VoxelPlate {
    pub fn width(&self) -> usize {
        let max_row = self
            .internal
            .iter()
            .max_by(|(.., row1), (.., row2)| row1.len().cmp(&row2.len()));
        let (.., row) = max_row.expect("Failed to find any rows in VoxelPlate");
        row.len()
    }

    pub fn add_voxel(&mut self, voxel: Vec3) {
        let z = voxel.z;
        self.internal.entry(z).or_insert_with(Vec::new).push(voxel);
    }

    pub fn rows(&self) -> Vec<(isize, &Vec<Vec3>)> {
        let mut keys: Vec<isize> = self.internal.keys().into_iter().map(|k| *k).collect();
        keys.sort_by(|z1, z2| z1.cmp(z2));

        keys.into_iter()
            .map(|key| (key, &self.internal[&key]))
            .collect()
    }
}
