use std::fs::{DirEntry, File};
use std::os::unix::prelude::MetadataExt;
use std::sync::mpsc::channel;
use std::{fs, thread};

use crate::{vec3, Vec3};
use fastanvil::{Chunk, CurrentJavaChunk, Region};
use fastnbt::from_bytes;

const CHUNK_BLOCKS_SIZE: usize = 16;
const FILE_CHUNKS_SIZE: isize = 32;

// TODO concurrency
// TODO use
struct ExportParams {
    start: Vec3,
    end: Vec3,
    skip_blocks: Vec<String>,
}

pub fn read_level(lvl_path: &str) -> Vec<Vec3> {
    let paths = fs::read_dir(lvl_path).expect("Cannot read lvl dir");
    let files: Vec<DirEntry> = paths
        .into_iter()
        .flatten()
        .filter(|dir| dir.metadata().map_or(false, |meta| meta.size() > 0))
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

#[test]
fn test_read_level_file() {
    let lvl_path = "./assets/simple_lvl";
    let v = read_level(lvl_path);
    let max_x = v.iter().max_by(|a, b| a.y.cmp(&b.y));

    dbg!(v.len());
    dbg!(max_x);
}
