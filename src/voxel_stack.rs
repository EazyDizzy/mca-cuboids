use std::collections::HashMap;

use crate::BlockCoordinates;
use crate::voxel_plate::VoxelPlate;

#[derive(Default, Clone)]
pub struct VoxelStack {
    internal: HashMap<isize, VoxelPlate>,
}

impl VoxelStack {

    fn add_voxel(&mut self, voxel: BlockCoordinates) {
        let y = voxel.y as isize;
        self.internal
            .entry(y)
            .or_insert_with(VoxelPlate::default)
            .add_voxel(voxel);
    }

    pub fn plates(self) -> Vec<(isize, VoxelPlate)> {
        let mut plates: Vec<(isize, VoxelPlate)> = self.internal.into_iter().collect();
        plates.sort_by(|(y1, ..), (y2, ..)| y1.cmp(y2));

        plates
    }
}

impl From<Vec<BlockCoordinates>> for VoxelStack {
    fn from(voxels: Vec<BlockCoordinates>) -> Self {
        let mut stack = VoxelStack::default();

        for voxel in voxels {
            stack.add_voxel(voxel);
        }

        stack
    }
}
