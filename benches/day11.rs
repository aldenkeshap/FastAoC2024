use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent::day11;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("short", |b| b.iter(|| day11::part1(black_box("125 17"))));
    c.bench_function("real", |b| {
        b.iter(|| day11::part1(black_box("92 0 286041 8034 34394 795 8 2051489")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
