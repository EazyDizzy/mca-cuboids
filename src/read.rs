use std::fs::{DirEntry, File};
use std::ops::RangeInclusive;
use std::os::unix::prelude::MetadataExt;
use std::sync::mpsc::channel;
use std::{cmp, fs, thread};

use crate::{vec3, ExportParams, Vec3};
use fastanvil::{Chunk, CurrentJavaChunk, Region};
use fastnbt::from_bytes;

const CHUNK_BLOCKS_SIZE: usize = 16;
const FILE_CHUNKS_SIZE: isize = 32;
const FILE_BLOCKS_SIZE: isize = CHUNK_BLOCKS_SIZE as isize * FILE_CHUNKS_SIZE as isize;

pub(crate) fn read_level(lvl_path: &str, params: ExportParams) -> Vec<Vec3> {
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

    for dir_entry in files {
        let own_sender = sender.clone();
        let p = params.clone();

        thread::spawn(move || {
            let voxels = read_level_file(&dir_entry, p);

            own_sender
                .send(voxels)
                .expect("Cannot send image from thread");
        });
    }

    let mut all_voxels = vec![];
    let mut received = 0;

    for mut voxels in receiver {
        all_voxels.append(&mut voxels);
        received += 1;

        if received == amount_of_files {
            break;
        }
    }

    all_voxels
}

fn read_level_file(dir_entry: &DirEntry, params: ExportParams) -> Vec<Vec3> {
    let mut voxels = vec![];

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
    region.iter().flatten().for_each(|raw_chunk| {
        let chunk_x = raw_chunk.x;
        let chunk_z = raw_chunk.z;

        let mut x_base = (chunk_x * CHUNK_BLOCKS_SIZE) as isize;
        if file_x < 0 {
            x_base = -x_base;
        }
        x_base += file_x * FILE_CHUNKS_SIZE * CHUNK_BLOCKS_SIZE as isize;
        let mut z_base = (chunk_z * CHUNK_BLOCKS_SIZE) as isize;
        if file_z < 0 {
            z_base = -z_base;
        }
        z_base += file_z * FILE_CHUNKS_SIZE * CHUNK_BLOCKS_SIZE as isize;
        let chunk_x_range = x_base..=x_base + CHUNK_BLOCKS_SIZE as isize;
        let chunk_z_range = z_base..=z_base + CHUNK_BLOCKS_SIZE as isize;

        let valid_chunk = (chunk_x_range.contains(x_range.start())
            || chunk_x_range.contains(x_range.end()))
            && (chunk_z_range.contains(z_range.start()) || chunk_z_range.contains(z_range.end()));

        if !valid_chunk {
            return;
        }

        let y_range = params.start.y..=params.end.y;

        let bytes = raw_chunk.data;
        let chunk: CurrentJavaChunk =
            from_bytes(bytes.as_slice()).expect("Cannot parse chunk data.");

        for y in y_range {
            for x in 0..CHUNK_BLOCKS_SIZE {
                for z in 0..CHUNK_BLOCKS_SIZE {
                    let voxel_x = x_base + x as isize;
                    let voxel_z = z_base + z as isize;

                    if x_range.contains(&voxel_x) && z_range.contains(&voxel_z) {
                        chunk
                            .block(x, y, z)
                            .filter(|block| block.name() != "minecraft:air")
                            .map(|_block| {
                                let point = vec3(voxel_x, y, voxel_z);

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
        let max = cmp::max(file_c * FILE_BLOCKS_SIZE, end_c);

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
    fn read_level_1() {
        let result = read_level(
            "./assets/test_lvl",
            ExportParams {
                start: vec3(1, -63, 1),
                end: vec3(2, -63, 2),
                skip_blocks: None,
            },
        );
        assert_eq!(
            result.as_slice(),
            vec![
                vec3(1, -63, 1),
                vec3(1, -63, 2),
                vec3(2, -63, 1),
                vec3(2, -63, 2),
            ]
            .as_slice()
        );
    }#[test]
    fn read_level_2() {
        let result = read_level(
            "./assets/test_lvl",
            ExportParams {
                start: vec3(1, -63, 5),
                end: vec3(2, -60, 6),
                skip_blocks: None,
            },
        );
        assert_eq!(
            result.as_slice(),
            vec![
                vec3(1, -63, 5),
                vec3(1, -62, 5),
                vec3(1, -61, 5),
            ]
            .as_slice()
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
            start: vec3(-1, 0, -1),
            end: vec3(1, 0, 1),
            skip_blocks: None,
        });

        assert_eq!(
            result,
            vec![
                String::from("-1.-1.mca"),
                String::from("-1.0.mca"),
                String::from("0.-1.mca"),
                String::from("0.0.mca")
            ]
        );
    }
    #[test]
    fn get_needed_filenames_0_0() {
        let result = get_needed_filenames(&ExportParams {
            start: vec3(1, 0, 1),
            end: vec3(2, 0, 2),
            skip_blocks: None,
        });

        assert_eq!(result, vec![String::from("0.0.mca")]);
    }
    #[test]
    fn get_needed_filenames_1_0() {
        let result = get_needed_filenames(&ExportParams {
            start: vec3(513, 1, 1),
            end: vec3(523, 1, 2),
            skip_blocks: None,
        });

        assert_eq!(result, vec![String::from("1.0.mca")]);
    }
    #[test]
    fn get_needed_filenames_0_1() {
        let result = get_needed_filenames(&ExportParams {
            start: vec3(1, 1, 513),
            end: vec3(2, 1, 523),
            skip_blocks: None,
        });

        assert_eq!(result, vec![String::from("0.1.mca")]);
    }
    #[test]
    fn get_needed_filenames_1_1() {
        let result = get_needed_filenames(&ExportParams {
            start: vec3(513, 1, 513),
            end: vec3(513, 1, 523),
            skip_blocks: None,
        });

        assert_eq!(result, vec![String::from("1.1.mca")]);
    }
    #[test]
    fn get_needed_filenames_minus_2_2() {
        let result = get_needed_filenames(&ExportParams {
            start: vec3(-513, 1, -513),
            end: vec3(-513, 1, -523),
            skip_blocks: None,
        });

        assert_eq!(result, vec![String::from("-2.-2.mca")]);
    }
}
