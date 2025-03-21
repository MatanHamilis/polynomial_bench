use benching_polynomials::{Polynomial, ScalarTrait};
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::{iter::Sum, num::Wrapping};

const DEGREE: usize = 8;
const EVALS: usize = 1;

fn bench_for_type<S: ScalarTrait + Sum>(c: &mut Criterion, type_name: &str) {
    let polynomial = Polynomial::<S>::random(DEGREE);
    let reversed_polynomial = polynomial.clone().reverse();
    let mut rng = rand::rng();
    let evals = (0..EVALS).map(|_| S::rand(&mut rng)).collect::<Vec<_>>();
    let e = S::rand(&mut rng);

    let mut group = c.benchmark_group(type_name);

    // each test passes the polynomial and the evaluations through a black_box to prevent the optimizer
    // from knowing anything about them.
    // we then sum the result and "return" them which will also be passed to a `black_box` by criterion
    // this prevents dead-code elimination and ensures that the benchmark is actually measuring the
    // time it takes to compute the sum of the evaluations. (and the addition there is very negligible)
    group.bench_function("regular single", |b| b.iter(|| polynomial.eval(&e)));
    group.bench_function("horner single", |b| b.iter(|| polynomial.eval_horner(&e)));
    group.bench_function("reversed regular single", |b| {
        b.iter(|| polynomial.reverse_eval(&e))
    });
    group.bench_function("reversed horner single", |b| {
        b.iter(|| polynomial.reverse_eval_horner(&e))
    });
    group.bench_function("regular", |b| {
        b.iter(|| {
            let (poly, evals) = black_box((&polynomial, evals.iter()));
            evals.map(|e| poly.eval(e)).sum::<S>()
        })
    });
    group.bench_function("horner", |b| {
        b.iter(|| {
            let (poly, evals) = black_box((&polynomial, evals.iter()));
            evals.map(|e| poly.eval_horner(e)).sum::<S>()
        })
    });
    group.bench_function("reversed regular", |b| {
        b.iter(|| {
            let (poly, evals) = black_box((&reversed_polynomial, evals.iter()));
            evals.map(|e| poly.reverse_eval(e)).sum::<S>()
        })
    });
    group.bench_function("reversed horner", |b| {
        b.iter(|| {
            let (poly, evals) = black_box((&reversed_polynomial, evals.iter()));
            evals.map(|e| poly.reverse_eval_horner(e)).sum::<S>()
        })
    });

    group.finish();
}

fn bench_u64(c: &mut Criterion) {
    bench_for_type::<Wrapping<u64>>(c, "u64");
}

fn bench_u128(c: &mut Criterion) {
    bench_for_type::<Wrapping<u128>>(c, "u128");
}

fn bench_k256(c: &mut Criterion) {
    bench_for_type::<k256::Scalar>(c, "k256::Scalar");
}

fn bench_curve25519(c: &mut Criterion) {
    bench_for_type::<curve25519_dalek::Scalar>(c, "curve25519_dalek::Scalar");
}

criterion_group!(benches, bench_u64, bench_u128, bench_k256, bench_curve25519);
criterion_main!(benches);
