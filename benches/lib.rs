use criterion::{criterion_group, criterion_main, Criterion};
use mca_cuboids::{export_cuboids, BlockCoordinates, ExportParams};
use pprof::criterion::{Output, PProfProfiler};

fn bench_export_cuboids(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_lvl");
    group.sample_size(10);

    group.bench_function("no collisions, small area", |b| {
        b.iter(|| {
            export_cuboids(
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
            export_cuboids(
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
            export_cuboids(
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
            export_cuboids(
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
            export_cuboids(
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
            export_cuboids(
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
    targets = bench_export_cuboids
}
criterion_main!(benches);
