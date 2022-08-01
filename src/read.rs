use std::fs::{DirEntry, File};
use std::ops::RangeInclusive;
use std::os::unix::prelude::MetadataExt;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::{cmp, fs, thread};

use crate::{BlockCoordinates, ExportParams, VoxelStack};
use fastanvil::{Chunk, CurrentJavaChunk, Region};
use fastnbt::from_bytes;

const CHUNK_BLOCKS_SIZE: usize = 16;
const FILE_CHUNKS_SIZE: isize = 32;
const FILE_BLOCKS_SIZE: isize = CHUNK_BLOCKS_SIZE as isize * FILE_CHUNKS_SIZE as isize;

pub(crate) fn read_level(lvl_path: &str, params: ExportParams) -> VoxelStack {
    let needed_files = get_needed_filenames(&params);

    let paths = fs::read_dir(lvl_path).expect("Cannot read lvl dir");
    let files: Vec<DirEntry> = paths
        .into_iter()
        .flatten()
        .filter(|dir| dir.metadata().map_or(false, |meta| meta.size() > 0))
        .filter(|dir| {
            dir.file_name().to_str().map_or(false, |filename| {
                needed_files.contains(&filename.to_owned())
            })
        })
        .collect();

    let (sender, receiver) = channel();
    let amount_of_files = files.len();

    let export_params = Arc::new(params);

    for dir_entry in files {
        let p = export_params.clone();
        let own_sender = sender.clone();

        thread::spawn(move || {
            let voxels = read_level_file(&dir_entry, &p);

            own_sender
                .send(voxels)
                .expect("Cannot send image from thread");
        });
    }

    let mut received = 0;
    let mut stack = VoxelStack::default();

    for voxels in receiver {
        stack.add_all(voxels);
        received += 1;

        if received == amount_of_files {
            break;
        }
    }

    stack
}

fn read_level_file(dir_entry: &DirEntry, params: &ExportParams) -> Vec<BlockCoordinates> {
    let mut voxels = vec![];
    let blocks_to_skip: Vec<&str> = params
        .skip_blocks
        .iter()
        .map(std::ops::Deref::deref)
        .collect();

    let (path, filename) = (
        dir_entry
            .path()
            .to_str()
            .expect("Cannot convert file path to str")
            .to_string(),
        dir_entry
            .file_name()
            .to_str()
            .expect("Cannot convert file name to str")
            .to_string(),
    );

    let file = File::open(&path).unwrap_or_else(|_| panic!("Can't open file {}", &path));
    let d = filename[2..filename.len() - 4]
        .split(".")
        .collect::<Vec<&str>>();
    let file_x = d[0].parse::<isize>().unwrap();
    let file_z = d[1].parse::<isize>().unwrap();
    let (x_range, z_range) = get_chunk_ranges(file_x, file_z, &params);

    let mut region = Region::from_stream(file).expect("Cannot create region from file.");
    let file_x_bonus = file_x * FILE_CHUNKS_SIZE * CHUNK_BLOCKS_SIZE as isize;
    let file_z_bonus = file_z * FILE_CHUNKS_SIZE * CHUNK_BLOCKS_SIZE as isize;

    region.iter().flatten().for_each(|raw_chunk| {
        let mut chunk_min_x = (raw_chunk.x * CHUNK_BLOCKS_SIZE) as isize;
        if file_x < 0 {
            chunk_min_x = -chunk_min_x + FILE_BLOCKS_SIZE;
        }
        chunk_min_x += file_x_bonus;
        let mut chunk_min_z = (raw_chunk.z * CHUNK_BLOCKS_SIZE) as isize;
        if file_z < 0 {
            chunk_min_z = -chunk_min_z + FILE_BLOCKS_SIZE;
        }
        chunk_min_z += file_z_bonus;

        let chunk_max_x = chunk_min_x + CHUNK_BLOCKS_SIZE as isize;
        let chunk_max_z = chunk_min_z + CHUNK_BLOCKS_SIZE as isize;
        let chunk_x_range = chunk_min_x..chunk_max_x;
        let chunk_z_range = chunk_min_z..chunk_max_z;

        let x_is_valid = chunk_x_range.contains(x_range.start())
            || chunk_x_range.contains(x_range.end())
            || x_range.contains(&chunk_x_range.start)
            || x_range.contains(&chunk_x_range.end);
        let z_is_valid = chunk_z_range.contains(z_range.start())
            || chunk_z_range.contains(z_range.end())
            || z_range.contains(&chunk_z_range.start)
            || z_range.contains(&chunk_z_range.end);

        let chunk_is_valid = x_is_valid && z_is_valid;

        if !chunk_is_valid {
            return;
        }

        let chunk: CurrentJavaChunk =
            from_bytes(raw_chunk.data.as_slice()).expect("Cannot parse chunk data.");

        for y in params.start.y..=params.end.y {
            for x in 0..CHUNK_BLOCKS_SIZE {
                for z in 0..CHUNK_BLOCKS_SIZE {
                    let block_x = chunk_min_x + x as isize;
                    let block_z = chunk_min_z + z as isize;

                    if x_range.contains(&block_x) && z_range.contains(&block_z) {
                        chunk
                            .block(x, y, z)
                            .filter(|block| {
                                block.name() != "minecraft:air"
                                    && !blocks_to_skip.contains(&block.name())
                            })
                            .map(|_block| {
                                let point = BlockCoordinates::new(block_x, y, block_z);

                                voxels.push(point);
                            });
                    }
                }
            }
        }
    });

    voxels
}

