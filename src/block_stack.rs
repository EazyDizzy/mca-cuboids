use rustc_hash::FxHashMap;

use crate::block_plate::BlockPlate;
use crate::BlockCoordinates;

#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub struct BlockStack {
    internal: FxHashMap<isize, BlockPlate>,
}

impl BlockStack {
    pub(crate) fn add_all(&mut self, blocks: Vec<BlockCoordinates>) {
        for v in blocks {
            self.add_block(v);
        }
    }
    pub(crate) fn add_block(&mut self, block: BlockCoordinates) {
        let y = block.y as isize;
        self.internal
            .entry(y)
            .or_insert_with(BlockPlate::default)
            .add_block(block);
    }

    pub fn plates(self) -> Vec<(isize, BlockPlate)> {
        let mut plates: Vec<(isize, BlockPlate)> = self.internal.into_iter().collect();
        plates.sort_by(|(y1, ..), (y2, ..)| y1.cmp(y2));

        plates
    }
}

impl From<Vec<BlockCoordinates>> for BlockStack {
    fn from(blocks: Vec<BlockCoordinates>) -> Self {
        let mut stack = BlockStack::default();
        stack.add_all(blocks);

        stack
    }
}
