// Copyright 2021 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkGroup, BenchmarkId, Criterion};
use num_traits::{One, WrappingAdd};
use rand::distributions::uniform::{SampleUniform, UniformSampler};
use rand::prelude::*;

type BenchRng = SmallRng;

fn bench_group<T: Copy + One + WrappingAdd + SampleUniform>(
    mut g: BenchmarkGroup<criterion::measurement::WallTime>, inputs: &[(&str, (T, T))],
) {
    macro_rules! do_group {
        ($name:literal, $f:ident) => {
            for input in inputs {
                g.bench_with_input(
                    BenchmarkId::new($name, input.0),
                    &input.1,
                    |b, (low, high)| {
                        let mut rng = BenchRng::from_entropy();
                        b.iter(|| T::Sampler::$f(low, high, &mut rng))
                    },
                );
            }
            g.bench_function(BenchmarkId::new($name, "varying"), |b| {
                let (low, mut high) = (inputs[0].1 .0, inputs[1].1 .1);
                let mut rng = BenchRng::from_entropy();
                b.iter(|| {
                    high = high.wrapping_add(&T::one());
                    T::Sampler::$f(low, high, &mut rng)
                })
            });
        };
    }

    do_group!("ONeill", sample_single_inclusive_oneill);
    do_group!("Canon", sample_single_inclusive_canon);
    do_group!("Canon-Lemire", sample_inclusive_canon_lemire);
    do_group!("Bitmask", sample_single_inclusive_bitmask);
}

macro_rules! bench {
    ($name:ident, $high:expr) => {
        fn $name(c: &mut Criterion) {
            bench_group(c.benchmark_group(stringify!($name)), &[
                ("high reject", $high),
                ("low reject", (-1, 2)),
            ]);
        }
    };
}

// for i8/i16, we use 32-bit integers internally so rejection is most common near full-size
// the exact values were determined with an exhaustive search
bench!(uniform_int_i8, (i8::MIN, 116));
bench!(uniform_int_i16, (i16::MIN, 32407));
bench!(uniform_int_i32, (i32::MIN, 1));
bench!(uniform_int_i64, (i64::MIN, 1));
bench!(uniform_int_i128, (i128::MIN, 1));

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = uniform_int_i8, uniform_int_i16, uniform_int_i32, uniform_int_i64, uniform_int_i128
}
criterion_main!(benches);