fn get_chunk_ranges(
    file_x: isize,
    file_z: isize,
    params: &ExportParams,
) -> (RangeInclusive<isize>, RangeInclusive<isize>) {
    let x_range = get_chunk_coordinate_ranges(file_x, params.start.x, params.end.x);
    let z_range = get_chunk_coordinate_ranges(file_z, params.start.z, params.end.z);

    (x_range, z_range)
}

fn get_chunk_coordinate_ranges(
    file_c: isize,
    start_c: isize,
    end_c: isize,
) -> RangeInclusive<isize> {
    if file_c < 0 {
        let min = cmp::max((file_c - 1) * FILE_BLOCKS_SIZE, start_c);
        let max = cmp::min((file_c + 1) * FILE_BLOCKS_SIZE, end_c);

        min..=max
    } else {
        let min = cmp::max(file_c * FILE_BLOCKS_SIZE, start_c);
        let max = cmp::min((file_c + 1) * FILE_BLOCKS_SIZE, end_c);

        min..=max
    }
}

fn get_needed_filenames(params: &ExportParams) -> Vec<String> {
    let mut needed_files = vec![];
    let get_file_index =
        |c: isize| -> isize { (c as f32 / FILE_BLOCKS_SIZE as f32).floor() as isize };
    let start_x = get_file_index(params.start.x);
    let start_z = get_file_index(params.start.z);
    let end_x = get_file_index(params.end.x);
    let end_z = get_file_index(params.end.z);

    for x in start_x..=end_x {
        for z in start_z..=end_z {
            needed_files.push(format!("r.{}.{}.mca", x, z));
        }
    }
    if needed_files.is_empty() {
        needed_files.push(format!("r.{}.{}.mca", start_x, start_z));
    }
    needed_files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_level_export_range_1() {
        let result = read_level(
            "./assets/test_lvl",
            ExportParams {
                start: BlockCoordinates::new(1, -63, 1),
                end: BlockCoordinates::new(2, -63, 2),
                ..Default::default()
            },
        );
        assert_eq!(
            result,
            VoxelStack::from(vec![
                BlockCoordinates::new(1, -63, 1),
                BlockCoordinates::new(1, -63, 2),
                BlockCoordinates::new(2, -63, 1),
                BlockCoordinates::new(2, -63, 2),
            ])
        );
    }
    #[test]
    fn read_level_export_range_2() {
        let result = read_level(
            "./assets/test_lvl",
            ExportParams {
                start: BlockCoordinates::new(1, -63, 5),
                end: BlockCoordinates::new(2, -60, 6),
                ..Default::default()
            },
        );
        assert_eq!(
            result,
            VoxelStack::from(vec![
                BlockCoordinates::new(1, -63, 5),
                BlockCoordinates::new(1, -62, 5),
                BlockCoordinates::new(1, -61, 5),
            ])
        );
    }

    #[test]
    fn read_level_skip_blocks_1() {
        let result = read_level(
            "./assets/test_lvl",
            ExportParams {
                start: BlockCoordinates::new(1, -63, 1),
                end: BlockCoordinates::new(2, -63, 2),
                skip_blocks: vec!["minecraft:stone".to_owned()],
            },
        );
        assert_eq!(result, VoxelStack::from(vec![]));
    }
    #[test]
    fn read_level_skip_blocks_2() {
        let result = read_level(
            "./assets/test_lvl",
            ExportParams {
                start: BlockCoordinates::new(1, -64, 1),
                end: BlockCoordinates::new(2, -63, 2),
                skip_blocks: vec!["minecraft:stone".to_owned()],
            },
        );
        assert_eq!(
            result,
            VoxelStack::from(vec![
                BlockCoordinates::new(1, -64, 1),
                BlockCoordinates::new(1, -64, 2),
                BlockCoordinates::new(2, -64, 1),
                BlockCoordinates::new(2, -64, 2),
            ])
        );
    }

    #[test]
    fn get_chunk_ranges_1() {
        assert_eq!(get_chunk_coordinate_ranges(-1, -10, -2), -10..=-2);
        assert_eq!(get_chunk_coordinate_ranges(0, 0, 50), 0..=50);
        assert_eq!(get_chunk_coordinate_ranges(0, -10, 1000), 0..=512);
        assert_eq!(get_chunk_coordinate_ranges(1, -10, 1000), 512..=1000);
    }
    #[test]
    fn get_needed_filenames_1() {
        let result = get_needed_filenames(&ExportParams {
            start: BlockCoordinates::new(-1, 0, -1),
            end: BlockCoordinates::new(1, 0, 1),
            ..Default::default()
        });

        assert_eq!(
            result,
            vec![
                String::from("r.-1.-1.mca"),
                String::from("r.-1.0.mca"),
                String::from("r.0.-1.mca"),
                String::from("r.0.0.mca")
            ]
        );
    }
    #[test]
    fn get_needed_filenames_0_0() {
        let result = get_needed_filenames(&ExportParams {
            start: BlockCoordinates::new(1, 0, 1),
            end: BlockCoordinates::new(2, 0, 2),
            ..Default::default()
        });

        assert_eq!(result, vec![String::from("r.0.0.mca")]);
    }
    #[test]
    fn get_needed_filenames_1_0() {
        let result = get_needed_filenames(&ExportParams {
            start: BlockCoordinates::new(513, 1, 1),
            end: BlockCoordinates::new(523, 1, 2),
            ..Default::default()
        });

        assert_eq!(result, vec![String::from("r.1.0.mca")]);
    }
    #[test]
    fn get_needed_filenames_0_1() {
        let result = get_needed_filenames(&ExportParams {
            start: BlockCoordinates::new(1, 1, 513),
            end: BlockCoordinates::new(2, 1, 523),
            ..Default::default()
        });

        assert_eq!(result, vec![String::from("r.0.1.mca")]);
    }
    #[test]
    fn get_needed_filenames_1_1() {
        let result = get_needed_filenames(&ExportParams {
            start: BlockCoordinates::new(513, 1, 513),
            end: BlockCoordinates::new(513, 1, 523),
            ..Default::default()
        });

        assert_eq!(result, vec![String::from("r.1.1.mca")]);
    }
    #[test]
    fn get_needed_filenames_minus_2_2() {
        let result = get_needed_filenames(&ExportParams {
            start: BlockCoordinates::new(-513, 1, -513),
            end: BlockCoordinates::new(-513, 1, -523),
            ..Default::default()
        });

        assert_eq!(result, vec![String::from("r.-2.-2.mca")]);
    }
}
