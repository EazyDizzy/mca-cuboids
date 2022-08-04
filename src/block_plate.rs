use rustc_hash::FxHashMap;

use crate::BlockCoordinates;

#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub struct BlockPlate {
    internal: FxHashMap<i32, Vec<BlockCoordinates>>,
}

impl BlockPlate {
    pub(crate) fn add_block(&mut self, block: BlockCoordinates) {
        let z = block.z;
        self.internal.entry(z).or_insert_with(Vec::new).push(block);
    }

    pub(crate) fn rows(self) -> Vec<(i32, Vec<BlockCoordinates>)> {
        let mut rows: Vec<(i32, Vec<BlockCoordinates>)> = self.internal.into_iter().collect();
        rows.sort_by(|(z1, ..), (z2, ..)| z1.cmp(z2));

        rows
    }
}
