use criterion::{criterion_group, criterion_main, Criterion};
use mca_collisions::read::read_level;
use pprof::criterion::{Output, PProfProfiler};
use mca_collisions::merge::merge_voxels;
use mca_collisions::voxel_stack::VoxelStack;

fn bench_merge(c: &mut Criterion) {
    let lvl = read_level("./assets/simple_lvl");
    let stack = VoxelStack::from(lvl);
    let mut group = c.benchmark_group("merge");
    group.sample_size(10);

    group.bench_function("merge", |b| {
        b.iter(|| merge_voxels(stack.clone()))
    });

    group.finish()
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_merge
}
criterion_main!(benches);
