use crate::BlockCoordinates;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct VoxelSequence {
    pub start: BlockCoordinates,
    pub end: BlockCoordinates,
}

impl VoxelSequence {
    pub(crate) fn new(start: BlockCoordinates, end: BlockCoordinates) -> VoxelSequence {
        VoxelSequence { start, end }
    }

    pub fn expand_start(&mut self, other: BlockCoordinates) {
        self.start = other;
    }
    pub fn expand_end(&mut self, other: Self) {
        self.end = other.end;
    }

    pub fn expand_z_end(&mut self, other: Self) {
        self.end = other.end;
    }

    pub fn start_position(&self) -> &BlockCoordinates {
        &self.start
    }

    pub fn end_position(&self) -> &BlockCoordinates {
        &self.end
    }

    pub fn same_x_size(&self, other: &Self) -> bool {
        other.start.x == self.start.x && other.end.x == self.end.x
    }
    pub fn same_z_size(&self, other: &Self) -> bool {
        other.start.z == self.start.z && other.end.z == self.end.z
    }

    pub fn has_z_end_on(&self, z: isize) -> bool {
        self.end.z == z
    }

    pub fn has_y_end_on(&self, y: isize) -> bool {
        self.end.y == y
    }
}
