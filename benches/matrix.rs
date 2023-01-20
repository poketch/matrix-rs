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

fn e(c: &mut Criterion) {
    let a = Matrix::from_list(3, 3, (1..=9).collect());
    
    let other = Matrix::from_list(3, 3, (1..=9).collect());
    
    c.bench_function("mul 3x3 batched", move |b| {
        b.iter_batched(
            || (a.clone(), other.clone()),
            |(a, other)| a * other,
            criterion::BatchSize::SmallInput,
        )
    });
}


fn f(c: &mut Criterion) {
    let a = Matrix::from_list(3, 3, (1..=9).collect());
    
    let other = Matrix::from_list(3, 3, (1..=9).collect());
    
    c.bench_function("strass 3x3", |b| b.iter(|| a.strass(&other)));
}

fn g(c: &mut Criterion) {
    let a = Matrix::from_list(3, 3, (1..=9).collect());
    
    let other = Matrix::from_list(3, 3, (1..=9).collect());

    c.bench_function("strass 3x3 batched", move |b| {
        b.iter_batched(
            || (a.clone(), other.clone()),
            |(a, other)| a .strass(&other),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(mul_2x2_bench, a, b, c,d);
criterion_group!(mul_3x3_bench, e, f, g);
criterion_main!(mul_2x2_bench, mul_3x3_bench);
