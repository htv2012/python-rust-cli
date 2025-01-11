use criterion::{black_box, criterion_group, criterion_main, Criterion};
use myfib::fibonacci;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

pub fn b25(c: &mut Criterion) {
    c.bench_function("fib 25", |b| b.iter(|| fibonacci(black_box(25))));
}

criterion_group!{
    name=benches;
    config = Criterion::default();
    targets = criterion_benchmark, b25
}
criterion_main!(benches);

