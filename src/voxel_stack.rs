use rustc_hash::FxHashMap;

use crate::voxel_plate::VoxelPlate;
use crate::BlockCoordinates;

#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub struct VoxelStack {
    internal: FxHashMap<isize, VoxelPlate>,
}

impl VoxelStack {
    pub(crate) fn add_all(&mut self, voxels: Vec<BlockCoordinates>) {
        for v in voxels {
            self.add_voxel(v);
        }
    }
    pub(crate) fn add_voxel(&mut self, voxel: BlockCoordinates) {
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
        stack.add_all(voxels);

        stack
    }
}
