use criterion::{black_box, criterion_group, criterion_main, Criterion};
use d11::{p1, p2};

pub fn criterion_benchmark(c: &mut Criterion) {
    // I think this assumes we are running 'cargo bench' from the root of the crate
    // need to figure out how to make this more robust
    let buf = std::fs::read_to_string("./input/input.txt").expect("failed to read input data");

    c.bench_function("loader", |b| {
        b.iter(|| std::fs::read_to_string(black_box("./input/input.txt")))
    });
    c.bench_function("p1", |b| b.iter(|| p1(black_box(&buf))));
    c.bench_function("p2", |b| b.iter(|| p2(black_box(&buf))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
