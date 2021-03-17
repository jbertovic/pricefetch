use criterion::{criterion_group, criterion_main, Criterion};
use pricefetchlib::calc::{min, max, n_window_sma, price_diff};
use async_std::task;
use rand::Rng;

const SIZE: usize = 1024;

pub fn min_bench(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let num = (0..SIZE).map(|_| rng.gen_range(75.0..125.0)).collect::<Vec<f64>>();

    c.bench_function("min", |b| {
        b.iter_batched_ref(
            || num.clone(),
            |v| task::block_on(async { min(v).await } ) ,
            criterion::BatchSize::SmallInput)
    });
}

pub fn max_bench(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let num = (0..SIZE).map(|_| rng.gen_range(75.0..125.0)).collect::<Vec<f64>>();

    c.bench_function("max", |b| {
        b.iter_batched_ref(
            || num.clone(),
            |v| task::block_on(async { max(v).await } ) ,
            criterion::BatchSize::SmallInput)
    });
}

pub fn sma_bench(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let num = (0..SIZE).map(|_| rng.gen_range(75.0..125.0)).collect::<Vec<f64>>();

    c.bench_function("sma", |b| {
        b.iter_batched_ref(
            || num.clone(),
            |v| task::block_on(async { n_window_sma(30,v).await } ) ,
            criterion::BatchSize::SmallInput)
    });
}

pub fn diff_bench(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let num = (0..SIZE).map(|_| rng.gen_range(75.0..125.0)).collect::<Vec<f64>>();

    c.bench_function("diff", |b| {
        b.iter_batched_ref(
            || num.clone(),
            |v| task::block_on(async { price_diff(v).await } ) ,
            criterion::BatchSize::SmallInput)
    });
}
criterion_group!(benches, min_bench, max_bench, sma_bench, diff_bench);
criterion_main!(benches);

