use std::collections::HashMap;

use crate::BlockCoordinates;

#[derive(Default, Clone)]
pub struct VoxelPlate {
    internal: HashMap<isize, Vec<BlockCoordinates>>,
}

impl VoxelPlate {
    pub(crate) fn add_voxel(&mut self, voxel: BlockCoordinates) {
        let z = voxel.z;
        self.internal.entry(z).or_insert_with(Vec::new).push(voxel);
    }

    pub(crate) fn rows(self) -> Vec<(isize, Vec<BlockCoordinates>)> {
        let mut rows: Vec<(isize, Vec<BlockCoordinates>)> = self.internal.into_iter().collect();
        rows.sort_by(|(z1, ..), (z2, ..)| z1.cmp(z2));

        rows
    }
}
