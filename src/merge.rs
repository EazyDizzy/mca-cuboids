use crate::voxel_sequence::VoxelSequence;
use crate::voxel_stack::VoxelStack;
use crate::BlockCoordinates;
use anyhow::Result;
use rustc_hash::{FxHashMap, FxHasher};
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

pub fn merge_voxels(voxel_stack: VoxelStack) -> Result<Vec<VoxelSequence>> {
    let mut all_sequences_by_end_y = FxHashMap::default();

    for (y, plate) in voxel_stack.plates() {
        let mut plane_sequences = vec![];

        for (z, row) in plate.rows() {
            let row_sequences = merge_voxels_x_row(row);

            plane_sequences = stretch_sequences_by_z(row_sequences, plane_sequences, z);
        }

        stretch_sequences_by_y(&mut all_sequences_by_end_y, plane_sequences, y);
    }
    let mut all_sequences = vec![];
    all_sequences_by_end_y.into_iter().for_each(|(.., seq)| {
        all_sequences.extend(seq);
    });

    Ok(all_sequences)
}

fn stretch_sequences_by_y(
    all_sequences_by_end_y: &mut HashMap<isize, Vec<VoxelSequence>, BuildHasherDefault<FxHasher>>,
    mut current: Vec<VoxelSequence>,
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
    row_sequences: Vec<VoxelSequence>,
    mut plane_sequences: Vec<VoxelSequence>,
    z: isize,
) -> Vec<VoxelSequence> {
    let mut sequences_to_append = vec![];
    let mut prev_row_sequences: Vec<&mut VoxelSequence> = plane_sequences
        .iter_mut()
        .filter(|s: &&mut VoxelSequence| s.has_z_end_on(z - 1))
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

fn merge_voxels_x_row(mut row: Vec<BlockCoordinates>) -> Vec<VoxelSequence> {
    row.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

    let mut x_sequences = vec![];
    let mut start_voxel_index = 0;
    let mut prev_voxel_index = 0;

    for (index, voxel) in row.iter().enumerate().skip(1) {
        let prev_voxel = &row[prev_voxel_index];
        let stop_concatenation = voxel.x != prev_voxel.x + 1;

        if stop_concatenation {
            x_sequences.push(VoxelSequence::new(
                row[start_voxel_index].clone(),
                row[prev_voxel_index].clone(),
            ));

            start_voxel_index = index;
        }

        prev_voxel_index = index;
    }
    x_sequences.push(VoxelSequence::new(
        row[start_voxel_index].clone(),
        row[prev_voxel_index].clone(),
    ));

    x_sequences
}
