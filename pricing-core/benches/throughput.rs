// pricing-core/benches/throughput.rs

use criterion::{
    criterion_group, criterion_main, Criterion,
    BenchmarkId,
};
use pricing_core::{bs_mc, bs_mc_parallel};
use std::time::Duration;

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
fn criterion_config() -> Criterion {
    Criterion::default()
        .sample_size(50)                           // weniger Samples, wenn’s sonst zu lange dauert
        .measurement_time(Duration::from_secs(20)) // Messdauer pro Benchmark
        .warm_up_time(Duration::from_secs(15))      // Warm‑up vor jeder Messung
                           // volle Ausgabe auf der Konsole
}

criterion_group! {
    name = benches;                     // Name der Gruppe
    config = criterion_config();        // hier wird Deine Funktion verwendet
    targets = bench_bs_variants         // Deine Bench‑Funktion(en)
}
criterion_main!(benches);
