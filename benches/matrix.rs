use criterion::{criterion_group, criterion_main, Criterion};
use matrix_rs::matrix::*;

fn a(c: &mut Criterion) {
    let a = Matrix::new(vec![vec![1, 2], vec![3, 4]]);

    let other = Matrix::new(vec![vec![1, 2], vec![3, 4]]);

    c.bench_function("mul 2x2 batched", move |b| {
        b.iter_batched(
            || (a.clone(), other.clone()),
            |(a, other)| a * other,
            criterion::BatchSize::SmallInput,
        )
    });
}

fn b(c: &mut Criterion) {
    let a = Matrix::new(vec![vec![1, 2], vec![3, 4]]);

    let other = Matrix::new(vec![vec![1, 2], vec![3, 4]]);

    c.bench_function("mul 2x2", |b| b.iter(|| a.clone() * other.clone()));
}

fn c(c: &mut Criterion) {
    let a = Matrix::new(vec![vec![1, 2], vec![3, 4]]);

    let other = Matrix::new(vec![vec![1, 2], vec![3, 4]]);

    c.bench_function("strass 2x2", |b| b.iter(|| a.strass(&other)));
}

fn d(c: &mut Criterion) {
    let a = Matrix::new(vec![vec![1, 2], vec![3, 4]]);

    let other = Matrix::new(vec![vec![1, 2], vec![3, 4]]);

    c.bench_function("strass 2x2 batched", move |b| {
        b.iter_batched(
            || (a.clone(), other.clone()),
            |(a, other)| a .strass(&other),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(mul_2x2_bench, a, b, c,d);
criterion_main!(mul_2x2_bench);
