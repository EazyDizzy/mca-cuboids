use crate::block_sequence::BlockSequence;
use crate::block_stack::BlockStack;
use crate::BlockCoordinates;
use rustc_hash::{FxHashMap, FxHasher};
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

pub(crate) fn merge_blocks(block_stack: BlockStack) -> Vec<BlockSequence> {
    let mut all_sequences_by_end_y = FxHashMap::default();

    for (y, plate) in block_stack.plates() {
        let mut plane_sequences = vec![];

        for (z, row) in plate.rows() {
            let row_sequences = merge_blocks_x_row(row);

            plane_sequences = stretch_sequences_by_z(row_sequences, plane_sequences, z);
        }

        stretch_sequences_by_y(&mut all_sequences_by_end_y, plane_sequences, y);
    }
    let mut all_sequences = vec![];
    for (.., seq) in all_sequences_by_end_y {
        all_sequences.extend(seq);
    }

    all_sequences
}

fn stretch_sequences_by_y(
    all_sequences_by_end_y: &mut HashMap<isize, Vec<BlockSequence>, BuildHasherDefault<FxHasher>>,
    mut current: Vec<BlockSequence>,
    y: isize,
) {
    let prev = all_sequences_by_end_y.get_mut(&(y - 1));

    if let Some(prev_sequences) = prev {
        let mut to_remove = vec![];
        prev_sequences.iter().enumerate().for_each(|(i, seq)| {
            let same_new_seq = current
                .iter_mut()
                .find(|s| s.same_x_size(seq) && s.same_z_size(seq));

            if let Some(current_seq) = same_new_seq {
                to_remove.push(i);
                current_seq.expand_start(seq.start.clone());
            }
        });

        to_remove.into_iter().rev().for_each(|i| {
            prev_sequences.remove(i);
        });
    }

    all_sequences_by_end_y.insert(y, current);
}

#[inline(never)]
fn stretch_sequences_by_z(
    row_sequences: Vec<BlockSequence>,
    mut plane_sequences: Vec<BlockSequence>,
    z: i32,
) -> Vec<BlockSequence> {
    let mut sequences_to_append = vec![];
    let mut prev_row_sequences: Vec<&mut BlockSequence> = plane_sequences
        .iter_mut()
        .filter(|s: &&mut BlockSequence| s.has_z_end_on(z - 1))
        .collect();

    for sequence in row_sequences {
        let same_sequence = prev_row_sequences
            .iter_mut()
            .find(|s| s.same_x_size(&sequence));

        if let Some(same) = same_sequence {
            same.expand_z_end(sequence);
        } else {
            sequences_to_append.push(sequence);
        }
    }

    plane_sequences.append(&mut sequences_to_append);

    plane_sequences
}

fn merge_blocks_x_row(mut row: Vec<BlockCoordinates>) -> Vec<BlockSequence> {
    row.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

    let mut x_sequences = vec![];
    let mut start_block_index = 0;
    let mut prev_block_index = 0;

    for (index, block) in row.iter().enumerate().skip(1) {
        let prev_block = &row[prev_block_index];
        let stop_concatenation = block.x != prev_block.x + 1;

        if stop_concatenation {
            x_sequences.push(BlockSequence::new(
                row[start_block_index].clone(),
                row[prev_block_index].clone(),
            ));

            start_block_index = index;
        }

        prev_block_index = index;
    }
    x_sequences.push(BlockSequence::new(
        row[start_block_index].clone(),
        row[prev_block_index].clone(),
    ));

    x_sequences
}
