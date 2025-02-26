use criterion::{Criterion, criterion_group, criterion_main};
fn bench(c: &mut Criterion) {
    c.bench_function("regular", |b| {
        let polynomial = benching_polynomials::Polynomial::random(10_000);
        b.iter(|| {
            polynomial.eval(3);
        })
    });
    c.bench_function("horner", |b| {
        let polynomial = benching_polynomials::Polynomial::random(10_000);
        b.iter(|| {
            polynomial.eval_horner(3);
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
