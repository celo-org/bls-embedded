#[macro_use]
extern crate criterion;

extern crate bls12_377;
use bls12_377::*;
use bls12_377::fp::Fp;

use criterion::{black_box, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    // G1Affine
    {
        let name = "G1Affine";
        let a = G1Affine::generator();
        let s = Scalar::from_raw([1, 2, 3, 4]);
        let compressed = [0u8; 48];
        let uncompressed = [0u8; 96];
        c.bench_function(&format!("{}_check_on_curve", name), move |b| {
            b.iter(|| black_box(a).is_on_curve())
        });
        c.bench_function(&format!("{}_check_equality", name), move |b| {
            b.iter(|| black_box(a) == black_box(a))
        });
        c.bench_function(&format!("{}_scalar_multiplication", name), move |b| {
            b.iter(|| black_box(a) * black_box(s))
        });
        c.bench_function(&format!("{}_subgroup_check", name), move |b| {
            b.iter(|| black_box(a).is_torsion_free())
        });
        c.bench_function(
            &format!("{} deserialize uncompressed point", name),
            move |b| b.iter(|| G1Affine::from_uncompressed(black_box(&uncompressed))),
        );
    }

    // G1Projective
    {
        let name = "G1Projective";
        let a = G1Projective::generator();
        let a_affine = G1Affine::generator();
        let s = Scalar::from_raw([1, 2, 3, 4]);

        const N: usize = 10000;
        let v = vec![G1Projective::generator(); N];
        let mut q = vec![G1Affine::identity(); N];

        c.bench_function(&format!("{}_check_on_curve", name), move |b| {
            b.iter(|| black_box(a).is_on_curve())
        });
        c.bench_function(&format!("{}_check_equality", name), move |b| {
            b.iter(|| black_box(a) == black_box(a))
        });
        c.bench_function(&format!("{}_to_affine", name), move |b| {
            b.iter(|| G1Affine::from(black_box(a)))
        });
        c.bench_function(&format!("{}_doubling", name), move |b| {
            b.iter(|| black_box(a).double())
        });
        c.bench_function(&format!("{}_addition", name), move |b| {
            b.iter(|| black_box(a).add(&a))
        });
        c.bench_function(&format!("{}_mixed_addition", name), move |b| {
            b.iter(|| black_box(a).add_mixed(&a_affine))
        });
        c.bench_function(&format!("{}_scalar_multiplication", name), move |b| {
            b.iter(|| black_box(a) * black_box(s))
        });
        c.bench_function(&format!("{}_batch_to_affine_n={}", name, N), move |b| {
            b.iter(|| {
                G1Projective::batch_normalize(black_box(&v), black_box(&mut q));
                black_box(&q)[0]
            })
        });
    }

    // G2Affine
    {
        let name = "G2Affine";
        let a = G2Affine::generator();
        let s = Scalar::from_raw([1, 2, 3, 4]);
        let compressed = [0u8; 96];
        let uncompressed = [0u8; 192];
        c.bench_function(&format!("{}_check_on_curve", name), move |b| {
            b.iter(|| black_box(a).is_on_curve())
        });
        c.bench_function(&format!("{}_check_equality", name), move |b| {
            b.iter(|| black_box(a) == black_box(a))
        });
        c.bench_function(&format!("{}_scalar_multiplication", name), move |b| {
            b.iter(|| black_box(a) * black_box(s))
        });
        c.bench_function(&format!("{}_subgroup_check", name), move |b| {
            b.iter(|| black_box(a).is_torsion_free())
        });
        c.bench_function(
            &format!("{} deserialize uncompressed point", name),
            move |b| b.iter(|| G2Affine::from_uncompressed(black_box(&uncompressed))),
        );
    }

    // G2Projective
    {
        let name = "G2Projective";
        let a = G2Projective::generator();
        let a_affine = G2Affine::generator();
        let s = Scalar::from_raw([1, 2, 3, 4]);

        const N: usize = 10000;
        let v = vec![G2Projective::generator(); N];
        let mut q = vec![G2Affine::identity(); N];

        c.bench_function(&format!("{}_check_on_curve", name), move |b| {
            b.iter(|| black_box(a).is_on_curve())
        });
        c.bench_function(&format!("{}_check_equality", name), move |b| {
            b.iter(|| black_box(a) == black_box(a))
        });
        c.bench_function(&format!("{}_to_affine", name), move |b| {
            b.iter(|| G2Affine::from(black_box(a)))
        });
        c.bench_function(&format!("{}_doubling", name), move |b| {
            b.iter(|| black_box(a).double())
        });
        c.bench_function(&format!("{}_addition", name), move |b| {
            b.iter(|| black_box(a).add(&a))
        });
        c.bench_function(&format!("{}_mixed_addition", name), move |b| {
            b.iter(|| black_box(a).add_mixed(&a_affine))
        });
        c.bench_function(&format!("{}_scalar_multiplication", name), move |b| {
            b.iter(|| black_box(a) * black_box(s))
        });
        c.bench_function(&format!("{}_batch_to_affine_n={}", name, N), move |b| {
            b.iter(|| {
                G2Projective::batch_normalize(black_box(&v), black_box(&mut q));
                black_box(&q)[0]
            })
        });
    }
    // Fp Arithmetic
    {
       let x = Fp::one();
       let y = Fp::one();
       c.bench_function("Fp_multiplication_new",  
           move |b| {
               b.iter(|| black_box(x).mul(&black_box(y)))
           });
       c.bench_function("Fp_multiplication_old",  
           move |b| {
               b.iter(|| black_box(x).mul_old(&black_box(y)))
           });
       c.bench_function("Fp_inverse",
           move |b| {
               b.iter(|| black_box(x).invert())
           });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
