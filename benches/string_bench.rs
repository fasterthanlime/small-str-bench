use criterion::{
    criterion_group, criterion_main,
    measurement::{Measurement, WallTime},
    BatchSize, BenchmarkGroup, BenchmarkId, Criterion,
};
use smartstring::{LazyCompact, SmartString};
use smol_str::SmolStr;
use std::{sync::Arc, time::Duration};

const FAST_AND_INACCURATE: bool = true;
const SIZES: &[usize] = &[3, 5, 8, 15, 24, 50, 120, 230, 550];

trait SizeGroup {
    fn size_group<F: Fn(&mut BenchmarkGroup<WallTime>, usize)>(&mut self, name: &str, f: F);
}

impl SizeGroup for Criterion {
    fn size_group<F: Fn(&mut BenchmarkGroup<WallTime>, usize)>(&mut self, name: &str, f: F) {
        let mut group = self.benchmark_group(name);
        if FAST_AND_INACCURATE {
            group.measurement_time(Duration::from_millis(200));
            group.warm_up_time(Duration::from_millis(50));
        }

        for &n in SIZES {
            f(&mut group, n)
        }
    }
}

trait BenchClone {
    fn bench_clone<S: From<String> + Clone>(&mut self, name: &str, n: usize);
}

impl<M: Measurement> BenchClone for BenchmarkGroup<'_, M> {
    fn bench_clone<S: From<String> + Clone>(&mut self, name: &str, n: usize) {
        self.bench_with_input(BenchmarkId::new(name, n), &n, |b, n| {
            b.iter_batched(
                || -> S { make_string(*n).into() },
                |s| s.clone(),
                BatchSize::SmallInput,
            )
        });
    }
}

fn benchmark(c: &mut Criterion) {
    c.size_group("clone", |g, n| {
        g.bench_clone::<String>("string", n);
        g.bench_clone::<Arc<String>>("arc", n);
        g.bench_clone::<SmolStr>("smol", n);
        g.bench_clone::<SmartString<LazyCompact>>("smart", n);
    });
}

fn make_string(size: usize) -> String {
    let mut s = String::new();
    for _ in 0..size {
        s.push(rand::random());
    }
    s
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
