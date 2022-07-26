use crate::voxel_sequence::VoxelSequence;
use crate::voxel_stack::VoxelStack;
use crate::Vec3;

pub fn merge_voxels(voxel_stack: &VoxelStack) -> Vec<VoxelSequence> {
    let mut all_sequences = vec![];

    for (y, plate) in voxel_stack.plates() {
        let mut plane_sequences = vec![];

        for (z, row) in plate.rows() {
            let row: Vec<&Vec3> = row.iter().collect();
            let row_sequences = merge_voxels_x_row(row);

            plane_sequences = stretch_sequences_by_z(row_sequences, plane_sequences, z);
        }

        all_sequences = stretch_sequences_by_y(plane_sequences, all_sequences, y);
    }

    all_sequences
}

fn stretch_sequences_by_y<'a>(
    mut plane_sequences: Vec<VoxelSequence<'a>>,
    mut all_sequences: Vec<VoxelSequence<'a>>,
    y: isize,
) -> Vec<VoxelSequence<'a>> {
    let needed_y = y - 1;
    let previous_layer_sequences = all_sequences
        .iter_mut()
        .filter(|s| s.has_y_end_on(needed_y))
        .collect::<Vec<&mut VoxelSequence<'a>>>();

    for seq in previous_layer_sequences {
        let same_new_seq = plane_sequences
            .iter()
            .enumerate()
            .find(|(_, s)| s.same_x_size(seq) && s.same_z_size(seq));

        if let Some((i, ..)) = same_new_seq {
            let d = plane_sequences.remove(i);
            seq.expand_y_end(d);
        }
    }

    all_sequences.extend(plane_sequences);

    all_sequences
}

fn stretch_sequences_by_z<'a>(
    row_sequences: Vec<VoxelSequence<'a>>,
    mut plane_sequences: Vec<VoxelSequence<'a>>,
    z: isize,
) -> Vec<VoxelSequence<'a>> {
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

fn merge_voxels_x_row(mut row: Vec<&Vec3>) -> Vec<VoxelSequence> {
    row.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

    let mut x_sequences = vec![];
    let mut start_voxel_index = 0;
    let mut prev_voxel_index = 0;

    for (index, voxel) in row.iter().enumerate().skip(1) {
        let prev_voxel = row[prev_voxel_index];
        let stop_concatenation = voxel.x != prev_voxel.x + 1;

        if stop_concatenation {
            x_sequences.push(VoxelSequence::new(
                row[start_voxel_index..=prev_voxel_index].to_vec(),
            ));

            start_voxel_index = index;
        }

        prev_voxel_index = index;
    }
    x_sequences.push(VoxelSequence::new(
        row[start_voxel_index..=prev_voxel_index].to_vec(),
    ));

    x_sequences
}
