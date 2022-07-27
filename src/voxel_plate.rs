use std::collections::HashMap;

use crate::Vec3;

#[derive(Default, Clone)]
pub struct VoxelPlate {
    internal: HashMap<isize, Vec<Vec3>>,
}

impl VoxelPlate {
    pub(crate) fn add_voxel(&mut self, voxel: Vec3) {
        let z = voxel.z;
        self.internal.entry(z).or_insert_with(Vec::new).push(voxel);
    }

    pub(crate) fn rows(self) -> Vec<(isize, Vec<Vec3>)> {
        let mut rows: Vec<(isize, Vec<Vec3>)> = self.internal.into_iter().collect();
        rows.sort_by(|(z1, ..), (z2, ..)| z1.cmp(z2));

        rows
    }
}
