// pricing-core/benches/throughput.rs

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use pricing_core::{bs_mc, bs_mc_parallel};

fn bench_bs_variants(c: &mut Criterion) {
    let mut group = c.benchmark_group("BlackScholes_MC");
    // Hier definieren wir mehrere Szenarien
    let path_sizes = [100_000usize, 1_000_000, 5_000_000];

    for &n in &path_sizes {
        group.bench_with_input(BenchmarkId::new("seq", n), &n, |b, &n| {
            b.iter(|| bs_mc(n, 100.0, 100.0, 0.05, 0.2, 1.0))
        });
        group.bench_with_input(BenchmarkId::new("par", n), &n, |b, &n| {
            b.iter(|| bs_mc_parallel(n, 100.0, 100.0, 0.05, 0.2, 1.0))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_bs_variants);
criterion_main!(benches);
