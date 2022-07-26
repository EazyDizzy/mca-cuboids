use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mca_collisions::read::read_level_file;
use std::fs;
use std::fs::DirEntry;
use std::os::unix::prelude::MetadataExt;

fn bench_read_lvl_file(c: &mut Criterion) {
    let lvl_path = "./assets/simple_lvl";
    let paths = fs::read_dir(lvl_path).expect("Cannot read lvl dir");
    let files: Vec<DirEntry> = paths
        .into_iter()
        .flatten()
        .filter(|dir| dir.metadata().map_or(false, |meta| meta.size() > 0))
        .collect();
    let dir_entry = files.iter().next().unwrap();

    let mut group = c.benchmark_group("bench_read_lvl_file");
    group.sample_size(10);

    group.bench_function("read_level_file", |b| b.iter(|| read_level_file(dir_entry)));

    group.finish()
}

criterion_group!(benches, bench_read_lvl_file);
criterion_main!(benches);
