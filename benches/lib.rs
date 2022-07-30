use criterion::{criterion_group, criterion_main, Criterion};
use mca_collisions::{detect_collisions, BlockCoordinates, ExportParams};
use pprof::criterion::{Output, PProfProfiler};

fn bench_detect_collisions(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_lvl");
    group.sample_size(10);

    group.bench_function("no collisions, small area", |b| {
        b.iter(|| {
            detect_collisions(
                "./assets/test_lvl",
                ExportParams {
                    start: BlockCoordinates::new(0, 0, 0),
                    end: BlockCoordinates::new(100, 100, 100),
                    ..Default::default()
                },
            )
        })
    });
    group.bench_function("no collisions, big area", |b| {
        b.iter(|| {
            detect_collisions(
                "./assets/test_lvl",
                ExportParams {
                    start: BlockCoordinates::new(-500, -64, -500),
                    end: BlockCoordinates::new(500, 500, 500),
                    ..Default::default()
                },
            )
        })
    });
    group.bench_function("small collisions, small area", |b| {
        b.iter(|| {
            detect_collisions(
                "./assets/test_lvl",
                ExportParams {
                    start: BlockCoordinates::new(0, -64, 0),
                    end: BlockCoordinates::new(100, 100, 100),
                    ..Default::default()
                },
            )
        })
    });
    group.bench_function("small collisions, big area", |b| {
        b.iter(|| {
            detect_collisions(
                "./assets/test_lvl",
                ExportParams {
                    start: BlockCoordinates::new(-500, -64, -500),
                    end: BlockCoordinates::new(500, 500, 500),
                    ..Default::default()
                },
            )
        })
    });
    group.bench_function("big collisions, small area", |b| {
        b.iter(|| {
            detect_collisions(
                "./assets/huge_lvl",
                ExportParams {
                    start: BlockCoordinates::new(0, -64, 0),
                    end: BlockCoordinates::new(100, 100, 100),
                    ..Default::default()
                },
            )
        })
    });
    group.bench_function("big collisions, big area", |b| {
        b.iter(|| {
            detect_collisions(
                "./assets/huge_lvl",
                ExportParams {
                    start: BlockCoordinates::new(-500, -64, -500),
                    end: BlockCoordinates::new(500, 500, 500),
                    ..Default::default()
                },
            )
        })
    });

    group.finish()
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_detect_collisions
}
criterion_main!(benches);
