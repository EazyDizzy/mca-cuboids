use std::fs::{DirEntry, File};
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
    let needed_files = get_needed_filenames(params);

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

        thread::spawn(move || {
            let voxels = read_level_file(&dir_entry);

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

fn read_level_file(dir_entry: &DirEntry) -> Vec<Vec3> {
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

    let mut region = Region::from_stream(file).expect("Cannot create region from file.");
    region.iter().flatten().for_each(|raw_chunk| {
        let chunk_x = raw_chunk.x;
        let chunk_z = raw_chunk.z;

        let bytes = raw_chunk.data;
        let chunk: CurrentJavaChunk =
            from_bytes(bytes.as_slice()).expect("Cannot parse chunk data.");

        for y in chunk.y_range() {
            for x in 0..CHUNK_BLOCKS_SIZE {
                for z in 0..CHUNK_BLOCKS_SIZE {
                    if let Some(block) = chunk.block(x, y, z) {
                        if block.name() != "minecraft:air" {
                            let mut voxel_x = ((chunk_x * CHUNK_BLOCKS_SIZE) + x) as isize;
                            if file_x < 0 {
                                voxel_x = -voxel_x;
                            }
                            let mut voxel_z = ((chunk_z * CHUNK_BLOCKS_SIZE) + z) as isize;
                            if file_z < 0 {
                                voxel_z = -voxel_z;
                            }

                            voxel_x += file_x * FILE_CHUNKS_SIZE * CHUNK_BLOCKS_SIZE as isize;
                            voxel_z += file_z * FILE_CHUNKS_SIZE * CHUNK_BLOCKS_SIZE as isize;
                            let point = vec3(voxel_x, y, voxel_z);

                            voxels.push(point);
                        }
                    }
                }
            }
        }
    });

    voxels
}

fn get_needed_filenames(params: ExportParams) -> Vec<String> {
    let mut needed_files = vec![];
    let get_file_index = |c: isize| -> isize {
        let mut c_index = (c as f32 / FILE_BLOCKS_SIZE as f32).floor();

        c_index as isize
    };
    let start_x = get_file_index(params.start.x);
    let start_z = get_file_index(params.start.z);
    let end_x = get_file_index(params.end.x);
    let end_z = get_file_index(params.end.z);

    for x in start_x..=end_x {
        for z in start_z..=end_z {
            needed_files.push(format!("{}.{}.mca", x, z));
        }
    }
    if needed_files.is_empty() {
        needed_files.push(format!("{}.{}.mca", start_x, start_z));
    }
    needed_files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_needed_filenames_1() {
        let result = get_needed_filenames(ExportParams {
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
        let result = get_needed_filenames(ExportParams {
            start: vec3(1, 0, 1),
            end: vec3(2, 0, 2),
            skip_blocks: None,
        });

        assert_eq!(result, vec![String::from("0.0.mca")]);
    }
    #[test]
    fn get_needed_filenames_1_0() {
        let result = get_needed_filenames(ExportParams {
            start: vec3(513, 1, 1),
            end: vec3(523, 1, 2),
            skip_blocks: None,
        });

        assert_eq!(result, vec![String::from("1.0.mca")]);
    }
    #[test]
    fn get_needed_filenames_0_1() {
        let result = get_needed_filenames(ExportParams {
            start: vec3(1, 1, 513),
            end: vec3(2, 1, 523),
            skip_blocks: None,
        });

        assert_eq!(result, vec![String::from("0.1.mca")]);
    }
    #[test]
    fn get_needed_filenames_1_1() {
        let result = get_needed_filenames(ExportParams {
            start: vec3(513, 1, 513),
            end: vec3(513, 1, 523),
            skip_blocks: None,
        });

        assert_eq!(result, vec![String::from("1.1.mca")]);
    }#[test]
    fn get_needed_filenames_minus_2_2() {
        let result = get_needed_filenames(ExportParams {
            start: vec3(-513, 1, -513),
            end: vec3(-513, 1, -523),
            skip_blocks: None,
        });

        assert_eq!(result, vec![String::from("-2.-2.mca")]);
    }
}
