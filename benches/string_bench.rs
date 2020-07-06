use criterion::{
    criterion_group, criterion_main,
    measurement::{Measurement, WallTime},
    AxisScale, BatchSize, BenchmarkGroup, BenchmarkId, Criterion, PlotConfiguration,
};
use smartstring::{LazyCompact, SmartString};
use smol_str::SmolStr;
use std::time::Duration;

const FAST_AND_INACCURATE: bool = false;
const FAST_SIZES: &[usize] = &[10, 100, 1000];
const GOOD_SIZES: &[usize] = &[1, 5, 10, 50, 100, 500, 1000, 5000, 10_000, 50_000];

trait SizeGroup {
    fn size_group<F: Fn(&mut BenchmarkGroup<WallTime>, usize)>(&mut self, name: &str, f: F);
}

impl SizeGroup for Criterion {
    fn size_group<F: Fn(&mut BenchmarkGroup<WallTime>, usize)>(&mut self, name: &str, f: F) {
        let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

        let mut group = self.benchmark_group(name);
        group.plot_config(plot_config);
        if FAST_AND_INACCURATE {
            group.measurement_time(Duration::from_millis(800));
            group.warm_up_time(Duration::from_millis(200));
        }

        let sizes = if FAST_AND_INACCURATE {
            FAST_SIZES
        } else {
            GOOD_SIZES
        };
        for &n in sizes {
            f(&mut group, n)
        }
    }
}

trait FromStr {
    fn from_str(s: &str) -> Self;
}

impl FromStr for String {
    fn from_str(s: &str) -> Self {
        Self::from(s)
    }
}

impl FromStr for SmolStr {
    fn from_str(s: &str) -> Self {
        Self::from(s)
    }
}

impl FromStr for SmartString<LazyCompact> {
    fn from_str(s: &str) -> Self {
        Self::from(s)
    }
}

trait Benches {
    fn bench_from<S: FromStr>(&mut self, name: &str, n: usize);
    fn bench_into<S: From<String> + Into<String>>(&mut self, name: &str, n: usize);
    fn bench_clone<S: From<String> + Clone>(&mut self, name: &str, n: usize);
}

impl<M: Measurement> Benches for BenchmarkGroup<'_, M> {
    fn bench_from<'a, S: FromStr>(&mut self, name: &str, n: usize) {
        self.bench_with_input(BenchmarkId::new(name, n), &n, |b, n| {
            b.iter_batched(
                || -> String { make_string(*n) },
                |s| S::from_str(&s),
                BatchSize::SmallInput,
            )
        });
    }

    fn bench_clone<S: From<String> + Clone>(&mut self, name: &str, n: usize) {
        self.bench_with_input(BenchmarkId::new(name, n), &n, |b, n| {
            b.iter_batched(
                || -> S { make_string(*n).into() },
                |s| s.clone(),
                BatchSize::SmallInput,
            )
        });
    }

    fn bench_into<S: From<String> + Into<String>>(&mut self, name: &str, n: usize) {
        self.bench_with_input(BenchmarkId::new(name, n), &n, |b, n| {
            b.iter_batched(
                || -> S { make_string(*n).into() },
                |s| -> String { s.into() },
                BatchSize::SmallInput,
            )
        });
    }
}

fn benchmark(c: &mut Criterion) {
    c.size_group("from", |g, n| {
        g.bench_from::<String>("string", n);
        g.bench_from::<SmolStr>("smol", n);
        g.bench_from::<SmartString<LazyCompact>>("smart", n);
    });
    c.size_group("clone", |g, n| {
        g.bench_clone::<String>("string", n);
        g.bench_clone::<SmolStr>("smol", n);
        g.bench_clone::<SmartString<LazyCompact>>("smart", n);
    });
    c.size_group("into", |g, n| {
        g.bench_into::<String>("string", n);
        g.bench_into::<SmolStr>("smol", n);
        g.bench_into::<SmartString<LazyCompact>>("smart", n);
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
