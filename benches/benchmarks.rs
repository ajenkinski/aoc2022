use criterion::{black_box, criterion_group, criterion_main, Criterion};

use itertools::Itertools;

// Benchmarking two ways of checking whether a sequence of elements are unique
pub fn benchmark_all_unique(c: &mut Criterion) {
    let sizes = (10..=50).step_by(10).chain((100..=1000).step_by(100));
    for n in sizes {
        let input = (0..n).collect_vec();

        c.bench_function(format!("all_unique: method n={}", n).as_str(), |b| {
            b.iter(|| black_box(&input).iter().all_unique())
        });

        c.bench_function(format!("all_unique: tuple_combinations n={}", n).as_str(), |b| {
            b.iter(|| {
                black_box(&input)
                    .iter()
                    .tuple_combinations()
                    .all(|(a, b)| a != b)
            })
        });
    }
}

criterion_group!(benches, benchmark_all_unique);
criterion_main!(benches);
