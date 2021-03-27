use async_std::task;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pricefetchlib::calc::{max, min, n_window_sma, price_diff};
use rand::Rng;

const SIZE: usize = 4000;

pub fn min_bench(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let num = (0..SIZE)
        .map(|_| rng.gen_range(75.0..125.0))
        .collect::<Vec<f64>>();

    c.bench_function("min", |b| b.iter(|| task::block_on(min(black_box(&num)))));
}

pub fn max_bench(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let num = (0..SIZE)
        .map(|_| rng.gen_range(75.0..125.0))
        .collect::<Vec<f64>>();

    c.bench_function("max", |b| b.iter(|| task::block_on(max(black_box(&num)))));
}

pub fn sma_bench(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let num = (0..SIZE)
        .map(|_| rng.gen_range(75.0..125.0))
        .collect::<Vec<f64>>();

    // Took idea from project solution on how to vary the amount using slices
    c.bench_function("sma-10-1000", |b| {
        b.iter(|| task::block_on(n_window_sma(10, black_box(&num[0..1000]))))
    });
    c.bench_function("sma-10-2000", |b| {
        b.iter(|| task::block_on(n_window_sma(10, black_box(&num[0..2000]))))
    });
    c.bench_function("sma-10-3000", |b| {
        b.iter(|| task::block_on(n_window_sma(10, black_box(&num[0..3000]))))
    });
    c.bench_function("sma-10-4000", |b| {
        b.iter(|| task::block_on(n_window_sma(10, black_box(&num))))
    });
    c.bench_function("sma-30-1000", |b| {
        b.iter(|| task::block_on(n_window_sma(30, black_box(&num[0..1000]))))
    });
    c.bench_function("sma-30-2000", |b| {
        b.iter(|| task::block_on(n_window_sma(30, black_box(&num[0..2000]))))
    });
    c.bench_function("sma-30-3000", |b| {
        b.iter(|| task::block_on(n_window_sma(30, black_box(&num[0..3000]))))
    });
    c.bench_function("sma-30-4000", |b| {
        b.iter(|| task::block_on(n_window_sma(30, black_box(&num))))
    });
}

pub fn diff_bench(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let num = (0..SIZE)
        .map(|_| rng.gen_range(75.0..125.0))
        .collect::<Vec<f64>>();

    c.bench_function("pd-10-1000", |b| {
        b.iter(|| task::block_on(price_diff(black_box(&num[0..1000]))))
    });
    c.bench_function("pd-10-2000", |b| {
        b.iter(|| task::block_on(price_diff(black_box(&num[0..2000]))))
    });
    c.bench_function("pd-10-3000", |b| {
        b.iter(|| task::block_on(price_diff(black_box(&num[0..3000]))))
    });
    c.bench_function("pd-10-4000", |b| {
        b.iter(|| task::block_on(price_diff(black_box(&num))))
    });
}

criterion_group!(benches, min_bench, max_bench, sma_bench, diff_bench);
criterion_main!(benches);
