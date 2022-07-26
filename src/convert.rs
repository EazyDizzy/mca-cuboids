use std::any::Any;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::unix::prelude::MetadataExt;

use fastanvil::{Block, Chunk, CurrentJavaChunk, Region};
use fastnbt::from_bytes;
use crate::{Vec3, vec3};

const LVL_DIR: &str = "./assets/lvl/";
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
    let mut voxels = vec![];

    for p in paths {
        let (path, filename) = if let Ok(a) = p {
            (
                a.path()
                    .to_str()
                    .expect("Cannot convert file path to str")
                    .to_string(),
                a.file_name()
                    .to_str()
                    .expect("Cannot convert file name to str")
                    .to_string(),
            )
        } else {
            continue;
        };
        if let Ok(original_metadata) = fs::metadata(&path) {
            if original_metadata.size() == 0 {
                continue;
            }
        } else {
            continue;
        };

        let file = File::open(&path).unwrap_or_else(|_| panic!("Can't open file {}", &path));
        let d = filename[2..filename.len() - 4]
            .split(".")
            .collect::<Vec<&str>>();
        let file_x = d[0].parse::<isize>().unwrap();
        let file_z = d[1].parse::<isize>().unwrap();

        let mut region = Region::from_stream(file).expect("Cannot create region from file.");
        region.iter().flatten().for_each(|chunk| {
            let chunk_x = chunk.x;
            let chunk_z = chunk.z;

            let data = chunk.data;
            let chunk: CurrentJavaChunk =
                from_bytes(data.as_slice()).expect("Cannot parse chunk data.");

            for x in 0..CHUNK_BLOCKS_SIZE {
                for z in 0..CHUNK_BLOCKS_SIZE {
                    for y in chunk.y_range() {
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
    }

    voxels
}
