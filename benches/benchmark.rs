use criterion::{Criterion, black_box, criterion_group, criterion_main};
fn bench(c: &mut Criterion) {
    let polynomial = benching_polynomials::Polynomial::random(10_000);
    let evals = (0..100).collect::<Vec<_>>();
    c.bench_function("regular", |b| {
        b.iter(|| {
            let (poly, evals) = black_box((&polynomial, evals.as_slice()));
            for &x in evals {
                black_box(poly.eval(x));
            }
        })
    });
    c.bench_function("horner", |b| {
        b.iter(|| {
            let (poly, evals) = black_box((&polynomial, evals.as_slice()));
            for &x in evals {
                black_box(poly.eval_horner(x));
            }
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
