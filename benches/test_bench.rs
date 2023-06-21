use criterion::{criterion_group, criterion_main, Criterion};
use tie_parse::test;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("load lfd", |b| b.iter(test));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
